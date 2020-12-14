use std::io::{self, BufRead};

fn p1_solve(schedule: &[&str], t: usize) -> (usize, usize) {
    let schedule = schedule.iter().filter(|&&s| s != "x").map(|bus| bus.parse().unwrap());
    schedule.map(|bus| (bus - t % bus, bus)).min().unwrap()
}

fn main() {
    let reader = io::stdin();
    let mut lines = reader.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let schedule = lines.next().unwrap().unwrap();
    let schedule: Vec<_> = schedule.split(',').collect();

    let result = p1_solve(&schedule, t);
    println!("part 1 result: {} {:?}", result.0 * result.1, result);
    assert_eq!(2305, result.0 * result.1);
}
