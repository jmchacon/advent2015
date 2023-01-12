//! day12 advent 2022
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
        let Ok(v) = serde_json::from_str::<Value>(&line) else {
            panic!("{} - bad line {line}", line_num+1);
        };

        num += total(&v, false);
        num2 += total(&v, true);
        println!("{v}");
    }
    println!("{num}");
    println!("{num2}");
    Ok(())
}

fn total(v: &Value, ignore_red: bool) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(v) => v.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(vals) => {
            let mut num = 0;
            for v in vals {
                num += total(v, ignore_red);
            }
            num
        }
        Value::Object(m) => {
            if ignore_red
                && m.iter()
                    .filter(|(_, v)| {
                        if let Value::String(s) = v {
                            if s == "red" {
                                return true;
                            }
                        }
                        false
                    })
                    .count()
                    != 0
            {
                return 0;
            };
            let mut num = 0;
            for (_, v) in m {
                num += total(v, ignore_red)
            }
            num
        }
    }
}
