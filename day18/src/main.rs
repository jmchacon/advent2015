//! day18 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use grid::{Grid, Location};
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

    #[arg(long, default_value_t = 100)]
    rounds: usize,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

#[derive(Clone, Debug, Default, Display, PartialEq, Eq)]
enum Light {
    On,
    #[default]
    Off,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut grid = Grid::<Light>::new(lines[0].len(), lines.len());
    for (line_num, line) in lines.iter().enumerate() {
        for (pos, b) in line.as_bytes().iter().enumerate() {
            let l = Location(pos, line_num);
            match b {
                b'#' => grid.add(&l, Light::On),
                b'.' => {}
                _ => panic!("{} - bad line {line}", line_num + 1),
            };
        }
    }

    let mut orig_grid = grid.clone();

    print_board(&grid);
    println!();

    for _ in 0..args.rounds {
        step(&mut grid, false);
        if args.debug {
            print_board(&grid);
            println!();
        }
    }

    let on = grid.iter().filter(|(_, x)| *x == &Light::On).count();
    println!("{on} lights on after {} rounds", args.rounds);
    if args.debug {
        println!();
    }
    for _ in 0..args.rounds {
        step(&mut orig_grid, true);
        if args.debug {
            print_board(&orig_grid);
            println!();
        }
    }

    let on = orig_grid.iter().filter(|(_, x)| *x == &Light::On).count();
    println!(
        "{on} lights with stuck corners on after {} rounds",
        args.rounds
    );
    Ok(())
}

fn step(grid: &mut Grid<Light>, corners: bool) {
    let mut newgrid = Grid::new(grid.width(), grid.height());
    // If corners is true the 4 corners always remain on.
    // So we set that now so references to them work and then below
    // force the corners to remain.
    if corners {
        grid.add(&Location(0, 0), Light::On);
        grid.add(&Location(grid.width() - 1, 0), Light::On);
        grid.add(&Location(0, grid.height() - 1), Light::On);
        grid.add(&Location(grid.width() - 1, grid.height() - 1), Light::On);
    }
    for gr in grid.iter() {
        let l = gr.0;
        let mut g = gr.1.clone();
        if corners
            && (l == Location(0, 0)
                || l == Location(grid.width() - 1, 0)
                || l == Location(0, grid.height() - 1)
                || l == Location(grid.width() - 1, grid.height() - 1))
        {
            g = Light::On;
        } else {
            let n = grid
                .neighbors_all(&l)
                .iter()
                .filter(|x| *x.1 == Light::On)
                .count();
            match g {
                Light::On => {
                    if n != 2 && n != 3 {
                        g = Light::Off;
                    }
                }
                Light::Off => {
                    if n == 3 {
                        g = Light::On;
                    }
                }
            }
        }
        newgrid.add(&l, g);
    }
    *grid = newgrid;
}

fn print_board(grid: &Grid<Light>) {
    for g in grid {
        match g.1 {
            Light::On => print!("#"),
            Light::Off => print!("."),
        }
    }
}
