use crate::{solution, Result};
use std::collections::HashSet;

fn parse_input(input: &str) -> Result<Vec<i32>> {
    input.split_whitespace().map(|s| Ok(s.parse()?)).collect()
}

const TARGET: i32 = 2020;

fn part1(input: &str) -> Result<i32> {
    let mut encountered = HashSet::new();
    parse_input(input)?
        .iter()
        .find(|&x| {
            encountered.insert(x);
            encountered.contains(&(TARGET - x))
        })
        .map(|&x| (x * (TARGET - x)))
        .ok_or_else(|| "not found".into())
}

fn part2(input: &str) -> Result<i32> {
    let nums = parse_input(input)?;
    let set: HashSet<_> = nums.iter().collect();
    for i in 0..nums.len() - 2 {
        for j in i..nums.len() - 1 {
            let (x, y, z) = (nums[i], nums[j], TARGET - nums[i] - nums[j]);
            if set.contains(&z) && z != x && z != y {
                return Ok(x * y * z);
            }
        }
    }
    Err("not found".into())
}

solution!(part1 => 259716, part2 => 120637440);
