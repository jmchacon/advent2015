//! day17 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use itertools::Itertools;
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

    #[arg(long, default_value_t = 150)]
    fill: u64,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let buckets = lines
        .iter()
        .map(|l| l.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut hm = HashMap::new();
    (2..=buckets.len()).for_each(|x| {
        let c = buckets
            .iter()
            .combinations(x)
            .filter(|x| x.iter().copied().sum::<u64>() == args.fill)
            .count();
        if c > 0 {
            hm.entry(x).and_modify(|x| *x += c).or_insert(c);
        }
    });
    let sum = hm.values().sum::<usize>();
    println!(
        "part1: {sum} combinations for {} buckets to fill to {}",
        buckets.len(),
        args.fill
    );
    let min = hm.iter().min().unwrap();
    println!("part2: {} combos where min is {}", min.1, min.0);
    Ok(())
}
