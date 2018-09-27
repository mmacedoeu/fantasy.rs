//! * Engine
//! This is the game engine library.
//! It's responsability to define game state
//! and handle state transition based on what is
//! returned from integration with bpm rules
//! It's a game state based on stream processing
//! of promises / futures using a declarative
//! [convention over configuration](https://en.wikipedia.org/wiki/Convention_over_configuration)
//! pattern as procedural approaches to implement
//! automaton proved to be hard to maintain and easy
//! to make coding mistakes 
#![cfg_attr(feature="flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature="flame_it", plugin(flamer))]

#[cfg(feature="flame_it")]
extern crate flame;
#[macro_use]
extern crate state_machine_future;
extern crate futures;
#[macro_use]
extern crate failure;
extern crate actix;
extern crate bpm;
extern crate fconfig;
extern crate core;
extern crate crossbeam_channel;
extern crate engine_io;
extern crate tokio;

pub mod integration;

use actix::{Actor, Addr, Arbiter, Context, Handler, System};
use bpm::Bpm;
use fconfig::AppConfig;
use core::{BattleWarmUpMsg, StartBattleMsg};
use engine_io::EnginePipeIo;
use futures::{Async, Future, Poll};
use state_machine_future::RentToOwn;
use std::io;

#[derive(Debug, Fail)]
/// Set of errors that can occurr during engine processing
pub enum EngineError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    Msg(String),
}

#[derive(Clone)]
pub struct GameContext {
    pub config: AppConfig,
    pub bpm: Addr<Bpm>,
    pub io: Addr<EnginePipeIo>,
}

/// To represent game state we are going to use
/// [DFA](https://en.wikipedia.org/wiki/Deterministic_finite_automaton)
/// Some kind of simple turn based game.
///
/// ```text
/// +----------------+
/// |     Start      |
/// +----------------+
///   |
///   |                        +-------------------------------+
///   v                        v                               |
/// +-----------------------------------------+                |
/// |                 WarmUp                  | ------+        |
/// +-----------------------------------------+       |        |
///   |                        |                      |        |
///   |                        |                      |        |
///   v                        v                      v        |
/// +----------------+       +----------------+     +-------+  |
/// | BattleAnnounce | -+    | WaitPlayerInfo | --> | Error |  |
/// +----------------+  |    +----------------+     +-------+  |
///   |                 |      |                      ^        |
///   |                 +------+----------------------+        |
///   v                        |                               |
/// +----------------+         |                               |
/// |                | ---+    |                               |
/// |    BattleOn    |    |    |                               |
/// |                | <--+    +-------------------------------+
/// +----------------+
///   |
///   |
///   v
/// +----------------+
/// |   BattleOver   |
/// +----------------+
///   |
///   |
///   v
/// +----------------+
/// |     Finish     |
/// +----------------+
/// ```
/// As a convention each state is defined as an enumeration option
/// and transition is defined by a promise / future which
/// call the BPM module where rules are applied and the response
/// is used to return the next state.
/// It's also Actor based using [green / light threads] https://en.wikipedia.org/wiki/Green_threads
/// and [Reactive] https://en.wikipedia.org/wiki/Reactive_programming 
/// enabling async low footprint scaling to millions of operations concurrently
/// on a single core
#[derive(StateMachineFuture)]
pub enum Game where 
{
    /// The game begins with GameContext as argument.
    /// This annotation is a declarative convention to define the state name
    /// and what transition's are enabled which will act close to the compiler
    /// static validation's to reinforce quality and fast error feedback while
    /// you are typing warning hints will actively show any mistakes
    #[state_machine_future(
        start,
        transitions(WaitPlayerInfo, BattleAnnounce, WarmUp, Error)
    )]
    Start(GameContext),

    /// Wait for input with player info until number of players.
    #[state_machine_future(transitions(WarmUp, Error))]
    WaitPlayerInfo {
        // every game state, despite Start, Finish and Error
        // has per convention a handler which is a promise / future
        // to handle transition logic in a coesive manner
        handler: Box<Future<Item = AfterWaitPlayerInfo, Error = failure::Error>>,
    },

    /// Battle warm up ! Gather players info.
    #[state_machine_future(transitions(WaitPlayerInfo, BattleAnnounce, Error))]
    WarmUp {
        handler: Box<Future<Item = AfterWarmUp, Error = failure::Error>>,
    },

    /// Announce Battle start.
    #[state_machine_future(transitions(BattleOn, Error))]
    BattleAnnounce {
        handler: Box<Future<Item = AfterBattleAnnounce, Error = failure::Error>>,
    },

    /// Battle started
    #[state_machine_future(transitions(BattleOver, BattleOn, Error))]
    BattleOn {
        handler: Box<Future<Item = AfterBattleOn, Error = failure::Error>>,
    },

    /// Battle is over, restart ?
    #[state_machine_future(transitions(Start, Finished))]
    BattleOver {
        handler: Box<Future<Item = AfterBattleOver, Error = failure::Error>>,
    },

    /// The game is finished with a `()` empty.
    /// The `()` becomes the `Future::Item`.
    #[state_machine_future(ready)]
    Finished(()),

    // Any state transition can implicitly go to this error state
    // This `failure::Error` is used as the `Future::Error`.
    #[state_machine_future(error)]
    Error(failure::Error),
}

