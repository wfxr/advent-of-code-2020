use std::cmp::Ordering;
use std::collections::HashSet;
use std::io::{self, BufRead};

fn two_sum(nums: &[i64], target: i64) -> bool {
    let nums: HashSet<_> = nums.iter().collect();
    nums.iter().any(|&x| nums.contains(&(target - x)))
}

fn p1_solve(nums: &[i64]) -> i64 {
    *nums
        .iter()
        .enumerate()
        .skip(25)
        .find(|(i, &x)| !two_sum(&nums[..*i], x))
        .unwrap()
        .1
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
            _ => {}
        }
    }
    None
}

fn p2_solve(nums: &[i64]) -> i64 {
    let target = p1_solve(nums);
    for i in 0..nums.len() {
        if let Some((min, max)) = contiguous_sum(&nums[i..], target) {
            return max + min;
        }
    }
    unreachable!()
}

fn main() {
    let inputs: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let result = p1_solve(&inputs);
    println!("part1 result : {}", result);
    assert_eq!(1309761972, result);

    let result = p2_solve(&inputs);
    println!("part2 result : {}", result);
    assert_eq!(0, result);
}
