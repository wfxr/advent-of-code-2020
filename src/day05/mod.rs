use crate::{solution, Result};

fn seat_id(seat: &str) -> usize {
    seat.chars()
        .rev()
        .fold((0, 1), |(acc, base), c| {
            (acc + base * (c == 'B' || c == 'R') as usize, base * 2)
        })
        .0
}

fn part1(input: &str) -> Result<usize> {
    input.lines().map(seat_id).max().ok_or_else(|| "empty".into())
}

fn part2(input: &str) -> Result<usize> {
    let mut seats: Vec<_> = input.lines().map(seat_id).collect();
    seats.sort_unstable();
    seats
        .iter()
        .zip(seats[0]..)
        .find(|&(a, b)| *a != b)
        .map(|(_, b)| b)
        .ok_or_else(|| "not found".into())
}

solution!(part1 => 994, part2 => 741);
