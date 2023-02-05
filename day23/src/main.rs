//! day23 advent 2015
use crate::Instruction::*;
use clap::Parser;
use color_eyre::eyre::Result;
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

#[derive(Clone, Debug, Display, Hash, PartialEq, Eq)]
enum Instruction {
    Hlf(usize),
    Tpl(usize),
    Inc(usize),
    Jmp(isize),
    Jie(usize, isize),
    Jio(usize, isize),
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut regs = [0_u64, 0_u64];
    let mut instructions = Vec::new();
    for (line_num, line) in lines.iter().enumerate() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() > 1, "{} - bad line {line}", line_num + 1);

        let ins = match parts[0] {
            "hlf" => {
                assert!(parts.len() == 2, "{} - bad line {line}", line_num + 1);
                Hlf(reg(parts[1], line, line_num))
            }
            "tpl" => {
                assert!(parts.len() == 2, "{} - bad line {line}", line_num + 1);
                Tpl(reg(parts[1], line, line_num))
            }
            "inc" => {
                assert!(parts.len() == 2, "{} - bad line {line}", line_num + 1);
                Inc(reg(parts[1], line, line_num))
            }
            "jmp" => {
                assert!(parts.len() == 2, "{} - bad line {line}", line_num + 1);
                Jmp(isize::from_str_radix(parts[1], 10).unwrap())
            }
            "jie" => {
                assert!(parts.len() == 3, "{} - bad line {line}", line_num + 1);
                Jie(
                    reg(parts[1], line, line_num),
                    isize::from_str_radix(parts[2], 10).unwrap(),
                )
            }
            "jio" => {
                assert!(parts.len() == 3, "{} - bad line {line}", line_num + 1);
                Jio(
                    reg(parts[1], line, line_num),
                    isize::from_str_radix(parts[2], 10).unwrap(),
                )
            }
            _ => panic!("{} - bad line {line}", line_num + 1),
        };
        instructions.push(ins);
    }

    for i in &instructions {
        println!("{i:?}");
    }

    run(&instructions, &mut regs);
    println!("A: {} B: {}", regs[0], regs[1]);

    regs[0] = 1;
    regs[1] = 0;
    run(&instructions, &mut regs);
    println!("A: {} B: {}", regs[0], regs[1]);
    Ok(())
}

fn run(instructions: &Vec<Instruction>, regs: &mut [u64; 2]) {
    let mut idx = 0;
    loop {
        match &instructions[idx as usize] {
            Hlf(r) => {
                regs[*r] /= 2;
                idx += 1;
            }
            Tpl(r) => {
                regs[*r] *= 3;
                idx += 1;
            }
            Inc(r) => {
                regs[*r] += 1;
                idx += 1;
            }
            Jmp(off) => {
                idx += off;
            }
            Jie(r, off) => {
                if regs[*r] % 2 == 0 {
                    idx += off
                } else {
                    idx += 1;
                }
            }
            Jio(r, off) => {
                if regs[*r] == 1 {
                    idx += off
                } else {
                    idx += 1;
                }
            }
        };
        if idx < 0 || idx >= instructions.len() as isize {
            break;
        }
    }
}

fn reg(p: &str, line: &str, line_num: usize) -> usize {
    match p.trim_end_matches(",") {
        "a" => 0_usize,
        "b" => 1_usize,
        _ => panic!("{} - bad line {line}", line_num + 1),
    }
}
