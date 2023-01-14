//! day13 advent 2022
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

    let mut happy = HashMap::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        assert!(parts.len() == 11, "{} - bad line {line}", line_num + 1);
        let mut happiness = i64::from_str_radix(parts[3], 10).unwrap();
        if parts[2] == "lose" {
            happiness *= -1;
        } else {
            assert!(parts[2] == "gain", "{} - bad line {line}", line_num + 1);
        }
        let sit = parts[10].trim_end_matches(".");
        happy
            .entry(parts[0])
            .and_modify(|v: &mut HashMap<&str, i64>| {
                v.insert(sit, happiness);
            })
            .or_insert(HashMap::from([(sit, happiness)]));
    }

    run_table(&happy, args.debug);

    // Part2 is add self
    let keys = happy.keys().cloned().collect::<Vec<_>>();
    for k in keys {
        happy
            .entry("Self")
            .and_modify(|v: &mut HashMap<&str, i64>| {
                v.insert(k, 0);
            })
            .or_insert(HashMap::from([(k, 0)]));
        happy
            .entry(k)
            .and_modify(|v: &mut HashMap<&str, i64>| {
                v.insert("Self", 0);
            })
            .or_insert(HashMap::from([("Self", 0)]));
    }

    run_table(&happy, args.debug);
    Ok(())
}

fn run_table(happy: &HashMap<&str, HashMap<&str, i64>>, debug: bool) {
    println!("Table size is {}", happy.len());
    if debug {
        for (person, v) in happy {
            for (sit, happy) in v {
                println!("{person} -> {sit} - {happy}");
            }
        }
    }

    let mut max = i64::MIN;
    for p in happy.keys().cloned().permutations(happy.len()) {
        let new = compute_happiness(&happy, &p);
        if debug {
            println!("{p:?} - {new}");
        }
        if new > max {
            max = new;
        }
    }
    println!("max is {max}");
}

fn compute_happiness(happy: &HashMap<&str, HashMap<&str, i64>>, table: &Vec<&str>) -> i64 {
    let mut h = 0;
    let last = table.len() - 1;
    // Get all combos except last person->first person
    // You have to get both the forward happiness and the reverse for each person.
    for i in 0..last {
        h += happy.get(table[i]).unwrap().get(table[i + 1]).unwrap();
        h += happy.get(table[i + 1]).unwrap().get(table[i]).unwrap();
    }
    h += happy.get(table[last]).unwrap().get(table[0]).unwrap();
    h += happy.get(table[0]).unwrap().get(table[last]).unwrap();
    h
}
