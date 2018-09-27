use actix::Addr;
use core::{ClientAction, PlayerInfo, AttackType, TurnResultMsg};
use engine_io::EnginePipeIo;
use failure::Error;
use rand::{self, Rng};
use std::io;
use std::slice::Iter;
use std::str::FromStr;

/// Generates a new random integer between 0 and 100
#[cfg_attr(feature = "flame_it", flame)]
pub fn roll_dice() -> u8 {
    rand::thread_rng().gen_range(0, 100)
}

/// If the attack is missed the attack damage is 0
#[cfg_attr(feature = "flame_it", flame)]
pub fn rule_miss(_power: u64) -> u64 {
    0
}

/// If the attack is standard the attack damage is 1/3 the power
#[cfg_attr(feature = "flame_it", flame)]
pub fn rule_standard(power: u64) -> u64 {
    power / 3
}

/// If the attack is lucky the attack damage is 20% more than standard
#[cfg_attr(feature = "flame_it", flame)]
pub fn rule_lucky(power: u64) -> u64 {
    let std = rule_standard(power);
    std + std / 5
}

/// If the attack is critical the attack damage is two times the standard
#[cfg_attr(feature = "flame_it", flame)]
pub fn rule_critical(power: u64) -> u64 {
    2 * rule_standard(power)
}

// Somewhat generic and parametrized ranges defining which cluster
// to which rule one to one mapping
#[cfg_attr(feature = "flame_it", flame)]
pub fn get_cluster_reducer(range_params: Iter<'_, u8>, dice: u8) -> (AttackType, Box<Fn(u64) -> u64>) {
    let mut lower = 0u8;
    for (i, upper) in range_params.enumerate() {
        if dice >= lower && dice < *upper {
            match i {
                0 => return (AttackType::Miss,Box::new(rule_miss)),
                1 => return (AttackType::Standard,Box::new(rule_standard)),
                2 => return (AttackType::Lucky, Box::new(rule_lucky)),
                3 => return (AttackType::Critical, Box::new(rule_critical)),
                _ => return (AttackType::Undefined, Box::new(|_| 0)),
            }
        } else {
            lower = *upper;
        }
    }
    (AttackType::Undefined, Box::new(|_| 0))
}

#[cfg_attr(feature = "flame_it", flame)]
pub fn process_dmg(hp: &u64, dmg: &u64) -> u64 {
    if dmg > hp {
        0
    } else {
        hp - dmg
    }
}

#[cfg_attr(feature = "flame_it", flame)]
pub fn is_winner(hp: Iter<'_, u64>, turn: usize) -> bool {
    for (i, h) in hp.enumerate() {
        if i != turn && *h > 0 {
            return false;
        }
    }
    true
}

#[cfg_attr(feature = "flame_it", flame)]
pub fn battle_turn(
    range: &[u8],
    info: &[PlayerInfo],
    hp: &[u64],
    turn: usize,
    io: Addr<EnginePipeIo>,
) -> Result<TurnResultMsg, Error> {
    let mut p2 = if turn == info.len() - 1 { 0 } else { turn + 1 };
    while hp[p2] <= 0 { // for more than 2 players
        p2 = if p2 == info.len() - 1 { 0 } else { p2 + 1 };
    }
    let mut out_hp: Vec<u64> = hp.iter().cloned().collect();
    // Send message asynchronously to Game IO Actor mailbox (stdout)
    let _ = io.do_send(ClientAction::PlayerAction(
        info[turn].name.clone(),
        info[p2].name.clone(),
    ));
    let dice = roll_dice();
    let (attack, reducer) = get_cluster_reducer(range.iter(), dice);
    let dmg = &reducer(info[turn].power);
    let new_hp = process_dmg(&hp[p2], &dmg);
    match attack {
        AttackType::Undefined => (),
        _ => {
                // Send message asynchronously to Game IO Actor mailbox (stdout)
                let _ = io.do_send(ClientAction::AttackResult(attack, *dmg));
        }
    }
    out_hp[p2] = new_hp;
    while out_hp[p2] <= 0 { // for more than 2 players next_turn
        p2 = if p2 == info.len() - 1 { 0 } else { p2 + 1 };
    }    
    let winner = if is_winner(out_hp.iter(), turn) {
        Some((turn, out_hp[turn]))
    } else {
        None
    };
    Ok(TurnResultMsg {
        hp: out_hp,
        next_turn: p2,
        winner: winner,
    })
}

// TODO: refactor to come from Game IO Actor
pub fn get_player_info() -> Result<PlayerInfo, Error> {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input)?;
    Ok(PlayerInfo::from_str(input.as_ref())?)
}

#[cfg_attr(feature = "flame_it", flame)]
pub fn battle_warm_up(
    players: usize,
    current_players: usize,
    io: Addr<EnginePipeIo>,
) -> Result<bool, Error> {
    if current_players < players {
        // Send message asynchronously to Game IO Actor mailbox (stdout)
        let _ = io.do_send(ClientAction::AskPlayerInfo(current_players + 1));
        return Ok(false);
    } else {
        return Ok(true);
    }
}

#[cfg_attr(feature = "flame_it", flame)]
pub fn battle_announce(player_names: Vec<String>, io: Addr<EnginePipeIo>) -> Result<(), Error> {
    // Send message asynchronously to Game IO Actor mailbox (stdout)
    let _ = io.do_send(ClientAction::Start);
    let _ = io.do_send(ClientAction::AnnouncePlayers(player_names));
    Ok(())
}

#[cfg_attr(feature = "flame_it", flame)]
pub fn battle_over(player_name: String, hp: u64, io: Addr<EnginePipeIo>) -> Result<(), Error> {
    // Send message asynchronously to Game IO Actor mailbox (stdout)
    let _ = io.do_send(ClientAction::Winner(player_name,hp));
    Ok(())
}
