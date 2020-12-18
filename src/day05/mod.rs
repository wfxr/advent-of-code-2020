use crate::Solution;

#[rustfmt::skip]
fn seat_id(seat: &str) -> usize {
    seat.chars().rev()
        .fold((0, 1), |(acc, base), c| (acc + base * (c == 'B' || c == 'R') as usize, base * 2))
        .0
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let result = input.lines().map(|seat| seat_id(seat)).max().unwrap();
        Ok(result.to_string())
    },
    part2: |input| {
        let mut seats: Vec<_> = input.lines().map(|seat| seat_id(seat)).collect();
        seats.sort_unstable();
        let result = seats.iter().zip(seats[0]..).find(|&(a, b)| *a != b).unwrap().1;
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(994, 741);
