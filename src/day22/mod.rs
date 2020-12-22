use crate::{solution, Result};
use std::cmp::Ordering;
use std::collections::VecDeque;

fn parse_input(input: &str) -> Result<(VecDeque<usize>, VecDeque<usize>)> {
    let mut it = input.split("\n\n").map(|part| {
        part.lines()
            .filter(|s| !s.is_empty())
            .skip(1)
            .map(|s| s.parse::<usize>().map_err(Into::into))
            .collect::<Result<_>>()
    });

    match (it.next(), it.next()) {
        (Some(player1), Some(player2)) => Ok((player1?, player2?)),
        _ => Err("no enough players".into()),
    }
}

fn play<'a>(pa: &'a mut VecDeque<usize>, pb: &'a mut VecDeque<usize>) -> Result<&'a VecDeque<usize>> {
    while !pa.is_empty() && !pb.is_empty() {
        let (a, b) = (pa.pop_front().unwrap(), pb.pop_front().unwrap()); // no panic
        match a.cmp(&b) {
            Ordering::Greater => {
                pa.push_back(a);
                pa.push_back(b);
            }
            Ordering::Less => {
                pb.push_back(b);
                pb.push_back(a);
            }
            Ordering::Equal => return Err(format!("same card: {}", a).into()),
        }
    }
    Ok(if pa.is_empty() { pb } else { pa })
}

fn part1(input: &str) -> Result<usize> {
    let (mut pa, mut pb) = parse_input(input)?;
    Ok(play(&mut pa, &mut pb)?
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, num)| acc + num * (i + 1)))
}

fn part2(input: &str) -> Result<usize> {
    unimplemented!()
}

solution!(part1 => 31809, part2 => 0);

#[cfg(test)]
mod example {
    crate::test!(
        part1,
        example: input!("Player 1:",
                        "9",
                        "2",
                        "6",
                        "3",
                        "1",
                        "",
                        "Player 2:",
                        "5",
                        "8",
                        "4",
                        "7",
                        "10") => 306,
    );
}
