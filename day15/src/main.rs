//! day15 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::{io, iter};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Clone, Debug)]
struct Ingrediant<'a> {
    _name: &'a str,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut ingrediants = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() == 11, "{} bad line - {line}", line_num + 1);
        ingrediants.push(Ingrediant {
            _name: parts[0].trim_end_matches(':'),
            capacity: parts[2].trim_end_matches(',').parse::<_>().unwrap(),
            durability: parts[4].trim_end_matches(',').parse::<_>().unwrap(),
            flavor: parts[6].trim_end_matches(',').parse::<_>().unwrap(),
            texture: parts[8].trim_end_matches(',').parse::<_>().unwrap(),
            calories: parts[10].trim_end_matches(',').parse::<_>().unwrap(),
        });
    }

    if args.debug {
        for i in &ingrediants {
            println!("{i:?}");
        }
    }

    let mut best = i64::MIN;
    let mut best_with_500_cal = i64::MIN;
    let l: i64 = ingrediants.len().try_into()?;

    for j in (1..100_i64)
        .permutations(ingrediants.len())
        .filter(|x| x.iter().sum::<i64>() == 100)
        .chain(iter::once(vec![100 / l; ingrediants.len()]))
    {
        let (mut c, mut d, mut f, mut t, mut cal) = (0, 0, 0, 0, 0);
        for (pos, x) in j.iter().enumerate() {
            c += ingrediants[pos].capacity * *x;
            d += ingrediants[pos].durability * *x;
            f += ingrediants[pos].flavor * *x;
            t += ingrediants[pos].texture * *x;
            cal += ingrediants[pos].calories * *x;
        }
        // Negative == 0 which means skip computing.
        if c < 0 || d < 0 || f < 0 || t < 0 {
            continue;
        }
        let tot = c * d * f * t;
        if tot > best {
            best = tot;
        }
        if cal == 500 && tot > best_with_500_cal {
            best_with_500_cal = tot;
        }
    }
    println!("part1: best is {best}");
    println!("part2: best with 500 cal is {best_with_500_cal}");
    Ok(())
}

/* fn compute_slots(total: usize, n: usize) -> Vec<Vec<usize>> {
    let ret = Vec::new();
    let mut pos = n;
    for f in 1..total {
        pos -=1;
        if pos > 0 {
            for g in
        }

    }
    ret
}
 */
