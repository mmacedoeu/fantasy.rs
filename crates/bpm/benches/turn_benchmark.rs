#[macro_use]
extern crate criterion;
extern crate bpm;
extern crate core;

use bpm::rules;
use core::{PlayerInfo, TurnResultMsg};
use criterion::Criterion;

fn battle_turn_bench(c: &mut Criterion) {
    c.bench_function("turn", |b| {
        b.iter(|| {
            let range = vec![15, 70, 96];
            let info = vec![
                PlayerInfo {
                    name: String::from("A"),
                    energy: 100,
                    power: 100,
                },
                PlayerInfo {
                    name: String::from("B"),
                    energy: 100,
                    power: 100,
                },
            ];
            let hp = vec![100, 100];
            let turn = 0;
            {
                let mut p2 = if turn == info.len() - 1 { 0 } else { turn + 1 };
                while hp[p2] <= 0 {
                    // for more than 2 players
                    p2 = if p2 == info.len() - 1 { 0 } else { p2 + 1 };
                }
                let mut out_hp: Vec<u64> = hp.iter().cloned().collect();
                let dice = rules::roll_dice();
                let (_attack, reducer) = rules::get_cluster_reducer(range.iter(), dice);
                let dmg = &reducer(info[turn].power);
                let new_hp = rules::process_dmg(&hp[p2], &dmg);
                out_hp[p2] = new_hp;
                while out_hp[p2] <= 0 {
                    // for more than 2 players next_turn
                    p2 = if p2 == info.len() - 1 { 0 } else { p2 + 1 };
                }
                let winner = if rules::is_winner(out_hp.iter(), turn) {
                    Some((turn, out_hp[turn]))
                } else {
                    None
                };
                let _out = TurnResultMsg {
                    hp: out_hp,
                    next_turn: p2,
                    winner: winner,
                };
            }
        })
    });
}

criterion_group!(benches, battle_turn_bench);
criterion_main!(benches);
