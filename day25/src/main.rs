//! day25 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,

    #[arg(long, default_value_t = 3010)]
    row: u64,

    #[arg(long, default_value_t = 3019)]
    column: u64,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let mut row = 1_u64;
    let mut col = 6_u64;
    let mut val = 33_511_524_u64;
    let mult = 252_533_u64;
    let rem = 33_554_393_u64;
    loop {
        if row == 1 {
            row = col + 1;
            col = 1;
        } else {
            row -= 1;
            col += 1;
        }
        val = val * mult % rem;
        if row == args.row && col == args.column {
            break;
        }
    }
    println!("part1: {val} is at {},{}", args.row, args.column);
    Ok(())
}
