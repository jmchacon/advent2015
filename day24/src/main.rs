//! day24 advent 2015
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

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = 3)]
    buckets: u64,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut weights = Vec::new();
    for line in &lines {
        weights.push(line.parse::<u64>().unwrap());
    }
    let sum: u64 = weights.iter().sum();
    let bucket_size = sum / args.buckets;
    println!("Sum: {sum}");
    assert!(
        sum == bucket_size * args.buckets,
        "Bad list, not equalable divisable"
    );

    let mut candidates = Vec::new();
    for k in 2..=weights.len() - 2 {
        for c in weights
            .iter()
            .cloned()
            .combinations(k)
            .filter(|x| x.iter().sum::<u64>() == bucket_size) // Find all combos which make equal the bucket size
            .map(|x| (x.len(), x.iter().product::<u64>(), x))
        // Return length of the vec, quantum entanglement and the vec (technically don't need but debugging..)
        {
            candidates.push(c);
        }
    }
    // Sort it and the top one will be the one for compartment one with least QE
    candidates.sort();
    println!("Candidates: {}", candidates.len());
    println!("1st: {:?}", candidates[0]);
    if args.debug {
        for c in &candidates {
            println!("Candidate: {c:?}");
        }
    }
    Ok(())
}
