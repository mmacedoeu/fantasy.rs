//! * Core
//! This is the core of the solution.
//! It holds everything common to all other modules
//! like basic business data structures
//! It's roadmap include detach language related content and provide
//! internacionalization also provide compatibility with the portable
//! WASM [target](https://webassembly.org/) so it can be reused across
//! several languages

#[macro_use]
extern crate failure;
extern crate actix;
#[macro_use]
extern crate serde_derive;

use actix::Message;
use failure::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AttackType {
    Miss,
    Standard,
    Lucky,
    Critical,
    Undefined
}

// language specific display, should i18n this on the future
impl fmt::Display for AttackType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            AttackType::Miss => String::from("Errou !"),
            AttackType::Standard => String::from("Normal"),
            AttackType::Lucky => String::from("Sorte!!!"),            
            AttackType::Critical => String::from("Crítico!"),
            AttackType::Undefined => String::from(""),
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientAction {
    AskPlayerInfo(usize),
    Start,
    AnnouncePlayers(Vec<String>),
    PlayerAction(String, String),
    AttackResult(AttackType, u64),
    Winner(String, u64),
    Message(String),
}

/// Turn ClientAction messaging enabled
impl Message for ClientAction {
    type Result = Result<(), Error>;
}

// language specific display, should i18n this on the future
impl fmt::Display for ClientAction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ClientAction::AskPlayerInfo(ref n) => format!("Entre o personagem {}", n),
            ClientAction::Start => String::from("O jogo começou"),
            ClientAction::AnnouncePlayers(ref p) => format!("Batalha entre {:?}", p),
            ClientAction::PlayerAction(ref p1, ref p2) => format!("{} atacou {}", p1, p2),
            ClientAction::AttackResult(ref t, ref d) => format!("{} - {} HP", t, d),
            ClientAction::Winner(ref p, ref hp) => format!(
                "Jogo acabou, o vencedor foi {} com HP restante de {}",
                p, hp
            ),
            ClientAction::Message(ref m) => m.clone(),
        };
        write!(f, "{}", printable)
    }
}

#[derive(Debug, Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub energy: u64,
    pub power: u64,
}

impl FromStr for PlayerInfo {
    type Err = Error;

    /// Conversion from str with error handling
    fn from_str(data: &str) -> Result<PlayerInfo, Error> {
        let info_vec: Vec<&str> = data.split_whitespace().collect();
        if info_vec.len() != 3 {
            // language specific display, should i18n this on the future
            return Err(format_err!("Player info insuficient params"));
        }
        let name = String::from(info_vec[0]);
        let energy = info_vec[1].parse::<u64>()?;
        let power = info_vec[2].parse::<u64>()?;
        Ok(PlayerInfo {
            name: name,
            energy: energy,
            power: power,
        })
    }
}

impl<'a> From<&'a str> for PlayerInfo {
    /// Conversion from String to PlayerInfo will
    /// crash if input is not on format "name energy power"
    fn from(data: &'a str) -> Self {
        let info_vec: Vec<&str> = data.split_whitespace().collect();
        let name = String::from(info_vec[0]);
        let energy = info_vec[1].parse::<u64>().unwrap();
        let power = info_vec[2].parse::<u64>().unwrap();
        Self {
            name: name,
            energy: energy,
            power: power,
        }
    }
}

pub struct GetPlayerInfoMsg;

/// Turn GetPlayerInfo messaging enabled
impl Message for GetPlayerInfoMsg {
    type Result = Result<PlayerInfo, Error>;
}

pub struct StartBattleMsg;
/// Turn GetPlayerInfo messaging enabled
impl Message for StartBattleMsg {
    type Result = Result<(), Error>;
}

pub struct BattleTurnMsg {
    pub range: Vec<u8>,
    pub info: Vec<PlayerInfo>,
    pub hp: Vec<u64>,
    pub turn: usize,    
}

/// BattleTurn messaging enabled
impl Message for BattleTurnMsg {
    type Result = Result<TurnResultMsg, Error>;
}

pub struct TurnResultMsg {
    pub hp: Vec<u64>,    
    pub next_turn: usize,
    pub winner: Option<(usize, u64)>,
}

pub struct BattleWarmUpMsg {
    pub players: usize,
    pub current_players: usize,
}

/// BattleTurn messaging enabled
impl Message for BattleWarmUpMsg {
    type Result = Result<bool, Error>;
}

pub struct BattleAnnounceMsg(pub Vec<String>);

/// BattleAnnounce messaging enabled
impl Message for BattleAnnounceMsg {
    type Result = Result<(), Error>;
}

pub struct WinnerMsg(pub String, pub u64);

/// WinnerMsg messaging enabled
impl Message for WinnerMsg {
    type Result = Result<(), Error>;
}
