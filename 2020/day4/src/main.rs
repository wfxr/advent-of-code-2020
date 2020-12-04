use std::collections::HashMap;
use std::io::{self, Read};

fn p1_valid(passport: &HashMap<&str, &str>) -> bool {
    match passport.len() {
        7 => !passport.contains_key("cid"),
        8 => true,
        _ => false,
    }
}

const EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
fn p2_valid(passport: &HashMap<&str, &str>) -> bool {
    p1_valid(passport)
        && passport.iter().fold(true, |acc, (&k, v)| {
            acc && match k {
                "byr" => (1920..=2002).contains(&v.parse().unwrap()),
                "iyr" => (2010..=2020).contains(&v.parse().unwrap()),
                "eyr" => (2020..=2030).contains(&v.parse().unwrap()),
                "hgt" => {
                    let value = v.trim_end_matches(|c| !('0'..='9').contains(&c));
                    let unit = v.trim_start_matches(|c| ('0'..='9').contains(&c));
                    match (value.parse(), unit) {
                        (Ok(value), "cm") => (150..=193).contains(&value),
                        (Ok(value), "in") => (59..=76).contains(&value),
                        _ => false,
                    }
                }
                "hcl" => v.len() == 7 && v.starts_with("#") && v[1..].chars().all(|c| c.is_digit(16)),
                "ecl" => EYE_COLORS.contains(v),
                "pid" => v.len() == 9 && v.chars().all(|c| c.is_digit(10)),
                _ => true,
            }
        })
}

fn solve(passports: &[HashMap<&str, &str>], func: fn(&HashMap<&str, &str>) -> bool) -> usize {
    passports.iter().fold(0, |acc, passport| acc + func(passport) as usize)
}

fn main() {
    let reader = io::stdin();
    let mut inputs: String = String::new();
    reader.lock().read_to_string(&mut inputs).unwrap();
    let inputs: Vec<_> = inputs.split("\n\n").filter(|s| !s.is_empty()).map(parse).collect();

    println!("part 1 result: {}", solve(&inputs, p1_valid));
    println!("part 1 result: {}", solve(&inputs, p2_valid));
}

#[rustfmt::skip]
fn parse(s: &str) -> HashMap<&str, &str> {
    return s.split_whitespace().filter(|s| !s.is_empty())
        .map(|item| {
            let mut split = item.split(":");
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect();
}
