use crate::{solution, Result};
use std::iter;

fn init_input(input: &str) -> Result<Vec<usize>> {
    let mut nums: Vec<usize> = iter::once(Ok(0))
        .chain(input.lines().map(|line| Ok(line.parse()?)))
        .collect::<Result<_>>()?;
    nums.sort_unstable();
    Ok(nums)
}

fn part1(input: &str) -> Result<usize> {
    let nums = init_input(input)?;

    let mut delta = [0, 0, 0, 1];
    nums.iter()
        .zip(nums.iter().skip(1))
        .for_each(|(curr, next)| delta[next - curr] += 1);
    Ok(delta[1] * delta[3])
}

fn part2(input: &str) -> Result<usize> {
    let nums = init_input(input)?;
    fn group_solve(group: &[usize]) -> usize {
        match group.len() {
            0 | 1 => 1,
            2 => (group[1] - group[0] <= 3) as usize,
            _ => {
                let mut sgroup: Vec<_> = group[1..].to_vec();
                sgroup[0] = group[0];
                group_solve(&group[1..]) + group_solve(&sgroup)
            }
        }
    }
    Ok(nums
        .iter()
        .zip(nums.iter().skip(1))
        .enumerate()
        .fold((1, 0), |(mut acc, mut pos), (i, (curr, next))| {
            if next - curr == 3 {
                acc *= group_solve(&nums[pos..i + 1]);
                pos = i + 1;
            }
            (acc, pos)
        })
        .0)
}

solution!(part1 => 2484, part2 => 15790581481472);
