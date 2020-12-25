use crate::{err, solution, Result};

fn part1(input: &str) -> Result<usize> {
    let nums: Vec<usize> = input.lines().map(|s| Ok(s.parse()?)).collect::<Result<_>>()?;
    let (a, b) = match nums[..] {
        [a, b] => (a.min(b), b.max(a)),
        _ => return err!("require exactly two numbers"),
    };
    let (mut v, mut x) = (1, 1);
    while v != a {
        v = v * 7 % 20201227;
        x = x * b % 20201227;
    }
    Ok(x)
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
