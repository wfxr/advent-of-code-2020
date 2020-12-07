use std::io::{self, Read};

fn p1_count(answers: &[&str]) -> usize {
    let mut questions = [0; 26];
    answers.iter().flat_map(|s| s.as_bytes()).for_each(|&c| {
        questions[(c - b'a') as usize] = 1;
    });
    questions.iter().sum()
}

fn p2_count(answers: &[&str]) -> usize {
    let mut questions = [0; 26];
    answers.iter().flat_map(|s| s.as_bytes()).for_each(|&c| {
        questions[(c - b'a') as usize] += 1;
    });
    questions.iter().filter(|&&n| n == answers.len()).count()
}

fn main() {
    let mut inputs = String::new();
    io::stdin().read_to_string(&mut inputs).unwrap();
    let inputs: Vec<Vec<_>> = inputs
        .split("\n\n")
        .map(|p| p.split('\n').filter(|s| !s.is_empty()).collect())
        .collect();

    let result: usize = inputs.iter().map(|v| p1_count(&v)).sum();
    println!("part 1 result: {}", result);

    let result: usize = inputs.iter().map(|v| p2_count(&v)).sum();
    println!("part 2 result: {}", result);
}
