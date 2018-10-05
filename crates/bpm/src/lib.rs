//! * BPM
//! This is main Business Process Management library.
//! It's responsability is to define the Business Processes,
//! and rules. For phase 0 of MVP it's just hardcoded game rules!
//! When the system grow in load bpm could detach from this process and
//! form a cluster and this module will just be a bpm client making
//! further calls
//! Roadmap includes making a full blown BPM running processing compiled
//! from any language to a portable WASM (Web Assembly) format
//! BPM is sandboxed and it's only view to external world is externalities
//! passed with BPM Actor inner initialization Engine IO message Box address
//! In order to collect metrics like latency on runtime another Actor could
//! be created to further export metrics to Analytics and monitoring platforms
//! like Graphene and others.
#![cfg_attr(feature = "flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature = "flame_it", plugin(flamer))]

#[cfg(feature = "flame_it")]
extern crate flame;
#[macro_use]
extern crate failure;
extern crate actix;
extern crate core;
extern crate engine_io;
extern crate rand;
extern crate crossbeam_channel as channel;

pub mod rules;

use actix::{Actor, Addr, Handler, SyncContext};
use core::{
    BattleAnnounceMsg, BattleTurnMsg, BattleWarmUpMsg, GetPlayerInfoMsg, PlayerInfo, TurnResultMsg,
    WinnerMsg
};
use engine_io::EnginePipeIo;

pub struct Bpm(pub Addr<EnginePipeIo>, pub channel::Receiver<PlayerInfo>);

/// Turn EnginePipeIo into Actor enabled
impl Actor for Bpm {
    type Context = SyncContext<Self>;
}

/// Message handling for type GetPlayerInfoMsg
impl Handler<GetPlayerInfoMsg> for Bpm {
    type Result = Result<PlayerInfo, failure::Error>;

    #[cfg_attr(feature = "flame_it", flame)]
    fn handle(&mut self, _msg: GetPlayerInfoMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.1.recv().ok_or(format_err!("channel closed"))
    }
}

/// Message handling for type BattleWarmUpMsg
impl Handler<BattleWarmUpMsg> for Bpm {
    type Result = Result<bool, failure::Error>;

    #[cfg_attr(feature = "flame_it", flame)]
    fn handle(&mut self, msg: BattleWarmUpMsg, _ctx: &mut Self::Context) -> Self::Result {
        rules::battle_warm_up(msg.players, msg.current_players, self.0.clone())
    }
}

/// Message handling for type BattleAnnounceMsg
impl Handler<BattleAnnounceMsg> for Bpm {
    type Result = Result<(), failure::Error>;

    #[cfg_attr(feature = "flame_it", flame)]
    fn handle(&mut self, msg: BattleAnnounceMsg, _ctx: &mut Self::Context) -> Self::Result {
        rules::battle_announce(msg.0, self.0.clone())
    }
}

/// Message handling for type BattleTurnMsg
impl Handler<BattleTurnMsg> for Bpm {
    type Result = Result<TurnResultMsg, failure::Error>;

    #[cfg_attr(feature = "flame_it", flame)]
    fn handle(&mut self, msg: BattleTurnMsg, _ctx: &mut Self::Context) -> Self::Result {
        rules::battle_turn(&msg.range, &msg.info, &msg.hp, msg.turn, self.0.clone())
    }
}

/// Message handling for type WinnerMsg
impl Handler<WinnerMsg> for Bpm {
    type Result = Result<(), failure::Error>;

    #[cfg_attr(feature = "flame_it", flame)]
    fn handle(&mut self, msg: WinnerMsg, _ctx: &mut Self::Context) -> Self::Result {
        rules::battle_over(msg.0, msg.1, self.0.clone())
    }
}

#[cfg(test)]
mod tests {
    use rules;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn regular_rule_standard_test() {
        let dmg = rules::rule_standard(100);
        assert_eq!(dmg, 33);
    }
    #[test]
    fn regular_rule_lucky_test() {
        let dmg = rules::rule_lucky(120);
        assert_eq!(dmg, 48);
    }
    #[test]
    fn regular_rule_critical_test() {
        let dmg = rules::rule_critical(120);
        assert_eq!(dmg, 80);
    }
}
