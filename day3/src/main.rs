//! day3 advent 2022
use color_eyre::eyre::Result;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Location(i32, i32);

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    for (line_num, line) in lines.iter().enumerate() {
        let mut cur = Location(0, 0);
        let mut hm = HashSet::from([cur.clone()]);
        for (pos, e) in line.bytes().enumerate() {
            match e {
                b'>' => {
                    cur = Location(cur.0 + 1, cur.1);
                }
                b'^' => {
                    cur = Location(cur.0, cur.1 + 1);
                }
                b'v' => {
                    cur = Location(cur.0, cur.1 - 1);
                }
                b'<' => {
                    cur = Location(cur.0 - 1, cur.1);
                }
                _ => {
                    panic!(
                        "{} - bad line. bad char {e} at pos {} - {line}",
                        line_num + 1,
                        pos + 1
                    );
                }
            }
            hm.insert(cur.clone());
        }
        println!("Visited {} houses", hm.len());
    }

    Ok(())
}
