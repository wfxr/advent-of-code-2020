mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day16;
mod day17;
#[cfg(test)]
mod testmacros;

use std::error::Error;

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
    FAKE_SOLUTION,
    FAKE_SOLUTION,
    FAKE_SOLUTION,
    FAKE_SOLUTION,
    FAKE_SOLUTION,
    FAKE_SOLUTION,
    FAKE_SOLUTION,
    FAKE_SOLUTION,
    FAKE_SOLUTION,
    FAKE_SOLUTION,
    day16::SOLUTION,
    day17::SOLUTION,
];

fn main() -> Result<(), Box<dyn Error>> {
    let day: usize = std::env::args().skip(1).next().ok_or("missing day number")?.parse()?;

    let Solution { part1, part2 } = SOLUTIONS.get(day - 1).ok_or("day number out of range")?;
    let input = std::fs::read_to_string(format!("src/day{:02}/input", day))?;

    println!("part 1: {}", part1(&input)?);
    println!("part 2: {}", part2(&input)?);
    Ok(())
}
