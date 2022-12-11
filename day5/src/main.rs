//! day5 advent 2022
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
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut nice = 0;
    for line in &lines {
        let mut vowels = HashMap::new();
        let mut twice = false;
        let mut prev = b'-';
        let mut naughty = false;

        for c in line.as_str().as_bytes() {
            if *c == prev {
                twice = true;
            }
            match c {
                b'a' | b'e' | b'i' | b'o' | b'u' => {
                    vowels.entry(c).and_modify(|v| *v += 1).or_insert(1);
                }
                b'b' => {
                    if prev == b'a' {
                        naughty = true;
                    }
                }
                b'd' => {
                    if prev == b'c' {
                        naughty = true;
                    }
                }
                b'q' => {
                    if prev == b'p' {
                        naughty = true;
                    }
                }
                b'y' => {
                    if prev == b'x' {
                        naughty = true;
                    }
                }
                _ => {}
            }
            prev = *c;
        }

        if !naughty {
            if vowels.iter().map(|(_, v)| *v).sum::<i32>() >= 3 && twice {
                nice += 1;
                println!("{line} is nice");
            }
        }
    }
    println!("total nice = {nice}");
    Ok(())
}
