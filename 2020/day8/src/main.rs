use std::io::{self, BufRead};

fn p1_solve(program: &[(String, i64)], init: i64) -> i64 {
    let mut program: Vec<_> = program.iter().map(|(ins, arg)| Some((ins.as_ref(), arg))).collect();
    let (mut acc, mut p) = (init, 0);
    loop {
        match program[p].take() {
            Some((ins, arg)) => match ins {
                "acc" => {
                    acc += arg;
                    p += 1;
                }
                "jmp" => p = (p as i64 + arg) as usize,
                "nop" => p += 1,
                _ => unreachable!(),
            },
            None => break,
        };
    }
    acc
}

fn main() {
    let inputs: Vec<(String, i64)> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|line| {
            let mut it = line.split(' ');
            match (it.next(), it.next()) {
                (Some(ins), Some(arg)) => (ins.to_owned(), arg.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect();

    let result = p1_solve(&inputs, 0);
    println!("part1 result: {}", result);
    assert_eq!(0, result);
}
