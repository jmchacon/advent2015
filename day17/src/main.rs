//! day17 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use itertools::Itertools;
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
        .map(|l| u64::from_str_radix(l, 10).unwrap())
        .collect::<Vec<_>>();

    let sum = (2..=buckets.len())
        .map(|x| {
            buckets
                .iter()
                .combinations(x)
                .filter(|x| x.iter().cloned().sum::<u64>() == args.fill)
                .count()
        })
        .sum::<usize>();
    println!(
        "{sum} combinations for {} buckets to fill to {}",
        buckets.len(),
        args.fill
    );
    Ok(())
}
