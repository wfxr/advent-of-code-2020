mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
#[cfg(test)]
mod testmacros;

use std::error::Error;
use std::time::{Duration, Instant};

struct Solution {
    part1: fn(&str) -> Result<String, Box<dyn Error>>,
    part2: fn(&str) -> Result<String, Box<dyn Error>>,
}

const FAKE_SOLUTION: Solution = Solution {
    part1: |_| unimplemented!(),
    part2: |_| unimplemented!(),
};

const SOLUTIONS: &[Solution] = &[
    day01::SOLUTION,
    day02::SOLUTION,
    day03::SOLUTION,
    day04::SOLUTION,
    day05::SOLUTION,
    day06::SOLUTION,
    day07::SOLUTION,
    day08::SOLUTION,
    day09::SOLUTION,
    day10::SOLUTION,
    day11::SOLUTION,
    day12::SOLUTION,
    day13::SOLUTION,
    day14::SOLUTION,
    day15::SOLUTION,
    day16::SOLUTION,
    day17::SOLUTION,
    day18::SOLUTION,
];

fn measure<T, F>(f: F) -> (Duration, T)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let r = f();
    (Instant::now() - start, r)
}

fn main() -> Result<(), Box<dyn Error>> {
    let day: usize = std::env::args().nth(1).ok_or("missing day number")?.parse()?;

    let Solution { part1, part2 } = SOLUTIONS.get(day - 1).ok_or("day number out of range")?;
    let input = std::fs::read_to_string(format!("src/day{:02}/input", day))?;

    let (t, result) = measure(|| part1(&input));
    println!("part 1: {}, time used: {:?}", result?, t);

    let (t, result) = measure(|| part2(&input));
    println!("part 2: {}, time used {:?}", result?, t);
    Ok(())
}
