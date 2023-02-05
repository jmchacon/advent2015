//! day8 advent 2015
use clap::Parser;
use color_eyre::eyre::Result;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::str;

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

    let mut code = 0;
    let mut chars = 0;
    let mut encoded_code = 0;
    for (line_num, line) in lines.iter().enumerate() {
        let mut local_chars = 0;
        let mut local_code = 0;
        let mut local_encoded_code = 0;
        assert!(line.len() >= 2, "{} - bad line {line}", line_num + 1);
        let raw = line.as_bytes();
        let last = raw.len() - 1;
        assert!(
            raw[0] == b'"' && raw[last] == b'"',
            "{} - bad line {line}",
            line_num + 1
        );

        local_code += raw.len();
        local_encoded_code += raw.len() + 4; // Have to escape first and last " to "\"
        let mut pos = 1;
        loop {
            if pos >= last {
                break;
            }
            match raw[pos] {
                b'\\' => {
                    pos += 1;
                    local_encoded_code += 1;
                    assert!(pos < last, "{} - bad line {line}", line_num + 1);
                    match raw[pos] {
                        b'\\' | b'"' => {
                            local_chars += 1;
                            local_encoded_code += 1;
                            pos += 1;
                        }
                        b'x' => {
                            pos += 1;
                            // Need 2 chars here
                            assert!(pos < last - 1, "{} - bad line {line}", line_num + 1);
                            let _ =
                                u8::from_str_radix(str::from_utf8(&raw[pos..pos + 2]).unwrap(), 16)
                                    .unwrap();
                            pos += 2;
                            local_chars += 1;
                        }
                        _ => {
                            panic!("{} - bad line {line}", line_num + 1);
                        }
                    }
                }
                _ => {
                    local_chars += 1;
                    pos += 1;
                }
            };
        }
        println!("{line} - {local_code} {local_chars} {local_encoded_code}");
        code += local_code;
        chars += local_chars;
        encoded_code += local_encoded_code;
    }
    println!("code - {code}");
    println!("chars - {chars}");
    println!("diff - {}", code - chars);
    println!("encoded_code - {}", encoded_code);
    println!("diff2 - {}", encoded_code - code);
    Ok(())
}
