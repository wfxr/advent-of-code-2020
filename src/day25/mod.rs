use crate::{err, solution, Result};

// NOTE: The performance is highly related to the input. For my input it takes about 50ms.
// But for some other inputs like [13316116, 13651422] it only takes 2ms.
fn part1(input: &str) -> Result<usize> {
    let nums: Vec<usize> = input.lines().map(|s| Ok(s.parse()?)).collect::<Result<_>>()?;
    let (a, b) = match nums[..] {
        [a, b] => (a, b),
        _ => return err!("require exactly two numbers"),
    };
    let (mut v, mut x) = (1, 1);
    // Theoretically, the loop can be ended when v == a or v == b.
    // But we cannot use the same loop to calculate the final key if we use this optimization.
    // The test results prove that using a single loop is much faster.
    // Again this should be related to the input.
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
