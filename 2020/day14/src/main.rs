use std::collections::HashMap;
use std::io::{self, Read};

fn parse_mask(mask: &str, target: char) -> u64 {
    mask.chars()
        .map(|c| if c == target { 1 } else { 0 })
        .fold(0, |x, b| (x << 1) + b)
}

#[rustfmt::skip]
fn p1_solve(inputs: &[(&str, Vec<(u64, u64)>)]) -> u64 {
    inputs.iter()
        .fold(HashMap::new(), |mut acc, (mask, lines)| {
            let (mask0, mask1) = (!parse_mask(mask, '0'), parse_mask(mask, '1'));
            acc.extend(lines.iter().map(|(mem, val)| (mem, val & mask0 | mask1)));
            acc
        }).values().sum()
}

fn update_memory(mask: u64, addr: u64, val: u64, memory: &mut HashMap<u64, u64>) {
    if mask == 0 {
        memory.insert(addr, val);
    } else {
        let x = mask & (!mask + 1); // get right-most '1' of mask
        let mask = mask & !x; // clear right-most '1' of mask
        update_memory(mask, addr & !x, val, memory);
        update_memory(mask, addr | x, val, memory);
    }
}

#[rustfmt::skip]
fn p2_solve(inputs: &[(&str, Vec<(u64, u64)>)]) -> u64 {
    inputs.iter()
        .fold(HashMap::new(), |mut acc, (mask, lines)| {
            let (mask1, maskx) = (parse_mask(mask, '1'), parse_mask(mask, 'X'));
            lines.iter().for_each(|&(mem, val)| update_memory(maskx, mem | mask1, val, &mut acc));
            acc
        }).values().sum()
}

#[rustfmt::skip]
fn main() {
    let mut inputs = String::new();
    io::stdin().read_to_string(&mut inputs).unwrap();
    let inputs: Vec<_> = inputs.split("mask = ")
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
