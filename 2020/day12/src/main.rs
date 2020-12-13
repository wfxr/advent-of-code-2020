use std::io::{self, BufRead};

fn p1_solve(ins: &[(char, i64)]) -> i64 {
    let ((x, y), _) = ins
        .iter()
        .fold(((0, 0), (1, 0)), |((x, y), (dx, dy)), &(ins, v)| match ins {
            'N' => ((x, y + v), (dx, dy)),
            'S' => ((x, y - v), (dx, dy)),
            'E' => ((x + v, y), (dx, dy)),
            'W' => ((x - v, y), (dx, dy)),
            'L' => ((x, y), turn_left((dx, dy), v)),
            'R' => ((x, y), turn_left((dx, dy), 360 - v)),
            'F' => ((x + v * dx, y + v * dy), (dx, dy)),
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

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Point { x, y }
    }

    fn turn(&self, degrees: i64) -> Self {
        match degrees {
            90 => Point::new(-self.y, self.x),
            180 => Point::new(-self.x, -self.y),
            270 => Point::new(self.y, -self.x),
            _ => unreachable!("degrees: {}", degrees),
        }
    }

    fn offset(&self, dx: i64, dy: i64) -> Self {
        Point::new(self.x + dx, self.y + dy)
    }

    fn manhattan(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}
fn p2_solve(ins: &[(char, i64)]) -> usize {
    let mut w = Point { x: 10, y: 1 };
    let mut s = Point { x: 0, y: 0 };
    for &(ins, v) in ins {
        match ins {
            'N' => w.y += v,
            'S' => w.y -= v,
            'E' => w.x += v,
            'W' => w.x -= v,
            'L' => w = w.turn(v),
            'R' => w = w.turn(360 - v),
            'F' => s = s.offset(w.x * v, w.y * v),
            _ => unreachable!("instruction: {}", ins),
        }
    }
    s.manhattan()
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
    assert_eq!(1956, result);

    let result = p2_solve(&inputs);
    println!("part 2 result: {}", result);
    assert_eq!(126797, result);
}
