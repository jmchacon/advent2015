//! day20 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = 34000000)]
    input: u64,

    #[arg(long, default_value_t = 1)]
    start: u64,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let (mut done1, mut done2) = (false, false);

    for start in args.start.. {
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_precision_loss,
            clippy::cast_sign_loss
        )]
        let root = (start as f64).sqrt() as u64;
        let mut div = Vec::new();
        // Find all divisors up to the square root.
        for j in 1..=root {
            if start % j == 0 {
                div.push(j);
            }
        }
        let mut f = div.clone();
        // For each one of the above divisors find the remaining
        // multiples and add those in.
        for d in div {
            if d * d != start {
                f.push(start / d);
            }
        }

        let sum = f.iter().sum::<u64>() * 10;
        let sum2 = f
            .iter()
            // Exclude anything that is above 50 deliveries
            .map(|x| if start / x > 50 { 0 } else { *x })
            .sum::<u64>()
            * 11;
        if sum >= args.input && !done1 {
            println!("part1: {start} -> {sum}");
            done1 = true;
        }
        if sum2 >= args.input && !done2 {
            println!("part2: {start} => {sum}");
            done2 = true;
        }
        if done1 && done2 {
            break;
        }
    }

    Ok(())
}
