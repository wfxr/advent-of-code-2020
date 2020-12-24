use crate::{solution, Result};

#[rustfmt::skip]
fn solve(nums: &[u32], nth: u32) -> Result<u32> {
    let mut position = vec![0u32; nth as usize];
    nums.iter().enumerate().for_each(|(i, &n)| position[n as usize] = i as u32 + 1);
    let res = (nums.len() as u32..nth).fold(*nums.last().ok_or("input is empty")?, |prev, i| {
        let t = position[prev as usize];
        position[prev as usize] = i;
        if t == 0 { 0 } else { i - t }
    });
    Ok(res)
}

fn parse_input(input: &str) -> Result<Vec<u32>> {
    input.split(',').map(|s| Ok(s.trim().parse()?)).collect()
}

fn part1(input: &str) -> Result<u32> {
    solve(&parse_input(input)?, 2020)
}

fn part2(input: &str) -> Result<u32> {
    solve(&parse_input(input)?, 30_000_000)
}

solution!(part1 => 866, part2 => 1437692);