/// Now, we implement the generated state transition polling trait for our state
/// machine description type.
impl PollGame for Game { 
    // this is where further initialization is done, mainly the first
    // promise / future
    #[cfg_attr(feature = "flame_it", flame)]
    fn poll_start<'a>(data: &'a mut RentToOwn<'a, Start>) -> Poll<AfterStart, failure::Error> {
        let context = data.take().0;
        let players = context.config.players.unwrap();
        let handler = integration::bpm_battle_warm_up_future(BattleWarmUpMsg { players, current_players: 0}, context, Vec::new(), );
        let player_info = WarmUp {
            handler,
        };
        Ok(Async::Ready(AfterStart::WarmUp(player_info)))
    }

    // Every other state just forward execution to it's handler
    #[cfg_attr(feature = "flame_it", flame)]
    fn poll_wait_player_info<'a>(
        data: &'a mut RentToOwn<'a, WaitPlayerInfo>,
    ) -> Poll<AfterWaitPlayerInfo, failure::Error> {
        data.handler.poll()
    }

    #[cfg_attr(feature = "flame_it", flame)]
    fn poll_warm_up<'a>(
        data: &'a mut RentToOwn<'a, WarmUp>,
    ) -> Poll<AfterWarmUp, failure::Error> {
        data.handler.poll()
    }

    #[cfg_attr(feature = "flame_it", flame)]
    fn poll_battle_announce<'a>(
        data: &'a mut RentToOwn<'a, BattleAnnounce>,
    ) -> Poll<AfterBattleAnnounce, failure::Error> {
        data.handler.poll()
    }

    #[cfg_attr(feature = "flame_it", flame)]
    fn poll_battle_on<'a>(
        data: &'a mut RentToOwn<'a, BattleOn>,
    ) -> Poll<AfterBattleOn, failure::Error> {
        data.handler.poll()
    }

    #[cfg_attr(feature = "flame_it", flame)]
    fn poll_battle_over<'a>(
        data: &'a mut RentToOwn<'a, BattleOver>,
    ) -> Poll<AfterBattleOver, failure::Error> {
        data.handler.poll()
    }
}

// The Game actor with inner Game context configuration
pub struct GameActor(pub GameContext);

/// Turn GameActor into Actor enabled
impl Actor for GameActor {
    type Context = Context<Self>;
}

/// Message handling for type StartBattle
impl Handler<StartBattleMsg> for GameActor {
    type Result = Result<(), failure::Error>;

    #[cfg_attr(feature = "flame_it", flame)]
    fn handle(&mut self, _msg: StartBattleMsg, _ctx: &mut Self::Context) -> Self::Result {
        // Handle the Start game by initializing the Game stream, when stream ends sends
        // message to to System Actor to terminate main reactor running on main thread
        // which will terminate program gracefully
        // spawn will launch a new green / light thread to process the game stream
        // spawn is async and returns immediately
        Arbiter::spawn(Game::start(self.0.clone()).and_then(|_| Ok(System::current().stop())).map_err(|_| ()).map(|_| ()));
        Ok(())
    }
}
