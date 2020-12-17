use crate::Solution;

fn run_once(program: &[(String, i64)], flip: Option<usize>) -> (i64, bool) {
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

fn parse_input(input: &str) -> Vec<(String, i64)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(' ');
            match (it.next(), it.next()) {
                (Some(ins), Some(arg)) => (ins.to_owned(), arg.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let input = parse_input(input);
        let (result, ..) = run_once(&input, None);
        Ok(result.to_string())
    },
    part2: |input| {
        let input = parse_input(input);
        let result = input
            .iter()
            .enumerate()
            .filter(|(_, (ins, _))| ins == "jmp" || ins == "nop")
            .find_map(|(i, ..)| match run_once(&input, Some(i)) {
                (result, true) => Some(result),
                _ => None,
            })
            .unwrap();
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(1939, 2212);
