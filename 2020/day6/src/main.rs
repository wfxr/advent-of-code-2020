use std::io::{self, Read};

fn count<F>(answers: &[&str], f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    let mut questions = [0; 26];
    answers.iter().flat_map(|s| s.as_bytes()).for_each(|&c| {
        questions[(c - b'a') as usize] += 1;
    });
    questions.iter().filter(|&&n| f(n)).count()
}

#[rustfmt::skip]
fn main() {
    let mut inputs = String::new();
    io::stdin().read_to_string(&mut inputs).unwrap();
    let inputs: Vec<Vec<_>> = inputs.split("\n\n")
        .map(|p| p.split('\n').filter(|s| !s.is_empty()).collect())
        .collect();

    let result: usize = inputs.iter().map(|v| count(&v, |n| n > 0)).sum();
    println!("part 1 result: {}", result);
    assert_eq!(6416, result);

    let result: usize = inputs.iter().map(|v| count(&v, |n| n == v.len())).sum();
    println!("part 2 result: {}", result);
    assert_eq!(3050, result);
}
