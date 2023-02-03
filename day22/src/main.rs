//! day22 advent 2022
use crate::Effect::*;
use clap::Parser;
use color_eyre::eyre::Result;
use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
};
use strum_macros::Display;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Clone, Debug, Display, Hash, PartialEq, Eq)]
enum Effect {
    Shield,
    Poison,
    Recharge,
}

#[derive(Clone, Debug)]
struct Spell<'a> {
    name: &'a str,
    cost: i64,
    damage: i64,
    heal: i64,
    effect: Option<Effect>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Stats {
    hp: i64,
    mana: i64,
    dmg: i64,
    armor: i64,
    effects: HashMap<Effect, u64>,
}

impl Ord for Stats {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hp.cmp(&other.hp)
    }
}

impl PartialOrd for Stats {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let spells = vec![
        Spell {
            name: "Magic Missle",
            cost: 53,
            damage: 4,
            heal: 0,
            effect: None,
        },
        Spell {
            name: "Drain",
            cost: 73,
            damage: 2,
            heal: 2,
            effect: None,
        },
        Spell {
            name: "Shield",
            cost: 113,
            damage: 0,
            heal: 0,
            effect: Some(Shield),
        },
        Spell {
            name: "Poison",
            cost: 173,
            damage: 0,
            heal: 0,
            effect: Some(Poison),
        },
        Spell {
            name: "Recharge",
            cost: 229,
            damage: 0,
            heal: 0,
            effect: Some(Recharge),
        },
    ];

    let player = Stats {
        hp: 50,
        mana: 500,
        dmg: 0,
        armor: 0,
        effects: HashMap::new(),
    };
    let boss = Stats {
        hp: 51,
        mana: 0,
        dmg: 9,
        armor: 0,
        effects: HashMap::new(),
    };

    let total = run_battle(&player, &boss, &spells, false, args.debug);
    println!("Total {total}");
    let hard_total = run_battle(&player, &boss, &spells, true, args.debug);
    println!("Hard total {hard_total}");
    Ok(())
}

fn run_battle(player: &Stats, boss: &Stats, spells: &Vec<Spell>, hard: bool, debug: bool) -> i64 {
    let mut q = BinaryHeap::new();

    q.push(Reverse((0, player.clone(), boss.clone())));

    let mut total = 0;
    'outer: while let Some(e) = q.pop() {
        let spent = e.0 .0;
        let mut player = e.0 .1;
        let mut boss = e.0 .2;

        // player turn

        // Hard mode
        if hard {
            player.hp -= 1;
            if player.hp <= 0 {
                // Died so nothing else to do.
                continue;
            }
        }

        // Run current effects and then reset the hash for spells
        let mut neweffect = HashMap::new();
        for (k, v) in &player.effects {
            let dur = v - 1;
            // NOTE: Spells always take effect..Then we check duration
            match k {
                Shield => {
                    if dur == 0 {
                        player.armor -= 7;
                    }
                }
                Poison => {
                    boss.hp -= 3;
                }
                Recharge => {
                    player.mana += 101;
                }
            }
            if dur > 0 {
                neweffect.insert(k.clone(), dur);
            }
        }
        player.effects = neweffect;

        // If the boss died we're good.
        if boss.hp <= 0 {
            total = spent;
            break;
        }

        // Otherwise we try each spell and push onto the queue if it can be cast.
        for s in spells {
            if player.mana - s.cost > 0 {
                let mut new = player.clone();
                new.mana -= s.cost;
                let newspent = spent + s.cost;
                let mut newboss = boss.clone();
                if s.damage > 0 {
                    newboss.hp -= s.damage;
                    if newboss.hp <= 0 {
                        total = newspent;
                        break 'outer;
                    }
                }
                if s.heal > 0 {
                    new.hp += s.heal;
                }
                if let Some(eff) = &s.effect {
                    if !new.effects.contains_key(eff) {
                        match eff {
                            Shield => {
                                new.armor += 7;
                                new.effects.insert(Shield, 6);
                            }
                            Poison => {
                                new.effects.insert(Poison, 6);
                            }
                            Recharge => {
                                new.effects.insert(Recharge, 5);
                            }
                        }
                    }
                }

                // At this point the player is done so do the boss

                // Run effects again
                let mut neweffect = HashMap::new();
                for (k, v) in &new.effects {
                    // NOTE: Spells always take effect..Then we check duration
                    let dur = v - 1;
                    match k {
                        Shield => {
                            if dur == 0 {
                                new.armor -= 7;
                            }
                        }
                        Poison => {
                            newboss.hp -= 3;
                        }
                        Recharge => {
                            new.mana += 101;
                        }
                    }
                    if dur > 0 {
                        neweffect.insert(k.clone(), dur);
                    }
                }
                new.effects = neweffect;

                // If the boss died we're good.
                if newboss.hp <= 0 {
                    total = newspent;
                    break 'outer;
                }

                // Boss always does at least one damage
                let mut dmg = newboss.dmg - new.armor;
                if dmg < 1 {
                    dmg = 1;
                }
                new.hp -= dmg;
                if new.hp <= 0 {
                    // Player died so bad path so skip this one
                    continue;
                }

                // Otherwise push it on to try getting to boss death
                if debug {
                    println!("Casting {}", s.name);
                }
                q.push(Reverse((newspent, new, newboss)));
            }
        }
    }
    total
}
