use std::io::{self, Read};

fn questions_count(answers: &[&str]) -> usize {
    let mut questions = [0; 26];
    answers.iter().flat_map(|s| s.as_bytes()).for_each(|&c| {
        questions[(c - b'a') as usize] = 1;
    });
    questions.iter().sum()
}

fn main() {
    let mut inputs = String::new();
    io::stdin().read_to_string(&mut inputs).unwrap();
    let inputs: Vec<Vec<_>> = inputs.split("\n\n").map(|p| p.split('\n').collect()).collect();

    let result: usize = inputs.iter().map(|v| questions_count(&v)).sum();
    println!("part 1 result: {}", result);
}
