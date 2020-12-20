use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<i32> {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

const TARGET: i32 = 2020;

fn part1(input: &str) -> i32 {
    let mut encountered = HashSet::new();
    parse_input(input)
        .iter()
        .find(|&x| {
            encountered.insert(x);
            encountered.contains(&(TARGET - x))
        })
        .map(|&x| (x * (TARGET - x)))
        .unwrap()
}

fn part2(input: &str) -> i32 {
    let nums = parse_input(input);
    let set: HashSet<_> = nums.iter().collect();
    for i in 0..nums.len() - 2 {
        for j in i..nums.len() - 1 {
            let (x, y, z) = (nums[i], nums[j], TARGET - nums[i] - nums[j]);
            if set.contains(&z) && z != x && z != y {
                return x * y * z;
            }
        }
    }
    unreachable!()
}

crate::solution!(part1 => 259716, part2 => 120637440);
