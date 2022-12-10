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

    const DELIVERERS: usize = 2;
    for (line_num, line) in lines.iter().enumerate() {
        let mut cur = Vec::new();
        let mut hm = HashSet::new();
        for _ in 0..DELIVERERS {
            cur.push(Location(0, 0));
            hm.insert(Location(0, 0));
        }
        for (pos, e) in line.bytes().enumerate() {
            let ind = pos % DELIVERERS;
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
        println!("Visited {} houses", hm.len());
    }

    Ok(())
}
