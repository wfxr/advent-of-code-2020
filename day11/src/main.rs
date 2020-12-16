use std::io::{self, BufRead};

fn solve(seats: &[&[char]], occupied: fn(&[Vec<char>], (usize, usize)) -> usize) -> usize {
    let mut curr: Vec<_> = seats.iter().map(|row| row.to_vec()).collect();

    let mut changed = true;
    while changed {
        changed = false;
        let mut next = curr.clone();
        for i in 0..next.len() {
            for j in 0..next[i].len() {
                next[i][j] = match curr[i][j] {
                    '#' if occupied(&curr, (i, j)) >= 5 => 'L',
                    'L' if occupied(&curr, (i, j)) == 0 => '#',
                    seat => seat,
                };
                changed |= next[i][j] != curr[i][j];
            }
        }
        curr = next;
    }
    curr.iter().flatten().filter(|&&seat| seat == '#').count()
}

// self included
fn p1_occupied(seats: &[Vec<char>], (i, j): (usize, usize)) -> usize {
    let mut sum = 0;
    for i in if i == 0 { 0 } else { i - 1 }..seats.len().min(i + 2) {
        for j in if j == 0 { 0 } else { j - 1 }..seats[i].len().min(j + 2) {
            sum += (seats[i][j] == '#') as usize
        }
    }
    sum
}

// self not included
fn p2_occupied(seats: &[Vec<char>], (i, j): (usize, usize)) -> usize {
    let (maxdi, maxdj) = (seats.len() - i - 1, seats[i].len() - j - 1);

    fn occupied(mut iter: impl Iterator<Item = char>) -> usize {
        iter.find(|&s| s != '.').map(|s| (s == '#') as usize).unwrap_or(0)
    }

    let mut sum = 0;

    sum += occupied((1..=i).map(|d| seats[i - d][j])); // up
    sum += occupied((1..=j).map(|d| seats[i][j - d])); // left
    sum += occupied((1..=maxdi).map(|d| seats[i + d][j])); // down
    sum += occupied((1..=maxdj).map(|d| seats[i][j + d])); // right
    sum += occupied((1..=i.min(j)).map(|d| seats[i - d][j - d])); // up-left
    sum += occupied((1..=i.min(maxdj)).map(|d| seats[i - d][j + d])); // up-right
    sum += occupied((1..=(maxdi).min(j)).map(|d| seats[i + d][j - d])); // down-left
    sum += occupied((1..=(maxdi).min(maxdj)).map(|d| seats[i + d][j + d])); // down-right

    sum
}

fn main() {
    let inputs: Vec<Vec<_>> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    let inputs: Vec<_> = inputs.iter().map(|row| row.as_ref()).collect();

    let result = solve(&inputs, p1_occupied);
    println!("part 1 result: {}", result);
    assert_eq!(2453, result);

    let result = solve(&inputs, p2_occupied);
    println!("part 2 result: {}", result);
    assert_eq!(2159, result);
}
