use super::GameContext;
use core::{
    BattleAnnounceMsg, BattleTurnMsg, BattleWarmUpMsg, GetPlayerInfoMsg, PlayerInfo, WinnerMsg,
};
use failure::Error;
use futures::Future;

// promise / future generation
// it gets the BPM mailbox Addr and use the send method to deliver
// messaging with process parameters in the form of Messaging
// struct terminating with ...Msg
// After flattening the response is used to decide transition
// to next state with next state own handler initialized
pub fn bpm_get_player_info_future(
    msg: GetPlayerInfoMsg,
    context: GameContext,
    mut info: Vec<PlayerInfo>,
) -> Box<Future<Item = super::AfterWaitPlayerInfo, Error = Error>> {
    Box::new(
        context
            .bpm // bpm message box address
            .send(msg) // sending message promise / future
            .map_err(Into::into) // error conversion
            .and_then(|r| r) // flattening
            .and_then(move |p| { // response from bpm
                // include new Player Info on list
                let _ = info.push(p);
                // get players number on App configuration
                let players = context.config.players.unwrap();
                // get current number of players
                let current_players = info.len();
                // initialize next state with it's handler promise
                let warm_up = super::WarmUp {
                    handler: bpm_battle_warm_up_future(
                        BattleWarmUpMsg {
                            players,
                            current_players,
                        },
                        context,
                        info,
                    ),
                };
                // return new game state
                Ok(super::AfterWaitPlayerInfo::WarmUp(warm_up))
            }),
    )
}

pub fn bpm_battle_warm_up_future(
    msg: BattleWarmUpMsg,
    context: GameContext,
    info: Vec<PlayerInfo>,
) -> Box<Future<Item = super::AfterWarmUp, Error = Error>> {
    Box::new(
        context
            .bpm
            .send(msg)
            .map_err(Into::into)
            .and_then(|r| r)
            .and_then(move |b| {
                if !b {
                    // players < number of players
                    // and bpm successfully returned
                    let player_info = super::WaitPlayerInfo {
                        handler: bpm_get_player_info_future(
                            GetPlayerInfoMsg {},
                            context,
                            info,
                        ),
                    };
                    Ok(super::AfterWarmUp::WaitPlayerInfo(player_info))
                } else {
                    // players == number of players
                    // and bpm successfully returned
                    // use of functional map reduce to initialize players HP
                    let hp: Vec<u64> = info.iter().map(|i| i.energy).collect();
                    // use of functional map reduce to initialize players names
                    let player_names = info.iter().map(|i| i.name.clone()).collect();
                    let battle_data = super::BattleAnnounce {
                        handler: bpm_battle_announce_future(
                            BattleAnnounceMsg(player_names),
                            context,
                            info,
                            hp,
                            0, // next turn
                        ),
                    };
                    Ok(super::AfterWarmUp::BattleAnnounce(battle_data))
                }
            }),
    )
}

pub fn bpm_battle_announce_future(
    msg: BattleAnnounceMsg,
    context: GameContext,
    info: Vec<PlayerInfo>,
    hp: Vec<u64>,
    turn: usize,
) -> Box<Future<Item = super::AfterBattleAnnounce, Error = Error>> {
    Box::new(
        context
            .bpm
            .send(msg)
            .map_err(Into::into)
            .and_then(|r| r)
            .and_then(move |_p| {
                // get dice cluster of ranges from App configuration
                let range = context.config.range.clone();
                let warm_up = super::BattleOn {
                    handler: bpm_battle_turn_future(
                        BattleTurnMsg {
                            range,
                            info: info.clone(),
                            hp,
                            turn,
                        },
                        context,
                        info,
                    ),
                };
                Ok(super::AfterBattleAnnounce::BattleOn(warm_up))
            }),
    )
}

pub fn bpm_battle_turn_future(
    msg: BattleTurnMsg,
    context: GameContext,
    info: Vec<PlayerInfo>,
) -> Box<Future<Item = super::AfterBattleOn, Error = Error>> {
    Box::new(
        context
            .bpm
            .send(msg)
            .map_err(Into::into)
            .and_then(|r| r)
            .and_then(move |b| {
                if let Some((w, hp)) = b.winner {
                    // There is a winner
                    // get player name from Player Info list
                    let player = info[w].name.clone();
                    let over = super::BattleOver {
                        handler: bpm_battle_over_future(
                            WinnerMsg(player, hp),
                            context,
                        ),
                    };
                    Ok(super::AfterBattleOn::BattleOver(over))
                } else {
                    // No winner, call next turn
                    let range = context.config.range.clone();
                    let battle_on = super::BattleOn {
                        handler: bpm_battle_turn_future(
                            BattleTurnMsg {
                                range,
                                info: info.clone(),
                                hp: b.hp.clone(),
                                turn: b.next_turn.clone(),
                            },
                            context,
                            info,
                        ),
                    };
                    Ok(super::AfterBattleOn::BattleOn(battle_on))
                }
            }),
    )
}

pub fn bpm_battle_over_future(
    msg: WinnerMsg,
    context: GameContext,
) -> Box<Future<Item = super::AfterBattleOver, Error = Error>> {
    Box::new(
        context
            .bpm
            .send(msg)
            .map_err(Into::into)
            .and_then(|r| r)
            .and_then(move |_p| {
                // Game finished, this will end the stream processing
                Ok(super::AfterBattleOver::Finished(super::Finished(())))
            }),
    )
}
