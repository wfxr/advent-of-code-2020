#[rustfmt::skip]
fn seat_id(seat: &str) -> usize {
    seat.chars().rev()
        .fold((0, 1), |(acc, base), c| (acc + base * (c == 'B' || c == 'R') as usize, base * 2))
        .0
}

fn part1(input: &str) -> usize {
    input.lines().map(|seat| seat_id(seat)).max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut seats: Vec<_> = input.lines().map(|seat| seat_id(seat)).collect();
    seats.sort_unstable();
    seats.iter().zip(seats[0]..).find(|&(a, b)| *a != b).unwrap().1
}

crate::solution!(part1 => 994, part2 => 741);
