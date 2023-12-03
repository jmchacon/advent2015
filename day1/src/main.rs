//! day1 advent 2015
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

    let mut part2 = 0;
    for (line_num, line) in lines.iter().enumerate() {
        let mut floor = 0;
        let mut found = false;
        for (pos, e) in line.bytes().enumerate() {
            match e {
                b'(' => {
                    floor += 1;
                }
                b')' => {
                    floor -= 1;
                }
                _ => {
                    panic!(
                        "{} - bad line {line} at postion {} - {e}",
                        line_num + 1,
                        pos + 1
                    );
                }
            }
            if !found && floor == -1 {
                found = true;
                part2 = pos + 1;
            }
        }
        println!("part1: Ended on {floor}");
        println!("part2: {part2} first floor -1 pos");
    }

    Ok(())
}
