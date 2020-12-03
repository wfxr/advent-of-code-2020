use std::io::{self, BufRead};

fn p1_solve(map: &Vec<Vec<char>>, direction: (usize, usize)) -> usize {
    let (h, w) = (map.len(), map[0].len());
    let (dx, dy) = direction;
    let (mut px, mut py) = (0, 0);
    let mut count = 0;
    while py < h {
        count += (map[py][px] == '#') as usize;
        px = (px + dx) % w;
        py += dy;
    }
    count
}

fn main() {
    let inputs: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|line| line.chars().collect())
        .collect();

    println!("part 1 result: {}", p1_solve(&inputs, (3, 1)));
}
