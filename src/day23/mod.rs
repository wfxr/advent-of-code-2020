use crate::{solution, Result};
use std::collections::VecDeque;

fn solve1(cups: &mut VecDeque<u8>) {
    let (curr, a, b, c) = (
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
    );
    assert!(curr > 0);
    let pos = (1..=6)
        .map(|x| if curr >= x { curr - x } else { curr + 10 - x })
        .find_map(|target| cups.iter().position(|&x| x == target))
        .unwrap();
    let tmp: Vec<_> = cups.drain(0..=pos).rev().collect();
    cups.push_front(c);
    cups.push_front(b);
    cups.push_front(a);
    tmp.into_iter().for_each(|x| cups.push_front(x));
    cups.push_back(curr);
}

fn part1(input: &str) -> Result<usize> {
    let mut input: VecDeque<_> = input.trim().chars().map(|c| c as u8 - '0' as u8).collect();
    for _ in 0..100 {
        solve1(&mut input);
    }
    Ok(input
        .iter()
        .cycle()
        .skip_while(|&&x| x != 1)
        .skip(1)
        .take(input.len() - 1)
        .fold(0, |acc, &x| acc * 10 + x as usize))
}

fn part2(input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 49725386, part2 => 0);

#[cfg(test)]
mod examples {
    use crate::test;
    test!(part1, example1: "389125467" => 67384529);
}
