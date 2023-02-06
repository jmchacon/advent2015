//! day21 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use itertools::Itertools;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Clone, Debug, Default)]
struct Item<'a> {
    name: &'a str,
    cost: u64,
    damage: i64,
    armor: i64,
}

#[derive(Clone, Debug, Default)]
struct Stats {
    hp: i64,
    dmg: i64,
    armor: i64,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let weapons = weapons();
    let armor = armor();
    let rings = rings();
    let boss = Stats {
        hp: 100,
        dmg: 8,
        armor: 2,
    };

    if args.debug {
        // Test
        let test_player = Stats {
            hp: 8,
            dmg: 5,
            armor: 5,
        };
        let test_boss = Stats {
            hp: 12,
            dmg: 7,
            armor: 2,
        };
        println!(
            "Test fight for player: {test_player:?} and boss: {test_boss:?} - {}",
            do_fight(&test_player, &test_boss)
        );
    }

    // Now find the optimal player

    // Must have a weapon
    let mut best = u64::MAX;

    // Now bad shop keeper who steers you into the most expensive but losing combo.
    let mut worst = u64::MIN;
    for w in &weapons {
        // Armor optional so run with only ring options first

        // No ring
        best = cheapest_buy_and_fight(&vec![w], 100, &boss, best, args.debug);
        worst = expensive_buy_and_fight(&vec![w], 100, &boss, worst, args.debug);

        // 1 ring
        for r in &rings {
            best = cheapest_buy_and_fight(&vec![w, r], 100, &boss, best, args.debug);
            worst = expensive_buy_and_fight(&vec![w, r], 100, &boss, worst, args.debug);
        }

        // 2 rings
        for r in rings.iter().combinations(2) {
            best = cheapest_buy_and_fight(&vec![w, r[0], r[1]], 100, &boss, best, args.debug);
            worst = expensive_buy_and_fight(&vec![w, r[0], r[1]], 100, &boss, worst, args.debug);
        }

        // Now check for each armor and possible rings
        for a in &armor {
            // No ring
            best = cheapest_buy_and_fight(&vec![w, a], 100, &boss, best, args.debug);
            worst = expensive_buy_and_fight(&vec![w, a], 100, &boss, worst, args.debug);

            // 1 ring
            for r in &rings {
                best = cheapest_buy_and_fight(&vec![w, a, r], 100, &boss, best, args.debug);
                worst = expensive_buy_and_fight(&vec![w, a, r], 100, &boss, worst, args.debug);
            }

            // 2 rings
            for r in rings.iter().combinations(2) {
                best =
                    cheapest_buy_and_fight(&vec![w, a, r[0], r[1]], 100, &boss, best, args.debug);
                worst =
                    expensive_buy_and_fight(&vec![w, a, r[0], r[1]], 100, &boss, worst, args.debug);
            }
        }
    }
    println!("part1: Lowest cost is {best}");
    println!("part2: Worst cost is {worst}");

    Ok(())
}

fn rings() -> Vec<Item<'static>> {
    vec![
        Item {
            name: "dmg+1",
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            name: "dmg+2",
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            name: "dmg+3",
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            name: "dfs+1",
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            name: "dfs+2",
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            name: "dfs+3",
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ]
}

fn armor() -> Vec<Item<'static>> {
    vec![
        Item {
            name: "leather",
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            name: "chainmail",
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            name: "splintmail",
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            name: "bandedmail",
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            name: "platemail",
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ]
}

fn weapons() -> Vec<Item<'static>> {
    vec![
        Item {
            name: "dagger",
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            name: "shortsword",
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            name: "warhammer",
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            name: "longsword",
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            name: "greataxe",
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ]
}
fn buy(items: &Vec<&Item>, hp: i64, debug: bool) -> (Stats, u64) {
    let mut p = Stats {
        hp,
        dmg: 0,
        armor: 0,
    };

    let mut cost = 0;
    for i in items {
        if debug {
            println!("Adding item {}", i.name);
        }
        p.dmg += i.damage;
        p.armor += i.armor;
        cost += i.cost;
    }
    (p, cost)
}

fn expensive_buy_and_fight(
    items: &Vec<&Item>,
    hp: i64,
    boss: &Stats,
    worst: u64,
    debug: bool,
) -> u64 {
    let (p, cost) = buy(items, hp, debug);

    if !do_fight(&p, boss) && cost > worst {
        cost
    } else {
        worst
    }
}

fn cheapest_buy_and_fight(
    items: &Vec<&Item>,
    hp: i64,
    boss: &Stats,
    best: u64,
    debug: bool,
) -> u64 {
    let (p, cost) = buy(items, hp, debug);

    if do_fight(&p, boss) && cost < best {
        cost
    } else {
        best
    }
}

fn do_fight(p: &Stats, b: &Stats) -> bool {
    let mut player_hp = p.hp;
    let mut boss_hp = b.hp;

    let player_diff = p.dmg - b.armor;
    let boss_diff = b.dmg - p.armor;
    let player_dmg = if player_diff > 0 { player_diff } else { 1 };
    let boss_dmg = if boss_diff > 0 { boss_diff } else { 1 };
    loop {
        // Player first
        boss_hp -= player_dmg;
        if boss_hp <= 0 {
            return true;
        }

        // Now boss
        player_hp -= boss_dmg;
        if player_hp <= 0 {
            return false;
        }
    }
}
