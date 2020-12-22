use crate::{err, solution, Result};
use std::cmp::Ordering;
use std::collections::HashSet;

fn two_sum(nums: &[i64], target: i64) -> bool {
    let nums: HashSet<_> = nums.iter().collect();
    nums.iter().any(|&x| nums.contains(&(target - x)))
}

fn contiguous_sum(nums: &[i64], target: i64) -> Option<(i64, i64)> {
    let mut sum = 0;
    let (mut min, mut max) = (i64::MAX, i64::MIN);
    for &x in nums.iter() {
        sum += x;
        min = min.min(x);
        max = max.max(x);
        match sum.cmp(&target) {
            Ordering::Equal => return Some((min, max)),
            Ordering::Greater => return None,
            _ => continue,
        }
    }
    None
}

fn find_target(nums: &[i64]) -> Result<i64> {
    Ok(*nums
        .iter()
        .enumerate()
        .skip(25)
        .find(|(i, &x)| !two_sum(&nums[..*i], x))
        .ok_or("not found")?
        .1)
}

fn parse_input(input: &str) -> Result<Vec<i64>> {
    input
        .lines()
        .map(|line| line.parse::<i64>().map_err(Into::into))
        .collect()
}

fn part1(input: &str) -> Result<i64> {
    find_target(&parse_input(input)?)
}

fn part2(input: &str) -> Result<i64> {
    let input = parse_input(input)?;
    let target = find_target(&input)?;
    for i in 0..input.len() {
        if let Some((min, max)) = contiguous_sum(&input[i..], target) {
            return Ok(max + min);
        }
    }
    err!("not found")
}

solution!(part1 => 1309761972, part2 => 177989832);
