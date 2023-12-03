//! day2 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
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

    let mut total: u32 = 0;
    let mut ribbon: u32 = 0;
    for (line_num, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split('x').collect();
        assert!(parts.len() == 3, "{} - bad line {line}", line_num + 1);
        let l = parts[0].parse::<u32>()?;
        let w = parts[1].parse::<u32>()?;
        let h = parts[2].parse::<u32>()?;
        let areas = [l * w, w * h, h * l];
        total += areas.iter().map(|a| 2 * a).sum::<u32>();
        total += areas.iter().min().unwrap();

        let perimeters = [2 * w + 2 * h, 2 * l + 2 * h, 2 * w + 2 * l];
        let vol = l * w * h;
        ribbon += perimeters.iter().min().unwrap() + vol;
    }
    println!("part1: total - {total}\npart2: ribbon - {ribbon}");
    Ok(())
}
