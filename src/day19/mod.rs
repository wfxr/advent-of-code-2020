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

fn parse_rule(s: &str) -> (usize, Rule) {
    let mut it = s.split(':');
    let id: usize = it.next().unwrap().parse().unwrap();
    let mut it = it.next().unwrap().split('|').map(|s| {
        let mut it = s.split_whitespace().map(|s| {
            s.parse()
                .map(Ref)
                .unwrap_or_else(|_| Val(s.chars().nth(1).unwrap_or_else(|| panic!("s: '{}'", s))))
        });
        match (it.next(), it.next(), it.next()) {
            (Some(a), Some(b), Some(c)) => And3(Box::new(a), Box::new(b), Box::new(c)),
            (Some(a), Some(b), None) => And2(Box::new(a), Box::new(b)),
            (Some(a), None, None) => a,
            _ => unreachable!(),
        }
    });
    let rule = match (it.next(), it.next()) {
        (Some(a), Some(b)) => Or2(Box::new(a), Box::new(b)),
        (Some(a), None) => a,
        _ => unreachable!(),
    };
    (id, rule)
}

fn solve(input: &str, replacer: fn(&str) -> &str) -> usize {
    let mut it = input.split("\n\n");
    let mut rules: Vec<_> = it.next().unwrap().lines().map(replacer).map(parse_rule).collect();
    rules.sort_unstable_by_key(|item| item.0);
    let rules: Vec<_> = rules.into_iter().map(|item| item.1).collect();
    it.next()
        .unwrap()
        .lines()
        .map(|msg| msg.chars().collect::<Vec<_>>())
        .filter(|msg| matches(&rules[0], &rules, &msg).any(|s| s.is_empty()))
        .count()
}

fn part1(input: &str) -> usize {
    solve(input, |s| s)
}

fn part2(input: &str) -> usize {
    solve(input, |s| match &s[..3] {
        "8: " => "8: 42 | 42 8",
        "11:" => "11: 42 31 | 42 11 31",
        _ => s,
    })
}

crate::solution!(part1 => 220, part2 => 439);
