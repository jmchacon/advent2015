//! day2 advent 2022
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn main() -> Result<()> {
    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut total: u32 = 0;
    let mut ribbon: u32 = 0;
    for (line_num, line) in lines.iter().enumerate() {
        let parts: Vec<&str> = line.split("x").collect();
        assert!(parts.len() == 3, "{} - bad line {line}", line_num + 1);
        let l = u32::from_str_radix(parts[0], 10).unwrap();
        let w = u32::from_str_radix(parts[1], 10).unwrap();
        let h = u32::from_str_radix(parts[2], 10).unwrap();
        let areas = vec![l * w, w * h, h * l];
        total += areas.iter().map(|a| 2 * a).sum::<u32>();
        total += areas.iter().min().unwrap();

        let perimeters = vec![2 * w + 2 * h, 2 * l + 2 * h, 2 * w + 2 * l];
        let vol = l * w * h;
        ribbon += perimeters.iter().min().unwrap() + vol;
    }
    println!("total - {total}\nribbon - {ribbon}");
    Ok(())
}
