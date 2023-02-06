//! day11 advent 2015
use std::collections::HashMap;

use clap::Parser;
use color_eyre::eyre::Result;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("cqjxjnds"))]
    input: String,

    #[arg(long, default_value_t = usize::MAX)]
    iterations: usize,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let mut p = args.input.clone();

    let pass = unsafe { p.as_bytes_mut() };

    let mut part = 1;
    loop {
        if part > 2 {
            break;
        }
        if args.debug {
            // SAFETY: We know this is valid utf8 ascii as we just converted it
            // or finished.
            unsafe {
                println!("{}", std::str::from_utf8(pass).unwrap_unchecked());
            }
        }
        for i in 0..args.iterations {
            increment(pass);
            if args.debug {
                // SAFETY: We know this is valid utf8 ascii so increment will
                //         still leave it in a state we can blind convert.
                unsafe {
                    println!("{}", std::str::from_utf8(pass).unwrap_unchecked());
                }
            }
            if test1(pass) && test2(pass) && test3(pass) {
                // SAFETY: We know this is valid utf8 ascii so increment will
                //         still leave it in a state we can blind convert.
                unsafe {
                    println!(
                        "part{part}: {}",
                        std::str::from_utf8(pass).unwrap_unchecked()
                    );
                    part += 1;
                }
                if args.debug {
                    println!("Found in {i} iterations");
                }
                break;
            }
        }
    }
    Ok(())
}

fn increment(pass: &mut [u8]) {
    for i in (0..pass.len()).rev() {
        pass[i] += 1;
        if pass[i] > b'z' {
            pass[i] = b'a';
        } else {
            break;
        }
    }
}

// Passwords must include one increasing straight of at least three letters,
// like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
fn test1(pass: &[u8]) -> bool {
    for i in 0..pass.len() - 2 {
        // i.e. abc if a+1 == b and b+1 == c
        if pass[i] + 1 == pass[i + 1] && pass[i + 1] + 1 == pass[i + 2] {
            return true;
        }
    }
    false
}

// Passwords may not contain the letters i, o, or l, as these letters can be
// mistaken for other characters and are therefore confusing.
fn test2(pass: &[u8]) -> bool {
    pass.iter()
        .copied()
        .filter(|x| *x != b'i' && *x != b'o' && *x != b'l')
        .count()
        == pass.len()
}

// Passwords must contain at least two different, non-overlapping pairs of letters, like aa, bb, or zz.
fn test3(pass: &[u8]) -> bool {
    let mut idxs = HashMap::new();
    for i in 0..pass.len() - 1 {
        if pass[i] == pass[i + 1] {
            idxs.insert(pass[i], i);
        }
    }
    idxs.len() >= 2
}
