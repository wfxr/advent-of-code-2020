use std::io::{self, BufRead};

fn p1_solve(adapters: &[usize]) -> usize {
    let mut adapters = adapters.to_vec();
    adapters.sort();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);

    let (mut jolt1, mut jolt3) = (0, 0);
    for (curr, next) in adapters.iter().zip(adapters.iter().skip(1)) {
        match next - curr {
            1 => jolt1 += 1,
            3 => jolt3 += 1,
            _ => {}
        }
    }
    jolt1 * jolt3
}

fn main() {
    let inputs: Vec<usize> = io::stdin().lock().lines().map( |l|
        l.unwrap().parse().unwrap()
    ).collect();

    let result = p1_solve(&inputs);
    println!("part 1 result: {}", result);
    assert_eq!(0, result);
}
