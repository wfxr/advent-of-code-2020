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

fn main() {
    let inputs: Vec<usize> = include_str!("../inputs.txt")
        .split(',')
        .map(|s| s.trim().parse().unwrap())
        .collect();

    let result = solve(&inputs, 2020);
    println!("part 1 result: {}", result);
    assert_eq!(866, result);

    let result = solve(&inputs, 30_000_000);
    println!("part 2 result: {}", result);
    assert_eq!(1437692, result);
}
