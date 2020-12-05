use std::io::{self, BufRead};

#[allow(non_snake_case)]
fn p1_solve(intcode: &[usize], verb: usize, noun: usize) -> usize {
    let mut C = intcode.to_vec();
    C[1] = noun;
    C[2] = verb;

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

fn p2_solve(intcode: &[usize]) -> usize {
    for verb in 0..100 {
        for noun in 0..100 {
            if p1_solve(intcode, verb, noun) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!()
}

#[rustfmt::skip]
fn main() {
    let inputs: Vec<usize> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let result = p1_solve(&inputs, 2, 12);
    println!("part 1 result: {}", result);
    assert_eq!(5305097, result);

    let result = p2_solve(&inputs);
    println!("part 2 result: {}", result);
    assert_eq!(0, result);
}
