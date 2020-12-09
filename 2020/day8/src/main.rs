use std::io::{self, BufRead};

fn p1_solve(program: &[(String, i64)], flip: Option<usize>) -> (i64, bool) {
    let mut program: Vec<_> = program
        .iter()
        .enumerate()
        .map(|(i, (ins, arg))| {
            let ins = match flip {
                Some(flip) if i == flip => match ins.as_ref() {
                    "nop" => "jmp",
                    "jmp" => "nop",
                    _ => ins,
                },
                _ => ins,
            };
            Some((ins, arg))
        })
        .collect();
    let (mut acc, mut p) = (0, 0);
    while p < program.len() {
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
            None => return (acc, false), // cannot halt
        };
    }
    (acc, true) // can halt
}

fn p2_solve(program: &[(String, i64)]) -> i64 {
    for (i, _) in program
        .iter()
        .enumerate()
        .filter(|(_, (ins, _))| ins == "jmp" || ins == "nop")
    {
        let (result, halt) = p1_solve(program, Some(i));
        if halt {
            return result;
        }
    }
    unreachable!()
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

    let (result, _) = p1_solve(&inputs, None);
    println!("part1 result: {}", result);
    assert_eq!(1939, result);

    let result = p2_solve(&inputs);
    println!("part2 result: {}", result);
    assert_eq!(2212, result);
}
