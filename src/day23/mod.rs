use crate::{solution, Result};

fn solve(cups: &mut Vec<usize>, moves: usize) {
    for _ in 0..moves {
        let a0 = cups[0];
        let a1 = cups[a0];
        let a2 = cups[a1];
        let a3 = cups[a2];
        let mut t = a0;
        while t == a0 || t == a1 || t == a2 || t == a3 {
            t = if t > 1 { t - 1 } else { t + cups.len() - 2 }
        }
        cups[a0] = cups[a3];
        cups[0] = cups[a3];
        let tnext = cups[t];
        cups[t] = a1;
        cups[a3] = tnext;
    }
}

fn parse_input(input: &str, total_cups: Option<usize>) -> Vec<usize> {
    let input: Vec<_> = input.trim().chars().map(|c| c as usize - '0' as usize).collect();
    let total_cups = if let Some(n) = total_cups { n } else { input.len() };
    let mut cups = vec![0; total_cups + 1];
    let mut x = 0;
    for &n in &input {
        cups[x] = n;
        x = n;
    }
    cups[input[input.len() - 1]] = if input.len() == total_cups {
        input[0]
    } else {
        input.len() + 1
    };
    (input.len() + 1..cups.len()).for_each(|x| {
        cups[x] = x + 1;
    });
    if total_cups > input.len() {
        cups[total_cups] = input[0];
    }
    cups
}

#[allow(clippy::unnecessary_wraps)]
fn part1(input: &str) -> Result<usize> {
    let mut cups = parse_input(input, None);
    solve(&mut cups, 100);
    let mut sum = 0;
    let mut cup = 1;
    while cups[cup] != 1 {
        sum = sum * 10 + cups[cup];
        cup = cups[cup];
    }
    Ok(sum)
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &str) -> Result<usize> {
    let mut cups = parse_input(input, Some(1_000_000));
    solve(&mut cups, 10_000_000);
    Ok(cups[1] * cups[cups[1]])
}

solution!(part1 => 49725386, part2 => 538935646702);

#[cfg(test)]
mod examples {
    use crate::test;
    test!(part1, example1: "389125467" => 67384529);
    test!(part2, example1: "389125467" => 149245887792);
}
