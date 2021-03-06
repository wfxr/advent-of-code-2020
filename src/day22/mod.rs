use crate::{err, solution, Result};
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

type Deck = VecDeque<u8>;

enum Player {
    One,
    Two,
}

fn parse_input(input: &str) -> Result<(Deck, Deck)> {
    let mut it = input.split("\n\n").map(|part| {
        part.lines()
            .filter(|s| !s.is_empty())
            .skip(1)
            .map(|s| Ok(s.parse()?))
            .collect::<Result<_>>()
    });

    match (it.next(), it.next()) {
        (Some(player1), Some(player2)) => Ok((player1?, player2?)),
        _ => err!("no enough players"),
    }
}

fn solve(input: &str, game: fn(&mut Deck, &mut Deck) -> Result<Player>) -> Result<usize> {
    let (mut player1, mut player2) = parse_input(input)?;
    let winner = match game(&mut player1, &mut player2)? {
        Player::One => player1,
        Player::Two => player2,
    };
    Ok(winner
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &num)| acc + num as usize * (i + 1)))
}

fn duel(a: u8, b: u8) -> Result<Player> {
    match a.cmp(&b) {
        Ordering::Greater => Ok(Player::One),
        Ordering::Less => Ok(Player::Two),
        Ordering::Equal => return err!("same card: {}", a),
    }
}

fn settle(p1: &mut Deck, p2: &mut Deck, a: u8, b: u8, winner: Player) {
    match winner {
        Player::One => {
            p1.push_back(a);
            p1.push_back(b);
        }
        Player::Two => {
            p2.push_back(b);
            p2.push_back(a);
        }
    }
}

fn game1(p1: &mut Deck, p2: &mut Deck) -> Result<Player> {
    while !p1.is_empty() && !p2.is_empty() {
        let (a, b) = (p1.pop_front().unwrap(), p2.pop_front().unwrap()); // no panic
        settle(p1, p2, a, b, duel(a, b)?);
    }
    Ok(if p1.is_empty() { Player::Two } else { Player::One })
}

fn game2(p1: &mut Deck, p2: &mut Deck) -> Result<Player> {
    let (mut h1, mut h2) = (HashSet::new(), HashSet::new());
    while !p1.is_empty() && !p2.is_empty() {
        if !h1.insert(p1.clone()) && !h2.insert(p2.clone()) {
            return Ok(Player::One);
        };
        let (a, b) = (p1.pop_front().unwrap(), p2.pop_front().unwrap()); // no panic
        let winner = if p1.len() >= a as usize && p2.len() >= b as usize {
            let mut p1 = p1.iter().take(a as usize).cloned().collect();
            let mut p2 = p2.iter().take(b as usize).cloned().collect();
            game2(&mut p1, &mut p2)?
        } else {
            duel(a, b)?
        };
        settle(p1, p2, a, b, winner);
    }
    Ok(if p1.is_empty() { Player::Two } else { Player::One })
}

fn part1(input: &str) -> Result<usize> {
    solve(input, game1)
}
fn part2(input: &str) -> Result<usize> {
    solve(input, game2)
}

solution!(part1 => 31809, part2 => 32835);

#[cfg(test)]
mod examples {
    crate::test!(
        part1,
        example1: input!("Player 1:",
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
    crate::test!(
        part2,
        example1: input!("Player 1:",
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
                         "10") => 291,
    );
}
