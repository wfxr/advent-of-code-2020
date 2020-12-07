use std::cmp::Ordering;

fn p1_solve(min: usize, max: usize) -> usize {
    (min..=max)
        .filter(|n| {
            let mut x = *n;
            let mut prev = 10;
            let mut repeated = 0;
            while x != 0 {
                let curr = x % 10;
                x /= 10;
                match curr.cmp(&prev) {
                    Ordering::Greater => return false,
                    Ordering::Equal => repeated += 1,
                    _ => {}
                }
                prev = curr;
            }
            repeated > 0
        })
        .count()
}

fn main() {
    let result = p1_solve(278384, 824795);
    println!("part 1 result: {}", result);
    assert_eq!(921, result);
}
