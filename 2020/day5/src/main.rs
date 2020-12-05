use std::io::{self, BufRead};

#[rustfmt::skip]
fn seat_id(seat: &str) -> usize {
    seat.chars().rev()
        .fold((0, 1), |(acc, base), c| (acc + base * (c == 'B' || c == 'R') as usize, base * 2))
        .0
}

fn p1_solve(seats: &[String]) -> usize {
    seats.iter().map(|l| seat_id(l)).max().unwrap()
}

fn p2_solve(seats: &[String]) -> usize {
    let mut seats: Vec<_> = seats.iter().map(|l| seat_id(l)).collect();
    seats.sort();
    let mut seat = seats[0];
    for &s in &seats {
        if s != seat {
            break;
        }
        seat += 1;
    }
    seat
}

fn main() {
    let inputs: Vec<_> = io::stdin().lock().lines().map(|l| l.unwrap()).collect();

    let result = p1_solve(&inputs);
    println!("part1 result: {}", result);
    assert_eq!(994, result);

    let result = p2_solve(&inputs);
    println!("part2 result: {}", result);
    assert_eq!(741, result);
}
