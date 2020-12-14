use std::collections::HashMap;
use std::io::{self, Read};

fn p1_solve(inputs: &[(&str, Vec<(u64, u64)>)]) -> u64 {
    inputs
        .iter()
        .fold(HashMap::new(), |mut acc, (mask, lines)| {
            let mask0 = mask
                .chars()
                .map(|c| if c == '0' { 0 } else { 1 })
                .fold(0u64, |x, b| x * 2 + b);
            let mask1 = mask
                .chars()
                .map(|c| if c == '1' { 1 } else { 0 })
                .fold(0u64, |x, b| x * 2 + b);
            acc.extend(lines.iter().map(|(mem, val)| (mem, val & mask0 | mask1)));
            acc
        })
        .values()
        .sum()
}

fn combinations(mask: u64, val: u64, addrs: &mut Vec<u64>) {
    if mask == 0 {
        addrs.push(val);
    } else {
        let x = mask & (!mask + 1); // get right-most '1' of mask
        let mask = mask & !x; // clear right-most '1' of mask
        combinations(mask, val & !x, addrs);
        combinations(mask, val | x, addrs);
    }
}

fn p2_solve(inputs: &[(&str, Vec<(u64, u64)>)]) -> u64 {
    inputs
        .iter()
        .fold(HashMap::new(), |mut acc, (mask, lines)| {
            let mask1 = mask
                .chars()
                .map(|c| if c == '1' { 1 } else { 0 })
                .fold(0u64, |x, b| x * 2 + b);
            let maskx = mask
                .chars()
                .map(|c| if c == 'X' { 1 } else { 0 })
                .fold(0u64, |x, b| x * 2 + b);
            acc.extend(lines.iter().flat_map(|&(mem, val)| {
                let mut addrs = Vec::new();
                combinations(maskx, mem | mask1, &mut addrs);
                addrs.into_iter().map(move |addr| (addr, val))
            }));
            acc
        })
        .values()
        .sum()
}

fn main() {
    let mut inputs = String::new();
    io::stdin().read_to_string(&mut inputs).unwrap();
    let inputs: Vec<_> = inputs
        .split("mask = ")
        .map(|s| {
            let mut it = s.split('\n');
            let mask = it.next().unwrap().trim();
            let lines: Vec<(u64, u64)> = it
                .filter_map(|line| {
                    let mut it = line.split(|c: char| !c.is_digit(10)).filter(|s| !s.is_empty());
                    match (it.next(), it.next()) {
                        (Some(mem), Some(val)) => Some((mem.parse().unwrap(), val.parse().unwrap())),
                        _ => None,
                    }
                })
                .collect();
            (mask, lines)
        })
        .collect();

    let result = p1_solve(&inputs);
    println!("part 1 result: {}", result);
    assert_eq!(5902420735773, result);

    let result = p2_solve(&inputs);
    println!("part 2 result: {}", result);
    assert_eq!(3801988250775, result);
}
