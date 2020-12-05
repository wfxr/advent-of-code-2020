use std::io::{self, BufRead};

fn p1_solve(modules: &[i64]) -> i64 {
    modules.iter().fold(0, |acc, m| m / 3 - 2 + acc)
}

#[rustfmt::skip]
fn p2_solve(modules: &[i64]) -> i64 {
    modules.iter().fold(0, |acc, &m| {
        let (mut m, mut total) = (m, 0);
        loop {
            m = m / 3 - 2;
            if m < 0 { break }
            total += m;
        }
        acc + total
    })
}

#[rustfmt::skip]
fn main() {
    let inputs: Vec<i64> = io::stdin().lock().lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let result = p1_solve(&inputs);
    println!("part1 result: {}", result);
    assert_eq!(3455717, result);

    let result = p2_solve(&inputs);
    println!("part2 result: {}", result);
    assert_eq!(5180690, result);
}
