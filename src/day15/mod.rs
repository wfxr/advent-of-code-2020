use crate::{solution, Result};

#[rustfmt::skip]
fn solve(nums: &[usize], nth: usize) -> Result<usize> {
    let mut position = vec![0; nth];
    nums.iter().enumerate().for_each(|(i, &n)| position[n] = i + 1);
    Ok(
        (nums.len()..nth).fold(*nums.last().ok_or("input is empty")?, |prev, i| {
            let t = position[prev];
            position[prev] = i;
            if t == 0 { 0 } else { i - t }
        })
    )
}

fn parse_input(input: &str) -> Result<Vec<usize>> {
    input
        .split(',')
        .map(|s| s.trim().parse::<usize>().map_err(|e| e.into()))
        .collect()
}

fn part1(input: &str) -> Result<usize> {
    solve(&parse_input(input)?, 2020)
}

fn part2(input: &str) -> Result<usize> {
    solve(&parse_input(input)?, 30_000_000)
}

solution!(part1 => 866, part2 => 1437692);
