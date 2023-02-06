//! day3 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashSet;
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(i32, i32);

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    for (line_num, line) in lines.iter().enumerate() {
        println!("part1: Visited {} houses", deliveries(1, line, line_num));
        println!("part2: Visited {} houses", deliveries(2, line, line_num));
    }

    Ok(())
}

fn deliveries(deliverers: usize, line: &str, line_num: usize) -> usize {
    let mut cur = Vec::new();
    let mut hm = HashSet::new();
    for _ in 0..deliverers {
        cur.push(Location(0, 0));
        hm.insert(Location(0, 0));
    }
    for (pos, e) in line.bytes().enumerate() {
        let ind = pos % deliverers;
        match e {
            b'>' => {
                cur[ind] = Location(cur[ind].0 + 1, cur[ind].1);
            }
            b'^' => {
                cur[ind] = Location(cur[ind].0, cur[ind].1 + 1);
            }
            b'v' => {
                cur[ind] = Location(cur[ind].0, cur[ind].1 - 1);
            }
            b'<' => {
                cur[ind] = Location(cur[ind].0 - 1, cur[ind].1);
            }
            _ => {
                panic!(
                    "{} - bad line. bad char {e} at pos {} - {line}",
                    line_num + 1,
                    pos + 1
                );
            }
        }
        hm.insert(cur[ind].clone());
    }
    hm.len()
}
