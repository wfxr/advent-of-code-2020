use std::collections::HashSet;
use std::io::{self, BufRead};

fn two_sum(nums: &[i32], target: i32) -> (i32, i32) {
    let nums: HashSet<i32> = nums.iter().cloned().collect();
    for &x in nums.iter() {
        let y = target - x;
        if nums.contains(&y) {
            return (x, y);
        }
    }
    unreachable!()
}

fn main() {
    let reader = io::stdin();
    let nums: Vec<i32> = reader.lock().lines().map(|s| s.unwrap().parse().unwrap()).collect();
    let (x, y) = two_sum(&nums, 2020);
    println!("answer: {} (by {} x {})", x * y, x, y);
}
