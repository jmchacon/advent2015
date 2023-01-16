//! day16 advent 2022
#![feature(iter_order_by)]

use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,
}

#[derive(Clone, Debug, Default)]
struct Sue {
    children: u64,
    cats: u64,
    samoyeds: u64,
    pomeranians: u64,
    akitas: u64,
    vizslas: u64,
    goldfish: u64,
    trees: u64,
    cars: u64,
    perfumes: u64,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut sues = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() == 8, "{} - bad line {line}", line_num + 1);
        assert!(
            sues.len() == usize::from_str_radix(parts[1].trim_end_matches(":"), 10).unwrap() - 1,
            "{} - bad sue {} {line}",
            sues.len(),
            line_num + 1
        );
        let mut sue = Sue::default();
        for (pos, m) in [parts[2], parts[4], parts[6]].iter().cloned().enumerate() {
            match m {
                "children:" => {
                    sue.children =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "cats:" => {
                    sue.cats =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "samoyeds:" => {
                    sue.samoyeds =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "pomeranians:" => {
                    sue.pomeranians =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "akitas:" => {
                    sue.akitas =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "vizslas:" => {
                    sue.vizslas =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "goldfish:" => {
                    sue.goldfish =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "trees:" => {
                    sue.trees =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "cars:" => {
                    sue.cars =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "perfumes:" => {
                    sue.perfumes =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                _ => panic!("{} - bad line {m} - {line}", line_num + 1),
            }
        }
        sues.push(sue);
    }

    //    for (pos, s) in sues.iter().enumerate() {
    //      println!("Sue {} -> {s:?}", pos + 1);
    // }
    let m = Sue {
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    };

    println!("best sue: {}", best_sue(&sues, &m, true));
    println!("best sue2: {}", best_sue(&sues, &m, false));
    Ok(())
}

fn best_sue(sues: &Vec<Sue>, m: &Sue, exact: bool) -> usize {
    // Turn these into arrays so we can iterate to compare vs field compare.
    let comp = [
        m.children,
        m.cats,
        m.samoyeds,
        m.pomeranians,
        m.akitas,
        m.vizslas,
        m.goldfish,
        m.trees,
        m.cars,
        m.perfumes,
    ];
    // Converts sues into arrays then enumerate each.
    // For each one it's a candidate if it passes our tests.
    // Tests may differ based on exact. If not exact cats/trees are > tests
    // and pomeranians/goldfish are < tests.
    // But because the tests are permissize and 0 == "ok" we may have N candidates
    // so map over those to fold down to a "score". Best score wins.
    sues.iter()
        .map(|s| {
            [
                s.children,
                s.cats,
                s.samoyeds,
                s.pomeranians,
                s.akitas,
                s.vizslas,
                s.goldfish,
                s.trees,
                s.cars,
                s.perfumes,
            ]
        })
        .enumerate()
        .filter(|(_, s)| {
            comp.iter().eq_by(s.iter().enumerate(), |x, y| {
                if exact || (y.0 != 1 && y.0 != 3 && y.0 != 6 && y.0 != 7) {
                    *x == *y.1 || *y.1 == 0
                } else {
                    if y.0 == 1 || y.0 == 7 {
                        *y.1 > *x || *y.1 == 0
                    } else {
                        *y.1 < *x || *y.1 == 0
                    }
                }
            })
        })
        .map(|(i, s)| {
            (
                comp.iter().enumerate().fold(0, |acc, x| {
                    // Don't score if we got here because it wasn't filled in.
                    if x.0 != 0 && s[x.0] == *x.1 {
                        acc + 1
                    } else {
                        acc
                    }
                }),
                i + 1,
            )
        })
        .max()
        .unwrap()
        .1
}
