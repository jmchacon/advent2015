//! day14 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashMap;
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

    #[arg(long, default_value_t = 2503)]
    race: u64,
}

#[derive(Clone, Debug)]
struct Reindeer {
    fly_speed: u64,
    fly_time: u64,
    rest: u64,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut deer = HashMap::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() == 15, "{} - bad line {line}", line_num + 1);
        deer.insert(
            parts[0],
            Reindeer {
                fly_speed: parts[3].parse::<u64>()?,
                fly_time: parts[6].parse::<u64>()?,
                rest: parts[13].parse::<u64>()?,
            },
        );
    }

    let mut speeds = Vec::new();
    for (d, r) in &deer {
        speeds.push((dist_time(r, args.race), *d));
    }
    speeds.sort_unstable();
    if args.debug {
        for s in &speeds {
            println!("{} flies {} in {} seconds", s.1, s.0, args.race);
        }

        println!();
    }
    let winner = speeds.last().unwrap();
    println!("part1: Winner is {} who flew {}", winner.1, winner.0);
    let mut scores = HashMap::new();
    for time in 1..=args.race {
        let mut speeds = Vec::new();
        for (d, r) in &deer {
            speeds.push((dist_time(r, time), *d));
        }
        speeds.sort_unstable();
        let (_, winner) = speeds[speeds.len() - 1];
        scores.entry(winner).and_modify(|x| *x += 1).or_insert(1);
    }
    // Now flip them so we can sort it.
    let mut s = scores.iter().map(|(k, v)| (*v, *k)).collect::<Vec<_>>();
    s.sort_unstable();

    if args.debug {
        for (s, r) in &s {
            println!("{r} has {s} score in {} seconds", args.race);
        }
    }
    let winner = s.last().unwrap();
    println!("part2: Winner is {} who flex {}", winner.1, winner.0);
    Ok(())
}

fn dist_time(r: &Reindeer, time: u64) -> u64 {
    let cycles = time / (r.fly_time + r.rest);
    let rem = time % (r.fly_time + r.rest);
    let mut dist = r.fly_speed * r.fly_time * cycles;
    if rem > r.fly_time {
        dist += r.fly_speed * r.fly_time;
    } else {
        dist += r.fly_speed * rem;
    }
    dist
}
