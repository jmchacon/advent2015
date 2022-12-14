//! day10 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use std::str;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("1113122113"))]
    start: String,

    #[arg(long, default_value_t = 40)]
    rounds: usize,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let mut cur = args.start.clone();
    for i in 0..args.rounds {
        //println!("{i} - {cur}");
        let mut new = String::new();
        let bytes = cur.as_str().as_bytes();
        let mut prev = bytes[0];
        let mut repeat = 1;
        let mut cc: [u8; 1] = [0];
        for c in &bytes[1..] {
            if *c == prev {
                repeat += 1;
                continue;
            }
            cc[0] = prev;
            let x = str::from_utf8(&cc).unwrap();
            new += &format!("{repeat}{x}");
            prev = *c;
            repeat = 1;
        }
        cc[0] = prev;
        let x = str::from_utf8(&cc).unwrap();
        new += &format!("{repeat}{x}");
        //println!("{i} - {cur} -> {new}");
        cur = new;
    }
    println!("len {}", cur.len());
    Ok(())
}
