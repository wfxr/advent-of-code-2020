#[rustfmt::skip]
fn solve(nums: &[usize], nth: usize) -> usize {
    let mut position = vec![0; nth];
    nums.iter().enumerate().for_each(|(i, &n)| position[n] = i + 1);
    (nums.len()..nth).fold(*nums.last().unwrap(), |prev, i| {
        let t = position[prev];
        position[prev] = i;
        if t == 0 { 0 } else { i - t }
    })
}

fn parse_input(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.trim().parse().unwrap()).collect()
}

fn part1(input: &str) -> usize {
    solve(&parse_input(input), 2020)
}

fn part2(input: &str) -> usize {
    solve(&parse_input(input), 30_000_000)
}

crate::solution!(part1 => 866, part2 => 1437692);
