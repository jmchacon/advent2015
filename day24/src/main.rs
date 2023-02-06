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

    for b in [(1, 3_u64), (2, 4_u64)] {
        let bucket_size = sum / b.1;
        if args.debug {
            println!("Sum: {sum}");
        }
        assert!(
            sum == bucket_size * b.1,
            "Bad list, not equalable divisable"
        );

        let mut candidates = Vec::new();
        for k in 2..=weights.len() - 2 {
            for c in weights
                .iter()
                .copied()
                .combinations(k)
                .filter(|x| x.iter().sum::<u64>() == bucket_size) // Find all combos which make equal the bucket size
                .map(|x| {
                    if args.debug {
                        (x.len(), x.iter().product::<u64>(), x)
                    } else {
                        (x.len(), x.iter().product::<u64>(), vec![])
                    }
                })
            // Return length of the vec, quantum entanglement and the vec (technically don't need but debugging..)
            {
                candidates.push(c);
            }
        }
        // Sort it and the top one will be the one for compartment one with least QE
        candidates.sort();
        if args.debug {
            println!("Candidates: {}", candidates.len());
        }
        println!("part{}: 1st: {:?}", b.0, candidates.first().unwrap());
        if args.debug {
            for c in &candidates {
                println!("Candidate: {c:?}");
            }
        }
    }
    Ok(())
}
