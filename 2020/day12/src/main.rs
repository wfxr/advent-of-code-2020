use std::io::{self, BufRead};

fn p1_solve(ins: &[(char, i64)]) -> i64 {
    let ((x, y), _) = ins
        .iter()
        .fold(((0, 0), (1, 0)), |((x, y), (dx, dy)), &(ins, v)| match ins {
            'F' => ((x + v * dx, y + v * dy), (dx, dy)),
            'L' => ((x, y), turn_left((dx, dy), v)),
            'R' => ((x, y), turn_left((dx, dy), 360 - v)),
            'N' => ((x, y + v), (dx, dy)),
            'S' => ((x, y - v), (dx, dy)),
            'E' => ((x + v, y), (dx, dy)),
            'W' => ((x - v, y), (dx, dy)),
            _ => unreachable!(),
        });
    x.abs() + y.abs()
}

fn turn_left((dx, dy): (i64, i64), degrees: i64) -> (i64, i64) {
    match degrees {
        90 => (-dy, dx),
        180 => (-dx, -dy),
        270 => (dy, -dx),
        _ => unreachable!("degrees: {}", degrees),
    }
}

fn main() {
    let inputs: Vec<(char, i64)> = io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
        .collect();

    let result = p1_solve(&inputs);
    println!("part 1 result: {}", result);
    assert_eq!(0, result);
}
