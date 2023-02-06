//! day7 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use strum_macros::Display;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Clone, Debug, Display)]
enum Input {
    Val(u16),
    Var(String),
}

#[derive(Clone, Debug, Display)]
enum Operator {
    And(Input),
    Or(Input),
    Lshift(u16),
    Rshift(u16),
    Not,
    Assign,
}

#[derive(Clone, Debug)]
struct Operation {
    operator: Operator,
    dest: String,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut hm = HashMap::new();
    let mut vals = HashMap::new();

    for (line_num, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        assert!(
            parts.len() >= 3 && parts.len() <= 5,
            "{} - bad line {line}",
            line_num + 1
        );

        parse_line(&parts, &mut hm, &mut vals, line, line_num);
    }
    if args.debug {
        println!("hm:");
        let mut keys: Vec<&String> = hm.keys().collect();
        keys.sort();
        for k in keys {
            println!("{k} - {:?}", hm[k]);
        }
        println!("vals:");
        let mut keys: Vec<&String> = vals.keys().collect();
        keys.sort();
        for k in keys {
            println!("{k} - {}", vals[k]);
        }
    }
    let hm2 = hm.clone();
    let mut vals2 = vals.clone();

    let mut iter = 0;
    loop {
        if vals.contains_key("a") {
            let val = vals.get(&String::from("a")).unwrap();
            println!("part{}: found a - {val}", iter + 1);
            vals2.insert(String::from("b"), *val);
            iter += 1;
            if iter >= 2 {
                break;
            }
            hm = hm2.clone();
            vals = vals2.clone();
        }
        let mut skipped = false;
        let mut keys = vals.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        for k in &keys {
            if !hm.contains_key(k) {
                skipped = true;
                continue;
            }

            let val = vals[k];
            for op in &hm[k] {
                match &op.operator {
                    Operator::And(x) | Operator::Or(x) => {
                        let other = match x {
                            Input::Val(o) => *o,
                            Input::Var(s) => {
                                if !vals.contains_key(s) {
                                    skipped = true;
                                    continue;
                                }
                                vals[s]
                            }
                        };
                        if let Operator::And(_) = &op.operator {
                            vals.insert(op.dest.clone(), val & other);
                        } else {
                            vals.insert(op.dest.clone(), val | other);
                        }
                    }
                    Operator::Lshift(v) => {
                        vals.insert(op.dest.clone(), val << *v);
                    }
                    Operator::Rshift(v) => {
                        vals.insert(op.dest.clone(), val >> *v);
                    }
                    Operator::Not => {
                        vals.insert(op.dest.clone(), !val);
                    }
                    Operator::Assign => {
                        vals.insert(op.dest.clone(), val);
                    }
                }
            }
        }
        if !skipped {
            break;
        }
    }

    Ok(())
}

fn parse_line(
    parts: &Vec<&str>,
    hm: &mut HashMap<String, Vec<Operation>>,
    vals: &mut HashMap<String, u16>,
    line: &str,
    line_num: usize,
) {
    match parts.len() {
        3 => {
            if let Ok(v) = parts[0].parse::<u16>() {
                vals.insert(String::from(parts[2]), v);
            } else {
                let e = String::from(parts[0]);
                let op = Operation {
                    operator: Operator::Assign,
                    dest: String::from(parts[2]),
                };
                hm.entry(e)
                    .and_modify(|v: &mut Vec<Operation>| v.push(op.clone()))
                    .or_insert(vec![op]);
            }
        }
        4 => {
            // all NOT
            assert!(parts[0] == "NOT", "{} - bad line {line}", line_num + 1);
            let e = String::from(parts[1]);
            let op = Operation {
                operator: Operator::Not,
                dest: String::from(parts[3]),
            };
            hm.entry(e)
                .and_modify(|v: &mut Vec<Operation>| v.push(op.clone()))
                .or_insert(vec![op]);
        }
        5 => {
            let op;
            let mut e = String::from(parts[0]);
            match *parts.get(1).unwrap() {
                "AND" => {
                    // Special case based on known input.
                    if parts[0] == "1" {
                        e = String::from(parts[2]);
                        op = Operation {
                            operator: Operator::And(Input::Val(1)),
                            dest: String::from(parts[4]),
                        };
                    } else {
                        op = Operation {
                            operator: Operator::And(Input::Var(String::from(parts[2]))),
                            dest: String::from(parts[4]),
                        };
                    }
                }
                "OR" => {
                    op = Operation {
                        operator: Operator::Or(Input::Var(String::from(parts[2]))),
                        dest: String::from(parts[4]),
                    };
                }
                "LSHIFT" => {
                    let v = parts[2].parse::<u16>().unwrap();
                    op = Operation {
                        operator: Operator::Lshift(v),
                        dest: String::from(parts[4]),
                    };
                }
                "RSHIFT" => {
                    let v = parts[2].parse::<u16>().unwrap();
                    op = Operation {
                        operator: Operator::Rshift(v),
                        dest: String::from(parts[4]),
                    };
                }
                _ => {
                    panic!("{} - bad line {line}", line_num + 1);
                }
            }
            hm.entry(e)
                .and_modify(|v: &mut Vec<Operation>| v.push(op.clone()))
                .or_insert(vec![op]);
        }
        _ => {
            panic!("{} - bad line {line}", line_num + 1);
        }
    }
}
