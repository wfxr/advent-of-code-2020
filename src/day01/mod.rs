use crate::Solution;
use std::collections::HashSet;

fn parse_input(input: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    Ok(input.split_whitespace().map(str::parse).collect::<Result<_, _>>()?)
}

const TARGET: i32 = 2020;

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let mut encountered = HashSet::new();
        parse_input(input)?
            .iter()
            .find(|&x| {
                encountered.insert(x);
                encountered.contains(&(TARGET - x))
            })
            .map(|&x| Ok((x * (TARGET - x)).to_string()))
            .ok_or_else(|| "no result found")?
    },
    part2: |input| {
        let nums = parse_input(input)?;
        let set: HashSet<_> = nums.iter().collect();
        for i in 0..nums.len() - 2 {
            for j in i..nums.len() - 1 {
                let (x, y, z) = (nums[i], nums[j], TARGET - nums[i] - nums[j]);
                if set.contains(&z) && z != x && z != y {
                    return Ok((x * y * z).to_string());
                }
            }
        }
        return Err("no result found")?;
    },
};

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1() {
        let input = include_str!("input");
        let res = (SOLUTION.part1)(&input).unwrap();
        assert_eq!(res, "259716");
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        let res = (SOLUTION.part2)(&input).unwrap();
        assert_eq!(res, "120637440");
    }
}
