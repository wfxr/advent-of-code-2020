use crate::{err, solution, Result};

fn run_once(program: &[(&str, i64)], flip: Option<usize>) -> Result<(i64, bool)> {
    let mut program: Vec<_> = program
        .iter()
        .enumerate()
        .map(|(i, &(ins, arg))| {
            let ins = match flip {
                Some(flip) if i == flip => match ins {
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
                s => return err!("unknown ins: {}", s),
            },
            None => return Ok((acc, false)), // cannot halt
        };
    }
    Ok((acc, true)) // can halt
}

fn parse_input(input: &str) -> Result<Vec<(&str, i64)>> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split(' ');
            Ok((
                it.next().ok_or("missing ins")?,
                it.next().ok_or("missing arg")?.parse::<i64>()?,
            ))
        })
        .collect()
}

fn part1(input: &str) -> Result<i64> {
    Ok(run_once(&parse_input(input)?, None)?.0)
}

fn part2(input: &str) -> Result<i64> {
    let input = parse_input(input)?;
    input
        .iter()
        .enumerate()
        .filter(|(_, &(ins, _))| ins == "jmp" || ins == "nop")
        .map(|(i, ..)| run_once(&input, Some(i)))
        .find(|res| matches!(res, Ok((_, true))))
        .transpose()?
        .ok_or_else(|| "not found".into())
        .map(|(res, _)| res)
}

solution!(part1 => 1939, part2 => 2212);
