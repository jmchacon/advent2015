//! day19 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(long, default_value_t = String::from("input.txt"))]
    filename: String,

    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args: Args = Args::parse();

    let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join(args.filename);
    let file = File::open(filename)?;
    let lines: Vec<String> = io::BufReader::new(file).lines().flatten().collect();

    let mut transforms = HashMap::new();
    let goal = lines[lines.len() - 1].as_str();
    for (line_num, line) in lines.iter().enumerate() {
        // Blank line means we're done with the main part. Only thing after should
        // be the last line which we already snagged above.
        if line.len() == 0 {
            break;
        }
        let parts = line.split_whitespace().collect::<Vec<_>>();
        assert!(parts.len() == 3, "{} - bad line {line}", line_num + 1);
        // The input should have distinct right sides with N identical left.
        // So invert and store into a hashmap. This is simpler than keying off
        // the left and having to use a Vec of values.
        if transforms.insert(parts[2], parts[0]) != None {
            panic!("{} - bad line {line} - dup", line_num + 1);
        }
    }

    println!("start: {goal}");
    for (k, v) in &transforms {
        println!("{k} -> {v:?}");
    }

    let mut new = HashSet::new();
    for (v, k) in &transforms {
        if args.debug {
            println!("Transforming {goal} {k} with {v}");
        }
        new = new
            .union(&do_transform(goal, k, v, args.debug))
            .cloned()
            .collect();
    }
    println!("Count is {} for {goal}", new.len());

    println!();
    // Someone did an analysis of the input and since it's XRn..Ar sequences
    // and Y sequences and everything else is single chars you can reduce
    // this to a calculation.
    let ar = goal.matches("Ar").count();
    let rn = goal.matches("Rn").count();
    let y = goal.matches("Y").count();
    let upper = goal.bytes().filter(|x| *x >= b'A' && *x <= b'Z').count();
    println!(
        "{} {ar} {rn} {y} Took {} steps to get from e -> {goal}",
        upper,
        upper - rn - ar - 2 * y - 1
    );

    // The other way to do it...There is a path based on reducing the Rn..Ar sequences
    // first and then applying transforms to get back to e. But there are paths that
    // basically loop as we're doing DFS and keep repeating bad solutions. But tracking
    // this is too memory intensive. So we just Monte-Carlo and sample here since the
    // dfs() will randomize iteration of the transforms. So it won't work for more than 3s
    // before giving up and we try again.
    // Generally this finds a solution within 10s but immediate is about 300ms.
    loop {
        let steps = dfs(Instant::now(), goal, "e", &transforms, 0);
        if steps != usize::MAX {
            println!("Steps are {steps}");
            break;
        }
        println!("try");
    }
    Ok(())
}

fn dfs(now: Instant, cur: &str, m: &str, transforms: &HashMap<&str, &str>, cost: usize) -> usize {
    //println!("Checking {cur} at {cost}");

    if now.elapsed().as_secs() > 3 {
        return usize::MAX;
    }
    if cur == m {
        return cost;
    }

    let mut test = String::from(cur);
    let mut new_cost = cost;
    // Reduce all the Rn..Y[F|Mg]Ar sequences first since these are terminal and can't be separated.
    'outer: loop {
        for (k, v) in transforms
            .iter()
            .filter(|(k, _)| k.ends_with("YFAr") || k.ends_with("YMgAr"))
        {
            if let Some(t) = do_transform(&test, k, v, false).iter().next() {
                test = t.clone();
                new_cost += 1;
            } else {
                break 'outer;
            }
        }
    }

    // Randomize rest of the attempts as one of these will find the solution.
    let mut trs = transforms.iter().collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    trs.shuffle(&mut rng);

    println!("{:?}", trs[0]);
    for (k, v) in trs {
        if *v == "e" && test.len() != k.len() {
            continue;
        }
        let c = do_transform(&test, k, v, false);
        for new in c {
            let found = dfs(now, &new, m, transforms, new_cost + 1);
            if found != usize::MAX {
                return found;
            }
        }
    }
    usize::MAX
}

fn do_transform(start: &str, k: &str, trans: &str, debug: bool) -> HashSet<String> {
    if debug {
        println!("Calling with {start} {k} {trans}");
    }
    let mut ret = HashSet::new();
    for i in start.match_indices(k) {
        // Split into 2 chunks where we've removed the bit we're replacing.
        let (pre, rem) = start.split_at(i.0);
        let (_, post) = rem.split_at(k.len());
        // Now recombine into a new string.
        ret.insert(format!("{pre}{trans}{post}"));
    }
    ret
}
