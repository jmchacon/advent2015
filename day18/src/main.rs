//! day18 advent 2022
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

    print_board(&grid);
    println!();

    for _ in 0..args.rounds {
        step(&mut grid);
        if args.debug {
            print_board(&grid);
            println!();
        }
    }

    let on = grid.into_iter().filter(|x| *x == Light::On).count();
    println!("{on} lights on after {} rounds", args.rounds);
    Ok(())
}

fn step(grid: &mut Grid<Light>) {
    let mut newgrid = Grid::new(grid.width(), grid.height());
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            let l = Location(x, y);
            let mut g = grid.get(&l).clone();
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
            newgrid.add(&l, g);
        }
    }
    *grid = newgrid;
}

fn print_board(grid: &Grid<Light>) {
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            match grid.get(&Location(x, y)) {
                Light::On => print!("#"),
                Light::Off => print!("."),
            }
        }
        println!();
    }
}
