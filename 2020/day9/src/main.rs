use std::io::{self, BufRead};
use std::collections::HashSet;

fn valid(nums: &[i64], target: i64) -> bool {
    let nums : HashSet<_> = nums.iter().collect();
    nums.iter().any(|&x| {
        nums.contains(&(target - x))
    })
}

fn p1_solve(nums: &[i64]) -> i64 {
    *nums.iter().enumerate().skip(25).find(|(i, &x)| {
        !valid(&nums[..*i], x)
    }).unwrap().1
}
fn main() {
    let inputs : Vec<i64> = io::stdin().lock().lines().map(|l| {
        l.unwrap().parse().unwrap()
    }).collect();

    let result = p1_solve(&inputs);
    println!("part1 result : {}", result);
    assert_eq!(1309761972, result);
}
