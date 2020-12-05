use std::io::{self, BufRead};

fn p1_solve(modules: &[usize]) -> usize {
    modules.iter().fold(0, |acc, m| m / 3 - 2 + acc)
}

fn main() {
    let inputs: Vec<usize> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let result = p1_solve(&inputs);
    println!("part1 result: {}", result);
    assert_eq!(3455717, result);
}
