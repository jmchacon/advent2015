//! day4 advent 2015
use color_eyre::eyre::Result;
use md5;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    for line in &lines {
        for i in 1.. {
            let digest = md5::compute(format!("{line}{i}"));
            match format!("{:x}", digest).as_bytes() {
                [b'0', b'0', b'0', b'0', b'0', b'0', ..] => {
                    println!("Found 6 at {i} - {line}");
                    break;
                }
                [b'0', b'0', b'0', b'0', b'0', ..] => {
                    println!("Found 5 at {i} - {line}");
                }
                _ => {}
            }
        }
    }

    Ok(())
}
