use crate::{solution, Result};
use std::collections::VecDeque;

fn solve1(cups: &mut VecDeque<usize>, moves: usize) -> Result<()> {
    let len = cups.len();
    for _ in 0..moves {
        let mut heads: VecDeque<_> = cups.drain(0..4).collect();
        let mut target = heads[0];
        while heads.contains(&target) {
            target = if target > 1 { target - 1 } else { target + len - 1 }
        }
        let head = heads.pop_front().unwrap(); // no panic
        let pos = cups.iter().position(|&x| x == target).ok_or("target not found")?;
        let body: Vec<_> = cups.drain(0..=pos).rev().collect();
        heads.into_iter().rev().for_each(|x| cups.push_front(x));
        body.into_iter().for_each(|x| cups.push_front(x));
        cups.push_back(head);
    }
    Ok(())
}

fn solve2(cups: &mut Vec<usize>, moves: usize) -> usize {
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
    cups[1] * cups[cups[1]]
}

fn part1(input: &str) -> Result<usize> {
    let mut cups: VecDeque<_> = input.trim().chars().map(|c| c as usize - '0' as usize).collect();
    solve1(&mut cups, 100)?;
    Ok(cups
        .iter()
        .cycle()
        .skip_while(|&&x| x != 1)
        .skip(1)
        .take(cups.len() - 1)
        .fold(0, |acc, &x| acc * 10 + x as usize))
}

#[allow(clippy::unnecessary_wraps)]
fn part2(input: &str) -> Result<usize> {
    const CUPS: usize = 1_000_000;
    const MOVES: usize = 10_000_000;
    let mut cups = vec![0; CUPS + 1];
    let input: Vec<_> = input.trim().chars().map(|c| c as usize - '0' as usize).collect();
    let mut x = 0;
    for &n in &input {
        cups[x] = n;
        x = n;
    }
    cups[input[input.len() - 1]] = if input.len() == CUPS { input[0] } else { input.len() + 1 };
    (input.len() + 1..cups.len()).for_each(|x| {
        cups[x] = x + 1;
    });
    if CUPS > input.len() {
        cups[CUPS] = input[0];
    }
    Ok(solve2(&mut cups, MOVES))
}

solution!(part1 => 49725386, part2 => 538935646702);

#[cfg(test)]
mod examples {
    use crate::test;
    test!(part1, example1: "389125467" => 67384529);
    test!(part2, example1: "389125467" => 149245887792);
}
