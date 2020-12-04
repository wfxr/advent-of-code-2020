use std::collections::HashSet;
use std::io::{self, Read};

fn p1_solve(passports: &[&str], required: &HashSet<&str>) -> usize {
    passports.iter().fold(0, |count, s| {
        match s.split_whitespace().fold(0, |acc, item| match item.split(':').next() {
            Some(field) => acc + required.contains(field) as usize,
            None => acc,
        }) {
            7 => count + 1,
            _ => count,
        }
    })
}

fn main() {
    let reader = io::stdin();
    let mut inputs: String = String::new();
    reader.lock().read_to_string(&mut inputs).unwrap();
    let inputs: Vec<_> = inputs.split("\n\n").filter(|s| !s.is_empty()).collect();

    let mut required = HashSet::new();
    required.insert("byr");
    required.insert("iyr");
    required.insert("eyr");
    required.insert("hgt");
    required.insert("hcl");
    required.insert("ecl");
    required.insert("pid");

    println!("part 1 result: {}", p1_solve(&inputs, &required));
}
