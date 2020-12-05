use std::io::{self, BufRead};

#[allow(non_snake_case)]
fn p1_solve(intcode: &[usize]) -> usize {
    let mut C = intcode.to_vec();
    C[1] = 12;
    C[2] = 2;

    let mut p = 0;
    while p < C.len() {
        match (C[p], C[p + 1], C[p + 2], C[p + 3]) {
            (1, a, b, r) => C[r] = C[a] + C[b],
            (2, a, b, r) => C[r] = C[a] * C[b],
            _ => break,
        }
        p += 4;
    }
    C[0]
}

#[rustfmt::skip]
fn main() {
    let inputs: Vec<usize> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let result = p1_solve(&inputs);
    println!("part 1 result: {}", result);
    assert_eq!(10, result);
}
