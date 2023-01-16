//! day16 advent 2022
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

#[derive(Clone, Debug, Default)]
struct Sue {
    children: u64,
    cats: u64,
    samoyeds: u64,
    pomeranians: u64,
    akitas: u64,
    vizslas: u64,
    goldfish: u64,
    trees: u64,
    cars: u64,
    perfumes: u64,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut sues = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() == 8, "{} - bad line {line}", line_num + 1);
        assert!(
            sues.len() == usize::from_str_radix(parts[1].trim_end_matches(":"), 10).unwrap() - 1,
            "{} - bad sue {} {line}",
            sues.len(),
            line_num + 1
        );
        let mut sue = Sue::default();
        for (pos, m) in [parts[2], parts[4], parts[6]].iter().cloned().enumerate() {
            match m {
                "children:" => {
                    sue.children =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "cats:" => {
                    sue.cats =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "samoyeds:" => {
                    sue.samoyeds =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "pomeranians:" => {
                    sue.pomeranians =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "akitas:" => {
                    sue.akitas =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "vizslas:" => {
                    sue.vizslas =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "goldfish:" => {
                    sue.goldfish =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "trees:" => {
                    sue.trees =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "cars:" => {
                    sue.cars =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                "perfumes:" => {
                    sue.perfumes =
                        u64::from_str_radix(parts[pos * 2 + 3].trim_end_matches(","), 10).unwrap()
                }
                _ => panic!("{} - bad line {m} - {line}", line_num + 1),
            }
        }
        sues.push(sue);
    }

    //    for (pos, s) in sues.iter().enumerate() {
    //      println!("Sue {} -> {s:?}", pos + 1);
    // }
    let m = Sue {
        children: 3,
        cats: 7,
        samoyeds: 2,
        pomeranians: 3,
        akitas: 0,
        vizslas: 0,
        goldfish: 5,
        trees: 3,
        cars: 2,
        perfumes: 1,
    };

    let rem = sues
        .iter()
        .enumerate()
        .filter(|(_, s)| s.children == m.children || s.children == 0)
        .filter(|(_, s)| s.cats == m.cats || s.cats == 0)
        .filter(|(_, s)| s.samoyeds == m.samoyeds || s.samoyeds == 0)
        .filter(|(_, s)| s.pomeranians == m.pomeranians || s.pomeranians == 0)
        .filter(|(_, s)| s.akitas == m.akitas || s.akitas == 0)
        .filter(|(_, s)| s.vizslas == m.vizslas || s.vizslas == 0)
        .filter(|(_, s)| s.goldfish == m.goldfish || s.goldfish == 0)
        .filter(|(_, s)| s.trees == m.trees || s.trees == 0)
        .filter(|(_, s)| s.cars == m.cars || s.cars == 0)
        .filter(|(_, s)| s.perfumes == m.perfumes || s.perfumes == 0)
        .collect::<Vec<_>>();
    println!("Remaining: {rem:?}");
    println!("The Sue is {}", rem[0].0 + 1);

    let rem = sues
        .iter()
        .enumerate()
        .filter(|(_, s)| s.children == m.children || s.children == 0)
        .filter(|(_, s)| s.cats > m.cats || s.cats == 0)
        .filter(|(_, s)| s.samoyeds == m.samoyeds || s.samoyeds == 0)
        .filter(|(_, s)| s.pomeranians < m.pomeranians || s.pomeranians == 0)
        .filter(|(_, s)| s.akitas == m.akitas || s.akitas == 0)
        .filter(|(_, s)| s.vizslas == m.vizslas || s.vizslas == 0)
        .filter(|(_, s)| s.goldfish < m.goldfish || s.goldfish == 0)
        .filter(|(_, s)| s.trees > m.trees || s.trees == 0)
        .filter(|(_, s)| s.cars == m.cars || s.cars == 0)
        .filter(|(_, s)| s.perfumes == m.perfumes || s.perfumes == 0)
        .collect::<Vec<_>>();
    println!("Remaining: {rem:?}");
    println!("The Sue for part2  is {}", rem[0].0 + 1);
    Ok(())
}
