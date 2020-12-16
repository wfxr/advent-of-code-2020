mod day01;
mod day02;

use std::error::Error;

struct Solution {
    part1: fn(&str) -> Result<String, Box<dyn Error>>,
    part2: fn(&str) -> Result<String, Box<dyn Error>>,
}

const SOLUTIONS: &[Solution] = &[day01::SOLUTION, day02::SOLUTION];

fn main() -> Result<(), Box<dyn Error>> {
    let day: usize = std::env::args().skip(1).next().ok_or("missing day number")?.parse()?;

    let Solution { part1, part2 } = SOLUTIONS.get(day - 1).ok_or("day number out of range")?;
    let input = std::fs::read_to_string(format!("src/day{:02}/input", day))?;

    println!("part 1: {}", part1(&input)?);
    println!("part 2: {}", part2(&input)?);
    Ok(())
}
