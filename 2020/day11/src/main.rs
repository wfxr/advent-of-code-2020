use std::io::{self, BufRead};

fn p1_solve(seats: &[&[char]]) -> usize {
    let mut curr: Vec<_> = seats.iter().map(|row| row.to_vec()).collect();

    let mut transformed = true;
    while transformed {
        transformed = false;
        let mut next = curr.clone();
        for i in 0..next.len() {
            for j in 0..next[i].len() {
                if should_transform(&curr, i, j) {
                    next[i][j] = match curr[i][j] {
                        '#' => 'L',
                        'L' => '#',
                        _ => unreachable!(),
                    };
                    transformed = true;
                }
            }
        }
        curr = next;
    }
    curr.iter().flatten().filter(|&&seat| seat == '#').count()
}

fn should_transform(seats: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    match seats[i][j] {
        '#' => squre_occupied(seats, i, j) >= 5,
        'L' => squre_occupied(seats, i, j) == 0,
        _ => false,
    }
}

fn squre_occupied(seats: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut sum = 0;
    for i in if i == 0 { 0 } else { i - 1 }..seats.len().min(i + 2) {
        for j in if j == 0 { 0 } else { j - 1 }..seats[i].len().min(j + 2) {
            sum += (seats[i][j] == '#') as usize
        }
    }
    sum
}

fn main() {
    let inputs: Vec<Vec<_>> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let inputs: Vec<_> = inputs.iter().map(|row| row.as_ref()).collect();

    let result = p1_solve(&inputs);
    println!("part 1 result: {}", result);
    assert_eq!(0, result);
}
