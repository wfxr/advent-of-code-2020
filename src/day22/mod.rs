use crate::{solution, Result};
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

type Deck = VecDeque<usize>;

enum Player {
    A,
    B,
}

fn parse_input(input: &str) -> Result<(Deck, Deck)> {
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

fn solve<F>(input: &str, play: F) -> Result<usize>
where
    F: for<'a> Fn(&'a mut Deck, &'a mut Deck) -> Result<&'a Deck>,
{
    let (mut player1, mut player2) = parse_input(input)?;
    Ok(play(&mut player1, &mut player2)?
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, num)| acc + num * (i + 1)))
}

fn play1<'a>(p1: &'a mut Deck, p2: &'a mut Deck) -> Result<&'a Deck> {
    while !p1.is_empty() && !p2.is_empty() {
        let (a, b) = (p1.pop_front().unwrap(), p2.pop_front().unwrap()); // no panic
        match a.cmp(&b) {
            Ordering::Greater => {
                p1.push_back(a);
                p1.push_back(b);
            }
            Ordering::Less => {
                p2.push_back(b);
                p2.push_back(a);
            }
            Ordering::Equal => return Err(format!("same card: {}", a).into()),
        }
    }
    Ok(if p1.is_empty() { p2 } else { p1 })
}

fn play2<'a>(p1: &'a mut Deck, p2: &'a mut Deck) -> Result<&'a Deck> {
    match play2_inner(p1, p2)? {
        Player::A => Ok(p1),
        Player::B => Ok(p2),
    }
}

// 抓牌之前如果任意玩家牌组和（同一局）上一轮完全一样则玩家1获胜，否则按正常规则抓牌（防止无限递归）
// 如果每个玩家的剩余牌组数量都>=各自刚抓的牌数值，进入递归游戏
// 进入递归游戏时的牌组是复制得到的，数量等于各自触发递归游戏的那张牌的大小
// 否则按照之前的规则，牌面大的胜

// 赢家（包括子游戏获胜）在回合开始的时候将赢得的2张牌放到牌组底部（同part1）
// 牌组放入底部时始终是自己的先放，赢得的后放
// 首先获得全部卡牌的玩家获胜。
fn play2_inner(p1: &mut Deck, p2: &mut Deck) -> Result<Player> {
    let (mut hist1, mut hist2) = (HashSet::new(), HashSet::new());
    while !p1.is_empty() && !p2.is_empty() {
        let winner = match (hist1.insert(p1.clone()), hist2.insert(p2.clone())) {
            (true, true) => None,
            _ => Some(Player::A),
        };
        let (a, b) = (p1.pop_front().unwrap(), p2.pop_front().unwrap()); // no panic

        let winner = match winner {
            Some(winner) => winner,
            None => {
                if p1.len() >= a && p2.len() >= b {
                    let mut p1 = p1.iter().take(a).cloned().collect();
                    let mut p2 = p2.iter().take(b).cloned().collect();
                    play2_inner(&mut p1, &mut p2)?
                } else {
                    match a.cmp(&b) {
                        Ordering::Greater => Player::A,
                        Ordering::Less => Player::B,
                        Ordering::Equal => return Err(format!("same card: {}", a).into()),
                    }
                }
            }
        };
        match winner {
            Player::A => {
                p1.push_back(a);
                p1.push_back(b);
            }
            Player::B => {
                p2.push_back(b);
                p2.push_back(a);
            }
        }
    }
    Ok(if p1.is_empty() { Player::B } else { Player::A })
}

fn part1(input: &str) -> Result<usize> {
    solve(input, play1)
}
fn part2(input: &str) -> Result<usize> {
    solve(input, play2)
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
