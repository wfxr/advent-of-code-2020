use crate::Solution;

fn to_postfix(expr: &str, prec: fn(char) -> u8) -> Vec<char> {
    let mut st = vec![];
    let mut rs = vec![];
    for c in expr.chars() {
        match c {
            ' ' => continue,
            '0'..='9' => rs.push(c),
            '(' => st.push(c),
            ')' => loop {
                match st.pop() {
                    Some('(') | None => break,
                    Some(op) => rs.push(op),
                };
            },
            op => {
                loop {
                    match st.last() {
                        Some('(') => break,
                        Some(&top) if prec(top) <= prec(op) => rs.push(st.pop().unwrap()),
                        _ => break,
                    }
                }
                st.push(op)
            }
        }
    }
    rs.extend(st.into_iter().rev());
    rs
}

fn evaluate(expr: &[char]) -> i64 {
    let mut st = vec![];
    for c in expr {
        match c {
            '0'..='9' => st.push(*c as i64 - '0' as i64),
            op => {
                let (rhs, lhs) = (st.pop().unwrap(), st.pop().unwrap());
                st.push(match op {
                    '+' => lhs + rhs,
                    '*' => lhs * rhs,
                    _ => panic!(format!("unexpected operator: {:?}", op)),
                });
            }
        }
    }
    st[0]
}

fn solve(input: &str, prec: fn(char) -> u8) -> i64 {
    input
        .lines()
        .map(|expr| to_postfix(expr, prec))
        .map(|expr| evaluate(&expr))
        .sum()
}

pub(super) const SOLUTION: Solution = Solution {
    part1: |input| {
        let result = solve(input, |op| match op {
            '+' => 0,
            '*' => 0,
            _ => panic!(format!("unexpected operator: '{}'", op)),
        });
        Ok(result.to_string())
    },
    part2: |input| {
        let result = solve(input, |op| match op {
            '+' => 0,
            '*' => 1,
            _ => panic!(format!("unexpected operator: '{}'", op)),
        });
        Ok(result.to_string())
    },
};

#[cfg(test)]
crate::solution_test!(16332191652452, 351175492232654);
