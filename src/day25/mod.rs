use crate::{err, solution, Result};

fn part1(input: &str) -> Result<usize> {
    let nums: Vec<usize> = input.lines().map(|s| Ok(s.parse()?)).collect::<Result<_>>()?;
    let (a, b) = match nums[..] {
        [a, b] => (a, b),
        _ => return err!("need exactly two numbers as input"),
    };
    let (mut x, mut loops) = (1, 0);
    while x != a && x != b {
        x = x * 7 % 20201227;
        loops += 1;
    }
    let x = if x == a { b } else { a };
    Ok((0..loops).fold(1, |key, _| key * x % 20201227))
}

#[allow(clippy::unnecessary_wraps)]
fn part2(_input: &str) -> Result<&str> {
    Ok("🎄")
}

solution!(part1 => 19924389, part2 => "🎄");

#[cfg(test)]
mod examples {
    crate::test!(part1, example1: "5764801\n17807724" => 14897079);
}
