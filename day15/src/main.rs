use std::io::{self, BufRead};

fn p1_solve(nums: &[usize], nth: usize) -> usize {
    let mut position = vec![(0, 0); nth];
    nums.iter().enumerate().for_each(|(i, &n)| position[n] = (0, i + 1));
    let mut prev = *nums.last().unwrap();
    for i in nums.len()..nth {
        prev = match position[prev] {
            (0, _) => 0,
            (p1, p2) => p2 - p1,
        };
        position[prev] = (position[prev].1, i + 1);
    }
    prev
}

fn main() {
    let inputs: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .flat_map(|l| l.unwrap().split(',').map(|s| s.parse().unwrap()).collect::<Vec<_>>())
        .collect();

    let result = p1_solve(&inputs, 2020);
    println!("part 1 result: {}", result);
    assert_eq!(866, result);
}
