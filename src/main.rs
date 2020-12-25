use aoc2020::{build_solutions, Result, Solution};

fn main() {
    if let Err(e) = || -> Result<_> {
        let index = std::env::args().nth(1).ok_or("missing solution index")?;

        let solutions = build_solutions();
        let Solution { part1, part2 } = solutions.get(&index).ok_or("solution not found")?;
        let input = std::fs::read_to_string(format!("src/{}/input", index))?;

        let (t, result) = measure(|| part1(&input));
        println!("part 1: {} - time: {:?}", result?, t);

        let (t, result) = measure(|| part2(&input));
        println!("part 2: {} - time: {:?}", result?, t);
        Ok(())
    }() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

use std::time::{Duration, Instant};

fn measure<T, F>(f: F) -> (Duration, T)
where
    F: FnOnce() -> T,
{
    let now = Instant::now();
    let r = f();
    (now.elapsed(), r)
}
