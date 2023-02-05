//! day1 advent 2015
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

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
                println!("{} first floor -1 pos", pos + 1);
            }
        }
        println!("Ended on {floor}");
    }

    Ok(())
}
