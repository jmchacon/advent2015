//! day12 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use serde_json::Value;
use std::fs::File;
use std::io::{self, BufRead};
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

    let mut num = 0;
    let mut num2 = 0;
    for (line_num, line) in lines.iter().enumerate() {
        let Ok(v) = serde_json::from_str::<Value>(line) else {
            panic!("{} - bad line {line}", line_num+1);
        };

        num += total(&v, false);
        num2 += total(&v, true);
        if args.debug {
            println!("{v}");
        }
    }
    println!("part1: {num}");
    println!("part2: {num2}");
    Ok(())
}

fn total(v: &Value, ignore_red: bool) -> i64 {
    match v {
        Value::Null | Value::String(_) | Value::Bool(_) => 0,
        Value::Number(v) => v.as_i64().unwrap(),
        Value::Array(vals) => vals.iter().map(|x| total(x, ignore_red)).sum(),
        Value::Object(m) => {
            if ignore_red && m.values().any(|x| x == "red") {
                0
            } else {
                m.values().map(|x| total(x, ignore_red)).sum()
            }
        }
    }
}
