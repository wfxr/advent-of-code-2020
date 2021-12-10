use crate::{err, solution, Result};
use std::iter;
use Rule::{And2, And3, Or2, Ref, Val};

enum Rule {
    Val(char),
    Ref(usize),
    And2(Box<Rule>, Box<Rule>),
    And3(Box<Rule>, Box<Rule>, Box<Rule>),
    Or2(Box<Rule>, Box<Rule>),
}

fn matches<'a>(rule: &'a Rule, rules: &'a [Rule], rest: &'a [char]) -> Box<dyn Iterator<Item = &'a [char]> + 'a> {
    if rest.is_empty() {
        return Box::new(iter::empty());
    }
    match rule {
        Rule::Ref(i) => matches(&rules[*i], rules, rest),
        Rule::Val(c) => match rest[0] == *c {
            true => Box::new(iter::once(&rest[1..])),
            false => Box::new(iter::empty()),
        },
        Rule::Or2(a, b) => Box::new(matches(a, rules, rest).chain(matches(b, rules, rest))),
        Rule::And2(a, b) => Box::new(matches(a, rules, rest).flat_map(move |rest| matches(b, rules, rest))),
        Rule::And3(a, b, c) => Box::new(
            matches(a, rules, rest)
                .flat_map(move |rest| matches(b, rules, rest).flat_map(move |rest| matches(c, rules, rest))),
        ),
    }
}

fn parse_rule(s: &str) -> Result<(usize, Rule)> {
    let mut it = s.split(':');
    let id: usize = it.next().ok_or("missing id")?.parse()?;
    let mut it = it.next().ok_or("missing rule")?.split('|').map(|s| {
        let mut it = s.split_whitespace().map(|s| match s.parse() {
            Ok(x) => Ok(Ref(x)),
            _ => s.chars().nth(1).ok_or("invalid char rule").map(Val),
        });
        match [it.next(), it.next(), it.next()] {
            [Some(a), Some(b), Some(c)] => Ok(And3(Box::new(a?), Box::new(b?), Box::new(c?))),
            [Some(a), Some(b), ..] => Ok(And2(Box::new(a?), Box::new(b?))),
            [Some(a), ..] => Ok(a?),
            [None, ..] => Err("no nule found"),
        }
    });
    let rule = match [it.next(), it.next()] {
        [Some(a), Some(b)] => Or2(Box::new(a?), Box::new(b?)),
        [Some(a), None] => a?,
        [None, ..] => return err!("empty rule"),
    };
    Ok((id, rule))
}

fn solve(input: &str, replace_rule: fn(&str) -> &str) -> Result<usize> {
    let mut it = input.split("\n\n");
    let mut rules: Vec<(usize, Rule)> = it
        .next()
        .ok_or("missing rules")?
        .lines()
        .map(replace_rule)
        .map(parse_rule)
        .collect::<Result<_>>()?;
    rules.sort_unstable_by_key(|item| item.0);
    let rules: Vec<_> = rules.into_iter().map(|item| item.1).collect();
    Ok(it
        .next()
        .ok_or("missing strings")?
        .lines()
        .map(|msg| msg.chars().collect::<Vec<_>>())
        .filter(|msg| matches(&rules[0], &rules, msg).any(|s| s.is_empty()))
        .count())
}

fn part1(input: &str) -> Result<usize> {
    solve(input, |s| s)
}

fn part2(input: &str) -> Result<usize> {
    solve(input, |s| match &s[..3] {
        "8: " => "8: 42 | 42 8",
        "11:" => "11: 42 31 | 42 11 31",
        _ => s,
    })
}

solution!(part1 => 220, part2 => 439);
