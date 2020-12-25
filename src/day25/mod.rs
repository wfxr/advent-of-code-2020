use crate::{err, solution, Result};

fn next(x: usize, subject: usize) -> usize {
    (x * subject) % 20201227
}

fn encryption(subject: usize, n: usize) -> usize {
    let mut x = 1;
    for _ in 0..n {
        x = next(x, subject)
    }
    x
}

fn calc_loops(pubkey: usize) -> usize {
    let (mut x, mut n) = (1, 0);
    loop {
        n += 1;
        x = next(x, 7);
        if x == pubkey {
            return n;
        }
    }
}

fn part1(input: &str) -> Result<usize> {
    let nums: Vec<usize> = input.lines().map(|s| Ok(s.parse()?)).collect::<Result<_>>()?;
    if nums.len() != 2 {
        return err!("need exactly two numbers");
    }
    Ok(encryption(nums[1], calc_loops(nums[0])))
}

fn part2(input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 0, part2 => 0);

#[cfg(test)]
mod examples {
    crate::test!(part1, example1: "5764801\n17807724" => 14897079);
}
