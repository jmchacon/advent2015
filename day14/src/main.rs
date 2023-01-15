//! day14 advent 2022
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
                fly_speed: u64::from_str_radix(parts[3], 10).unwrap(),
                fly_time: u64::from_str_radix(parts[6], 10).unwrap(),
                rest: u64::from_str_radix(parts[13], 10).unwrap(),
            },
        );
    }

    let mut speeds = Vec::new();
    for (d, r) in &deer {
        let cycles = args.race / (r.fly_time + r.rest);
        let rem = args.race % (r.fly_time + r.rest);
        let mut dist = r.fly_speed * r.fly_time * cycles;
        if rem > r.fly_time {
            dist += r.fly_speed * r.fly_time;
        } else {
            dist += r.fly_speed * rem;
        }

        speeds.push((dist, *d));
    }
    speeds.sort();
    for s in &speeds {
        println!("{} flies {} in {} seconds", s.1, s.0, args.race);
    }
    Ok(())
}
