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

fn p2_solve(min: usize, max: usize) -> usize {
    (min..=max)
        .filter(|n| {
            let mut x = *n;
            let mut prev = 10;
            let mut digits = [0; 10];
            while x != 0 {
                let curr = x % 10;
                x /= 10;
                if curr > prev {
                    return false;
                }
                digits[curr] += 1;
                prev = curr;
            }
            digits.iter().any(|&count| count == 2)
        })
        .count()
}

fn main() {
    let (min, max) = (278384, 824795);

    let result = p1_solve(min, max);
    println!("part 1 result: {}", result);
    assert_eq!(921, result);

    let result = p2_solve(min, max);
    println!("part 2 result: {}", result);
    assert_eq!(921, result);
}
