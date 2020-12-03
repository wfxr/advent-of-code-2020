use std::io::{self, BufRead};

fn is_valid(min: u32, max: u32, t: char, s: &str) -> bool {
    let mut count = 0;
    for c in s.chars() {
        if c == t {
            count += 1;
        }
        if count > max {
            return false;
        }
    }
    return count >= min;
}

fn main() {
    let reader = io::stdin();
    let result: u32 = reader.lock().lines().map(|l| l.unwrap()).fold(0, |acc, line| {
        let mut parts = line
            .split(|c| c == '-' || c == ' ' || c == ':')
            .filter(|&s| !s.is_empty());
        let min: u32 = parts.next().unwrap().parse().unwrap();
        let max: u32 = parts.next().unwrap().parse().unwrap();
        let t: char = parts.next().unwrap().parse().unwrap();
        let s: &str = parts.next().unwrap();
        match is_valid(min, max, t, s) {
            true => acc + 1,
            false => acc,
        }
    });
    println!("part1 result: {}", result);
}
