use std::io::{self, BufRead};

fn p1_solve(map: &[&[char]], direction: (usize, usize)) -> usize {
    let (dx, dy) = direction;
    let mut x = 0;
    return map.iter().step_by(dy).fold(0, |acc, row| {
        let acc = acc + (row[x] == '#') as usize;
        x = (x + dx) % row.len();
        acc
    });
}

fn p2_solve(map: &[&[char]], slopes: &[(usize, usize)]) -> usize {
    slopes.iter().fold(1, |acc, &direction| acc * p1_solve(map, direction))
}

fn main() {
    #[rustfmt::skip]
    let inputs: Vec<Vec<_>> = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .map(|line| line.chars().collect())
        .collect();
    let inputs: Vec<&[char]> = inputs.iter().map(|v| v.as_ref()).collect();

    let rs = p1_solve(&inputs, (3, 1));
    println!("part 1 result: {}", rs);
    assert_eq!(237, rs);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let rs = p2_solve(&inputs, &slopes);
    println!("part 2 result: {}", rs);
    assert_eq!(2106818610, rs);
}
