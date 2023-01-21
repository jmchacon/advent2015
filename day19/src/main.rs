//! day19 advent 2022
use clap::Parser;
use color_eyre::eyre::Result;
//use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

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
        transforms
            .entry(parts[0])
            .and_modify(|v: &mut Vec<&str>| v.push(parts[2]))
            .or_insert(vec![parts[2]]);
    }

    println!("start: {goal}");
    for (k, v) in &transforms {
        println!("{k} -> {v:?}");
    }

    let mut new = HashSet::new();
    for k in transforms.keys() {
        for trans in &transforms[k] {
            if args.debug {
                println!("Transforming {goal} {k} with {trans}");
            }
            new = new
                .union(&do_transform(goal, k, trans, args.debug))
                .cloned()
                .collect();
        }
    }
    println!("Count is {} for {goal}", new.len());

    println!();
    // Part2 - implement Dijkstra's algorithm to find the least number of
    //         transforms to hit the goal.
    //let steps = bfs(goal, "e", &transforms, args.debug);
    //let steps = dfs_astar(goal, "e", &transforms, 0);
    let ar = goal.matches("Ar").count();
    let rn = goal.matches("Rn").count();
    let y = goal.matches("Y").count();
    let upper = goal.bytes().filter(|x| *x >= b'A' && *x <= b'Z').count();
    println!(
        "{} {ar} {rn} {y} Took {} steps to get from e -> {goal}",
        upper,
        upper - rn - ar - 2 * y - 1
    );
    let steps = dfs_astar(goal, "e", &transforms, 0);
    println!("Steps are {steps}");
    Ok(())
}

fn dfs_astar(cur: &str, m: &str, transforms: &HashMap<&str, Vec<&str>>, cost: usize) -> usize {
    //println!("{cur} =-> {m}");
    if cur == m {
        return cost;
    }

    let mut trs = transforms
        .iter()
        .map(|(k, v)| v.iter().map(|vv| (*vv, *k)).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    trs.sort_by(|a, b| a.0.len().partial_cmp(&b.0.len()).unwrap());

    let mut new = HashSet::new();
    for (k, v) in trs.iter().rev() {
        /*let res = do_all_transforms(cur, k, v, false);
        if res.1 > 0 {
            new.insert(res);
        }*/
        new = new
            .union(&do_transform(cur, k, v, false))
            .cloned()
            .collect();
    }
    let mut candidates = new.iter().map(|x| (x.len(), x)).collect::<Vec<_>>();
    candidates.sort();

    println!("{} candidates -< {candidates:?}", candidates.len());
    for k in &candidates {
        let ret = dfs_astar(&k.1, m, transforms, cost + 1);
        if ret != usize::MAX {
            return ret;
        }
        //println!("dead end");
    }
    usize::MAX
}

fn do_all_transforms(start: &str, k: &str, trans: &str, debug: bool) -> (String, usize) {
    let mut t = start.as_bytes().iter().cloned().collect::<Vec<_>>();
    let mut cost = 0;

    loop {
        let p = t.clone();
        let Some(i) = std::str::from_utf8(&p).unwrap().match_indices(k).next() else {
            break;
        };
        cost += 1;
        if debug {
            //    println!("reducing {start} with {i:?}");
        }
        for _ in 0..i.1.len() {
            t.remove(i.0);
            if debug {
                //    println!("{} now after remove", std::str::from_utf8(&t).unwrap());
            }
        }
        for byte in trans.bytes().rev() {
            t.insert(i.0, byte);
            if debug {
                //    println!("{} insert", std::str::from_utf8(&t).unwrap());
            }
        }
    }
    (String::from_utf8(t).unwrap(), cost)
}

fn do_transform(start: &str, k: &str, trans: &str, debug: bool) -> HashSet<String> {
    //println!("Calling with {start} {k} {trans}");
    let mut ret = HashSet::new();
    for i in 0..start.len() - k.len() + 1 {
        if start[i..i + k.len()] == *k {
            let mut t = start.as_bytes().iter().cloned().collect::<Vec<_>>();
            if debug {
                println!("Fixing {}", std::str::from_utf8(&t).unwrap());
            }
            for _ in i..i + k.len() {
                t.remove(i);
                if debug {
                    println!("{} now", std::str::from_utf8(&t).unwrap());
                }
            }
            for byte in trans.bytes().rev() {
                t.insert(i, byte);
                if debug {
                    println!("{} insert", std::str::from_utf8(&t).unwrap());
                }
            }

            let x = String::from_utf8(t).unwrap();
            if debug {
                println!("Found {x} at {}", i);
            }
            ret.insert(x);
        }
    }
    return ret;
    /*for i in start.match_indices(k) {
        let mut t = start.as_bytes().iter().cloned().collect::<Vec<_>>();
        if debug {
            println!("Fixing {}", std::str::from_utf8(&t).unwrap());
        }
        for _ in 0..i.1.len() {
            t.remove(i.0);
            if debug {
                println!("{} now", std::str::from_utf8(&t).unwrap());
            }
        }
        for byte in trans.bytes().rev() {
            t.insert(i.0, byte);
            if debug {
                println!("{} insert", std::str::from_utf8(&t).unwrap());
            }
        }

        let x = String::from_utf8(t).unwrap();
        if debug {
            println!("Found {x} at {}", i.0);
        }
        ret.insert(x);
    }
    ret*/
}
