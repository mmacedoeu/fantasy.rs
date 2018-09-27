//! * Client
//! This is main client library.
//! It's responsability to initialize, setup, glue together
//! all modules and start processing
#![cfg_attr(feature="flame_it", feature(plugin, custom_attribute))]
#![cfg_attr(feature="flame_it", plugin(flamer))]

#[cfg(feature="flame_it")]
extern crate flame;
#[macro_use]
extern crate failure;
extern crate engine;
extern crate engine_io;
extern crate bpm;
#[macro_use]
extern crate clap;
extern crate actix;
extern crate tokio;
extern crate app_dir as dirs;
extern crate fconfig;
extern crate core;

use actix::prelude::*;
use dirs::Directories;
use engine::{GameActor, GameContext};
use engine_io::EnginePipeIo;
use bpm::Bpm;
use failure::Error;
use std::path::Path;
use core::StartBattleMsg;

const CONFIG_FILENAME: &str = "Settings.toml";

#[derive(Debug, Fail)]
/// Set of errors that can occurr during client processing
pub enum ClientError {
    #[fail(display = "OsString conversion error")]
    OsString,
    #[fail(display = "{}", _0)]
    Msg(String),
}

pub fn run<I, T>(args: I) -> Result<(), Error>
where
    I: IntoIterator<Item = T>,
    T: Into<::std::ffi::OsString> + Clone,
{
    // arguments parsing and processing
    #[cfg(feature="flame_init")]
    flame::start("args parsing");
    let yaml = load_yaml!("./cli.yml");
    let matches = clap::App::from_yaml(yaml)
        .version(crate_version!())
        .get_matches_from_safe(args)?;
    #[cfg(feature="flame_init")]
    flame::end("args parsing");

    // app data folder setup
    #[cfg(feature="flame_init")]
    flame::start("app data setup");    
    let base = matches.value_of("base");
    let mut d = Directories::default();
    if let Some(bpath) = base {
        d.base = String::from(bpath);
    }
    let _ = d.create_dirs();
    #[cfg(feature="flame_init")]
    flame::end("app data setup");    

    // configuration loading from file and environment vars
    #[cfg(feature="flame_init")]
    flame::start("config handling");  
    let mut app_config = fconfig::AppConfig::default();
    let config_param = matches.value_of("config");
    let mut config_file_name: String = dirs::get_base_file(d.base, CONFIG_FILENAME)
        .into_string()
        .map_err(|_| ClientError::OsString)?;
    if let Some(c) = config_param {
        // validates configuration file exists and is readable
        if Path::new(&c).is_file() {
            config_file_name = String::from(c);
        }
    }
    if let Ok(file_config) = fconfig::load_config(&config_file_name) {
        if file_config.players.is_some() {
            app_config.players = file_config.players;
        }
        if !file_config.range.is_empty() {
            app_config.range = file_config.range.iter().cloned().collect();
        }
    }
    #[cfg(feature="flame_init")]
    flame::end("config handling"); 

    // Number of players argument initialization
    #[cfg(feature="flame_init")]
    flame::start("players handling");     
    let players = matches.value_of("players");
    if let Some(p) = players {
        app_config.players = Some(p.parse()?);
    }
    #[cfg(feature="flame_init")]
    flame::end("players handling");     

    // Create new Reactor for Reative programming
    let sys = System::new("fantasy");

    // Starting actors 
    // Initialize the Stdin Stdout connector running in a thread-pool
    // with just one real thread
    #[cfg(feature="flame_init")]
    flame::start("connector setup");     
    let io_addr: Addr<EnginePipeIo> = SyncArbiter::start(1, move || {               
        EnginePipeIo{}
    });
    #[cfg(feature="flame_init")]
    flame::end("connector setup");     

    let io_addr_bpm = io_addr.clone();
    // Initialize BPM module running in a distinct thread-pool with
    // just one real thread
    #[cfg(feature="flame_init")]
    flame::start("bpm setup");    
    let bpm_addr: Addr<Bpm> = SyncArbiter::start(1, move || {               
        Bpm(io_addr_bpm.clone())
    });
    #[cfg(feature="flame_init")]
    flame::end("bpm setup");    

    // Game Actor Factory, it's async sharing one real thread sharing
    // as much as green / light thread's as there is memory avaliable on
    // hardware enabling it to sustain under stress millions of requests
    #[cfg(feature="flame_init")]
    flame::start("game setup");  
    let game_addr: Addr<GameActor> = GameActor::create(move |ctx| {
        // setup game message box capacity to 1000 to handle cyber security
        // DDOS attacks, after the capacity threshold of normal operation
        // any new requests is immediately dropped in order to not compromise
        // operation of the server and backfiring attacks
        ctx.set_mailbox_capacity(1000);
        GameActor(GameContext {
            config: app_config,
            io: io_addr,
            bpm: bpm_addr,
        })
    });
    #[cfg(feature="flame_init")]
    flame::end("game setup");    

    // Send message asynchronously to Game Actor to Start New Game
    let _ = game_addr.do_send(StartBattleMsg{});
    // Start main reactor and blocks main thread until a terminate message
    // comes to the System Actor message box
    let _ = sys.run();

    Ok(())
}

// Smoke test
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
