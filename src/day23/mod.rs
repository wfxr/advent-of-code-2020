use crate::{solution, Result};
use std::collections::VecDeque;

fn solve(cups: &mut VecDeque<usize>, count: usize) -> Result<&VecDeque<usize>> {
    let len = cups.len();
    for _ in 0..count {
        let mut heads: VecDeque<_> = cups.drain(0..4).collect();
        let head = heads.pop_front().unwrap();
        let pos = (1..=len)
            .rev()
            .cycle()
            .skip(len - head + 1)
            .take(cups.len() - 1)
            .find_map(|target| cups.iter().position(|&x| x == target))
            .ok_or("no place to move")?;
        let body: Vec<_> = cups.drain(0..=pos).rev().collect();
        heads.into_iter().rev().for_each(|x| cups.push_front(x));
        body.into_iter().for_each(|x| cups.push_front(x));
        cups.push_back(head);
    }
    Ok(cups)
}

fn part1(input: &str) -> Result<usize> {
    let mut cups: VecDeque<_> = input.trim().chars().map(|c| c as usize - '0' as usize).collect();
    solve(&mut cups, 100)?;
    Ok(cups
        .iter()
        .cycle()
        .skip_while(|&&x| x != 1)
        .skip(1)
        .take(cups.len() - 1)
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
