//! day9 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use itertools::Itertools;
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

    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut hm: HashMap<String, HashMap<String, usize>> = HashMap::new();

    for (line_num, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(parts.len() == 5, "{} - bad line {line}", line_num + 1);
        let val = parts[4].parse::<usize>().unwrap();
        let key = String::from(parts[0]);
        let second = String::from(parts[2]);
        if !hm.contains_key(&key) {
            hm.insert(key.clone(), HashMap::new());
        }
        if !hm.contains_key(&second) {
            hm.insert(second.clone(), HashMap::new());
        }
        hm.get_mut(&key).unwrap().insert(second.clone(), val);
        hm.get_mut(&second).unwrap().insert(key.clone(), val);
    }
    if args.debug {
        for (k, v) in &hm {
            println!("{k} -> {v:?}");
        }
    }
    let perms = hm.keys().permutations(hm.len());
    if args.debug {
        println!("permutations");
    }
    let mut res = Vec::new();
    for p in perms {
        let mut tot = 0;
        for i in 0..p.len() - 1 {
            tot += hm.get(p[i]).unwrap().get(p[i + 1]).unwrap();
        }
        res.push(tot);
        if args.debug {
            println!("{p:?} - {tot}");
        }
    }
    println!("part1: min: {}", res.iter().min().unwrap());
    println!("part2: max: {}", res.iter().max().unwrap());
    Ok(())
}
