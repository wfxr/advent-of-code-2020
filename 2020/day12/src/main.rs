use std::io::{self, BufRead};

struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn turn(&self, degrees: i64) -> Self {
        match degrees {
            90 => Point::new(-self.y, self.x),
            180 => Point::new(-self.x, -self.y),
            270 => Point::new(self.y, -self.x),
            _ => unreachable!("degrees: {}", degrees),
        }
    }

    fn mul(&self, rhs: i64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }

    fn add(&self, rhs: &Point) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    fn manhattan(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

fn p1_solve(ins: &[(char, i64)]) -> usize {
    let mut ship = Point::new(0, 0);
    let mut direction = Point::new(1, 0);
    for &(ins, v) in ins {
        match ins {
            'N' => ship.y += v,
            'S' => ship.y -= v,
            'E' => ship.x += v,
            'W' => ship.x -= v,
            'L' => direction = direction.turn(v),
            'R' => direction = direction.turn(360 - v),
            'F' => ship = ship.add(&direction.mul(v)),
            _ => unreachable!(),
        }
    }
    ship.manhattan()
}

fn p2_solve(ins: &[(char, i64)]) -> usize {
    let mut waypoint = Point::new(10, 1);
    let mut ship = Point::new(0, 0);
    for &(ins, v) in ins {
        match ins {
            'N' => waypoint.y += v,
            'S' => waypoint.y -= v,
            'E' => waypoint.x += v,
            'W' => waypoint.x -= v,
            'L' => waypoint = waypoint.turn(v),
            'R' => waypoint = waypoint.turn(360 - v),
            'F' => ship = ship.add(&waypoint.mul(v)),
            _ => unreachable!("instruction: {}", ins),
        }
    }
    ship.manhattan()
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
