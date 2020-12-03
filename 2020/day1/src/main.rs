use std::collections::HashSet;
use std::io::{self, BufRead};

fn two_sum(nums: &[i32], target: i32) -> Result<(i32, i32), ()> {
    let nums: HashSet<i32> = nums.iter().cloned().collect();
    for &x in nums.iter() {
        let y = target - x;
        if nums.contains(&y) {
            return Ok((x, y));
        }
    }
    Err(())
}

// Simple solution
fn three_sum(nums: &[i32], target: i32) -> Result<(i32, i32, i32), ()> {
    for (i, &z) in nums.iter().enumerate().take(nums.len() - 2) {
        let target = target - z;
        if let Ok((x, y)) = two_sum(&nums[(i + 1)..], target) {
            return Ok((x, y, z));
        }
    }
    Err(())
}

fn main() -> Result<(), ()> {
    let reader = io::stdin();
    let nums: Vec<i32> = reader.lock().lines().map(|s| s.unwrap().parse().unwrap()).collect();

    let (x, y) = two_sum(&nums, 2020)?;
    println!("part 1 answer: {} (by {} x {})", x * y, x, y);

    let (x, y, z) = three_sum(&nums, 2020)?;
    println!("part 2 answer: {} (by {} x {} x {})", x * y * z, x, y, z);

    Ok(())
}
