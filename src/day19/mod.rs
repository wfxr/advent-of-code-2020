use Rule::{And2, And3, Ch, Or, Ref};

enum Rule {
    Ch(char),
    Ref(usize),
    And2(Box<Rule>, Box<Rule>),
    And3(Box<Rule>, Box<Rule>, Box<Rule>),
    Or(Box<Rule>, Box<Rule>),
}

fn matches<'a>(rule: &Rule, rules: &'a [Rule], rest: &'a [char]) -> Vec<&'a [char]> {
    if rest.is_empty() {
        return vec![];
    }
    match rule {
        Rule::Ref(i) => matches(&rules[*i], rules, rest),
        Rule::Ch(c) => match &rest[0] {
            next if next == c => vec![&rest[1..]],
            _ => vec![],
        },
        Rule::Or(a, b) => matches(a, rules, rest)
            .into_iter()
            .chain(matches(b, rules, rest).into_iter())
            .collect(),
        Rule::And2(a, b) => matches(a, rules, rest)
            .into_iter()
            .flat_map(|rest| matches(b, rules, rest).into_iter())
            .collect(),
        Rule::And3(a, b, c) => matches(a, rules, rest)
            .into_iter()
            .flat_map(|rest| {
                matches(b, rules, rest)
                    .into_iter()
                    .flat_map(|rest| matches(c, rules, rest).into_iter())
            })
            .collect(),
    }
}
fn parse_base_rule(s: &str) -> Rule {
    match s.parse() {
        Ok(i) => Ref(i),
        _ => Ch(s.chars().nth(1).expect(&format!("s: '{}'", s))),
    }
}

fn parse_rule(s: &str) -> (usize, Rule) {
    let mut it = s.split(':');
    let id: usize = it.next().unwrap().parse().unwrap();
    let mut it = it.next().unwrap().split('|').map(|s| {
        let mut it = s.split_whitespace().map(parse_base_rule);
        match (it.next(), it.next(), it.next()) {
            (Some(a), Some(b), Some(c)) => And3(Box::new(a), Box::new(b), Box::new(c)),
            (Some(a), Some(b), None) => And2(Box::new(a), Box::new(b)),
            (Some(a), None, None) => a,
            _ => unreachable!(),
        }
    });
    let rule = match (it.next(), it.next()) {
        (Some(a), Some(b)) => Or(Box::new(a), Box::new(b)),
        (Some(a), None) => a,
        _ => unreachable!(),
    };
    (id, rule)
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<&str>) {
    let mut it = input.split("\n\n");
    let mut rules: Vec<_> = it.next().unwrap().lines().map(parse_rule).collect();
    rules.sort_unstable_by_key(|item| item.0);
    let rules = rules.into_iter().map(|item| item.1).collect();
    let msgs = it.next().unwrap().lines().collect();
    (rules, msgs)
}

fn part1(input: &str) -> usize {
    let (rules, msgs) = parse_input(input);
    msgs.iter()
        .map(|msg| msg.chars().collect::<Vec<_>>())
        .filter(|msg| matches(&rules[0], &rules, &msg).into_iter().any(|s| s.is_empty()))
        .count()
}

fn part2(input: &str) -> usize {
    unimplemented!()
}

crate::solution!(part1 => 0, part2 => 0);
