use std::io::{self, BufRead};

fn is_valid_1(min: usize, max: usize, t: char, s: &str) -> bool {
    (min..=max).contains(&s.chars().filter(|&c| c == t).count())
}

fn is_valid_2(p1: usize, p2: usize, t: char, s: &str) -> bool {
    (s.chars().nth(p1 - 1).unwrap_or('-') == t) != (s.chars().nth(p2 - 1).unwrap_or('-') == t)
}

#[rustfmt::skip]
fn valid_count(inputs: &Vec<(usize, usize, char, String)>, f: fn(usize, usize, char, &str) -> bool) -> usize {
    inputs.iter().fold(0, |acc, (x, y, t, s)| acc + f(*x, *y, *t, s) as usize)
}

#[rustfmt::skip]
fn main() {
    let inputs: Vec<_> = io::stdin().lock().lines().map(|l| l.unwrap())
        .map(|line| {
            let mut parts = line
                .split(|c| c == '-' || c == ' ' || c == ':')
                .filter(|&s| !s.is_empty());
            let n1: usize = parts.next().unwrap().parse().unwrap();
            let n2: usize = parts.next().unwrap().parse().unwrap();
            let t:  char  = parts.next().unwrap().parse().unwrap();
            let s:  &str  = parts.next().unwrap();
            (n1, n2, t, s.to_owned())
        })
        .collect();

    println!("part1 result: {}", valid_count(&inputs, is_valid_1));
    println!("part2 result: {}", valid_count(&inputs, is_valid_2));
}
