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
mod day19;
mod day20;
mod day21;
// +MODS+

fn get_solution(index: &str) -> Option<Solution> {
    match index {
        "day01" => day01::SOLUTION.into(),
        "day02" => day02::SOLUTION.into(),
        "day03" => day03::SOLUTION.into(),
        "day04" => day04::SOLUTION.into(),
        "day05" => day05::SOLUTION.into(),
        "day06" => day06::SOLUTION.into(),
        "day07" => day07::SOLUTION.into(),
        "day08" => day08::SOLUTION.into(),
        "day09" => day09::SOLUTION.into(),
        "day10" => day10::SOLUTION.into(),
        "day11" => day11::SOLUTION.into(),
        "day12" => day12::SOLUTION.into(),
        "day13" => day13::SOLUTION.into(),
        "day14" => day14::SOLUTION.into(),
        "day15" => day15::SOLUTION.into(),
        "day16" => day16::SOLUTION.into(),
        "day17" => day17::SOLUTION.into(),
        "day18" => day18::SOLUTION.into(),
        "day19" => day19::SOLUTION.into(),
        "day20" => day20::SOLUTION.into(),
        "day21" => day21::SOLUTION.into(),
        _ => None, // +SOLUTIONS+
    }
}

#[cfg(test)]
mod testmacros;

use std::time::{Duration, Instant};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

struct Solution {
    part1: fn(&str) -> Result<String>,
    part2: fn(&str) -> Result<String>,
}

fn main() -> Result<()> {
    let index = std::env::args().nth(1).ok_or("missing solution index")?;

    let Solution { part1, part2 } = get_solution(&index).ok_or("solution not found")?;
    let input = std::fs::read_to_string(format!("src/{}/input", index))?;

    let (t, result) = measure(|| part1(&input));
    println!("part 1: {}, time used: {:?}", result?, t);

    let (t, result) = measure(|| part2(&input));
    println!("part 2: {}, time used {:?}", result?, t);
    Ok(())
}

fn measure<T, F>(f: F) -> (Duration, T)
where
    F: FnOnce() -> T,
{
    let start = Instant::now();
    let r = f();
    (Instant::now() - start, r)
}

#[macro_export]
macro_rules! solution {
    ($part1:ident => $expected1:expr, $part2:ident => $expected2:expr) => {
        pub(super) const SOLUTION: crate::Solution = crate::Solution {
            part1: |input| Ok($part1(input).to_string()),
            part2: |input| Ok($part2(input).to_string()),
        };
        #[cfg(test)]
        mod solution {
            crate::solution_test!(part1 => $expected1);
            crate::solution_test!(part2 => $expected2);
        }
    };
}

#[macro_export]
macro_rules! solution_result {
    ($part1:ident => $expected1:expr, $part2:ident => $expected2:expr) => {
        pub(super) const SOLUTION: crate::Solution = crate::Solution {
            part1: |input| $part1(input).map(|x| x.to_string()),
            part2: |input| $part2(input).map(|x| x.to_string()),
        };
        #[cfg(test)]
        mod solution {
            crate::solution_test!(part1 => $expected1);
            crate::solution_test!(part2 => $expected2);
        }
    };
}
