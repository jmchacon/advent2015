//! day6 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = 1000)]
    width: usize,

    #[arg(long, default_value_t = 1000)]
    height: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(usize, usize);

enum State {
    On,
    Off,
    Toggle,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut hs = HashSet::new();
    let mut hm = HashMap::new();

    for (line_num, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(
            parts.len() == 4 || parts.len() == 5,
            "{} - bad line {line}",
            line_num + 1
        );

        let mut state = State::Off;
        let (parse1, parse2);
        match (parts[0], parts[1], parts[2], parts[3]) {
            ("turn", "on" | "off", _, "through") => {
                assert!(parts.len() == 5, "{} - bad line {line}", line_num + 1);
                if parts[1] == "on" {
                    state = State::On;
                }
                parse1 = parts[2];
                parse2 = parts[4];
            }
            ("toggle", _, "through", _) => {
                assert!(parts.len() == 4, "{} - bad line {line}", line_num + 1);
                state = State::Toggle;
                parse1 = parts[1];
                parse2 = parts[3];
            }
            _ => {
                panic!("{} - bad line {line}", line_num + 1);
            }
        }
        let (x1, x2, y1, y2);
        let xs: Vec<&str> = parse1.split(',').collect();
        let ys: Vec<&str> = parse2.split(',').collect();
        assert!(
            xs.len() == 2 && ys.len() == 2,
            "{} - bad line {line}",
            line_num + 1
        );

        x1 = xs[0].parse::<usize>().unwrap();
        y1 = xs[1].parse::<usize>().unwrap();
        x2 = ys[0].parse::<usize>().unwrap();
        y2 = ys[1].parse::<usize>().unwrap();
        if args.debug {
            println!("{x1},{x2} -> {y1},{y2}");
        }
        for x in x1..=x2 {
            for y in y1..=y2 {
                match state {
                    State::On => {
                        hs.insert(Location(x, y));
                        hm.entry(Location(x, y))
                            .and_modify(|v| *v += 1)
                            .or_insert(1);
                    }
                    State::Off => {
                        hs.remove(&Location(x, y));
                        if hm.contains_key(&Location(x, y)) {
                            hm.entry(Location(x, y)).and_modify(|v| {
                                if *v > 0 {
                                    *v -= 1;
                                }
                            });

                            if *hm.get(&Location(x, y)).unwrap() == 0 {
                                hm.remove(&Location(x, y));
                            }
                        }
                    }
                    State::Toggle => {
                        if hs.contains(&Location(x, y)) {
                            hs.remove(&Location(x, y));
                        } else {
                            hs.insert(Location(x, y));
                        }
                        hm.entry(Location(x, y))
                            .and_modify(|v| *v += 2)
                            .or_insert(2);
                    }
                }
            }
        }
    }
    println!("part1: lit - {}", hs.len());
    println!("part2: brightness - {}", hm.values().sum::<usize>());
    Ok(())
}
