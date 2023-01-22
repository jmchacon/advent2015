//! day20 advent 2022
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
        let root = (start as f64).sqrt() as u64;
        let mut div = Vec::new();
        // Find all divisors up to the square root.
        for j in 1..=root + 1 {
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
            .map(|x| if start / x <= 50 { *x } else { 0 })
            .sum::<u64>()
            * 11;
        if sum >= args.input && !done1 {
            println!("{start} -> {sum}");
            done1 = true;
        }
        if sum2 >= args.input && !done2 {
            println!("part2 {start} => {sum}");
            done2 = true;
        }
        if done1 && done2 {
            break;
        }
    }

    Ok(())
}
