use crate::Solution;
use std::iter;

fn init_input(input: &str) -> Vec<usize> {
    let mut nums: Vec<usize> = iter::once(0)
        .chain(input.lines().map(|line| line.parse().unwrap()))
        .collect();
    nums.sort_unstable();
    nums
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let nums = init_input(input);

        let mut delta = [0, 0, 0, 1];
        nums.iter()
            .zip(nums.iter().skip(1))
            .for_each(|(curr, next)| delta[next - curr] += 1);
        let result = delta[1] * delta[3];
        Ok(result.to_string())
    },
    part2: |input| {
        let nums = init_input(input);
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
        let result =
            nums.iter()
                .zip(nums.iter().skip(1))
                .enumerate()
                .fold((1, 0), |(mut acc, mut pos), (i, (curr, next))| {
                    if next - curr == 3 {
                        acc *= group_solve(&nums[pos..i + 1]);
                        pos = i + 1;
                    }
                    (acc, pos)
                });
        Ok(result.0.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(2484, 15790581481472);
