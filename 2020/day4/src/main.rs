use std::collections::HashMap;
use std::io::{self, Read};

fn p1_solve(passports: &[HashMap<&str, &str>]) -> usize {
    passports.iter().fold(0, |count, passport| match passport.len() {
        7 => count + !passport.contains_key("cid") as usize,
        8 => count + 1,
        _ => count,
    })
}

fn main() {
    let reader = io::stdin();
    let mut inputs: String = String::new();
    reader.lock().read_to_string(&mut inputs).unwrap();
    let inputs: Vec<_> = inputs.split("\n\n").filter(|s| !s.is_empty()).map(parse).collect();

    println!("part 1 result: {}", p1_solve(&inputs));
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
