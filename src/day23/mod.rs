use crate::{solution, Result};

fn solve(cups: &mut [u32], moves: usize) {
    for _ in 0..moves {
        let a0 = cups[0];
        let a1 = cups[a0 as usize];
        let a2 = cups[a1 as usize];
        let a3 = cups[a2 as usize];
        let a4 = cups[a3 as usize];
        let mut t = a0;
        while t == a0 || t == a1 || t == a2 || t == a3 {
            t = if t > 1 { t - 1 } else { t + cups.len() as u32 - 2 }
        }
        cups[a0 as usize] = a4;
        cups[a3 as usize] = cups[t as usize];
        cups[t as usize] = a1;
        cups[0] = a4;
    }
}

fn parse_input(input: &str, cups: &mut [u32]) {
    let input: Vec<_> = input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .chain(10..cups.len() as u32)
        .collect();
    cups[0] = input[0];
    for i in 0..input.len() {
        cups[input[i] as usize] = input[(i + 1) % input.len()];
    }
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &str) -> Result<usize> {
    let mut cups = vec![0; 10];
    parse_input(input, &mut cups);
    solve(&mut cups, 100);
    let mut sum = 0usize;
    let mut cup = 1u32;
    while cups[cup as usize] != 1 {
        sum = sum * 10 + cups[cup as usize] as usize;
        cup = cups[cup as usize];
    }
    Ok(sum)
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &str) -> Result<usize> {
    let mut cups = vec![0; 1_000_001];
    parse_input(input, &mut cups);
    solve(&mut cups, 10_000_000);
    Ok(cups[1] as usize * cups[cups[1] as usize] as usize)
}

solution!(part1 => 49725386, part2 => 538935646702);

#[cfg(test)]
mod examples {
    use crate::test;
    test!(part1, example1: "389125467" => 67384529);
    test!(part2, example1: "389125467" => 149245887792);
}
