use std::io::{self, BufRead};

fn p1_solve(intcode: &[i64], input: i64) -> i64 {
    let mut intcode = intcode.to_vec();
    let mut result = 0;
    let mut p = 0;
    while p < intcode.len() {
        let mut modes = 1;
        let head = load(&intcode, &mut p, &mut modes) as usize;
        let (mut modes, opcode) = (head / 100, head % 100);
        match opcode {
            1 => {
                let a = load(&intcode, &mut p, &mut modes);
                let b = load(&intcode, &mut p, &mut modes);
                store(&mut intcode, &mut p, a + b);
            }
            2 => {
                let a = load(&intcode, &mut p, &mut modes);
                let b = load(&intcode, &mut p, &mut modes);
                store(&mut intcode, &mut p, a * b);
            }
            3 => store(&mut intcode, &mut p, input),
            4 => result = output(&mut intcode, &mut p),
            99 => break,
            _ => {
                println!("{}", opcode);
                println!("{}", p);
                unreachable!();
            }
        }
    }
    result
}

fn output(intcode: &mut [i64], p: &mut usize) -> i64 {
    let value = intcode[intcode[*p] as usize];
    *p += 1;
    return value;
}

fn store(intcode: &mut [i64], p: &mut usize, value: i64) {
    intcode[intcode[*p] as usize] = value;
    *p += 1;
}
fn load(intcode: &[i64], p: &mut usize, modes: &mut usize) -> i64 {
    let value = intcode[*p];
    let value = match *modes % 10 {
        0 => intcode[value as usize],
        1 => value,
        _ => unreachable!(),
    };
    *modes /= 10;
    *p += 1;
    value
}

#[rustfmt::skip]
fn main() {
    let inputs: Vec<_> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let result = p1_solve(&inputs , 1);
    println!("part 1 result: {}", result);
    assert_eq!(8332629, result);
}
