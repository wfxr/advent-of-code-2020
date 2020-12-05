use std::io::{self, BufRead};

fn seat_id(seat: &str) -> usize {
    seat.chars()
        .rev()
        .fold((0, 1), |(acc, base), c| {
            (acc + base * (c == 'B' || c == 'R') as usize, base * 2)
        })
        .0
}

fn main() {
    let inputs: Vec<_> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let result = inputs.iter().map(|l| seat_id(l)).max().unwrap();
    println!("part1 result: {}", result);
    assert_eq!(994, result);
}
