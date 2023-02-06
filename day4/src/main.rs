//! day4 advent 2015
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

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let (mut found_5, mut found_6) = (false, false);
    'outer: for line in &lines {
        for i in 1.. {
            let digest = md5::compute(format!("{line}{i}"));
            match format!("{digest:x}").as_bytes() {
                [b'0', b'0', b'0', b'0', b'0', b'0', ..] => {
                    if !found_6 {
                        println!("part2: Found 6 at {i} - {line}");
                        found_6 = true;
                    }
                }
                [b'0', b'0', b'0', b'0', b'0', ..] => {
                    if !found_5 {
                        println!("part1: Found 5 at {i} - {line}");
                        found_5 = true;
                    }
                }
                _ => {}
            }
            if found_5 && found_6 {
                break 'outer;
            }
        }
    }

    Ok(())
}
