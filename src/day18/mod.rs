use crate::{solution_result, Result};

fn to_postfix(expr: &str, prec: fn(char) -> Result<u8>) -> Result<Vec<char>> {
    let (mut st, mut rs) = (vec![], vec![]);
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
                        Some(&top) if prec(top)? <= prec(op)? => rs.push(st.pop().unwrap()),
                        _ => break,
                    }
                }
                st.push(op)
            }
        }
    }
    rs.extend(st.into_iter().rev());
    Ok(rs)
}

fn evaluate(expr: &[char]) -> Result<i64> {
    let mut st = vec![];
    for c in expr {
        match c {
            '0'..='9' => st.push(*c as i64 - '0' as i64),
            op => {
                let (rhs, lhs) = (st.pop().ok_or("missing operand")?, st.pop().ok_or("missing operand")?);
                st.push(match op {
                    '+' => lhs + rhs,
                    '*' => lhs * rhs,
                    _ => return Err(format!("unexpected operator: {}", op).into()),
                });
            }
        }
    }
    match st.len() {
        0..=1 => st.pop().ok_or_else(|| "no result".into()),
        _ => Err(format!("extra operands: {:?}", &st[0..st.len() - 1]).into()),
    }
}

fn solve(input: &str, prec: fn(char) -> Result<u8>) -> Result<i64> {
    input
        .lines()
        .map(|expr| to_postfix(expr, prec))
        .map(|expr| expr.map(|expr| evaluate(&expr))?)
        .sum()
}

fn part1(input: &str) -> Result<i64> {
    solve(input, |op| match op {
        '+' => Ok(0),
        '*' => Ok(0),
        _ => Err(format!("unexpected operator: '{}'", op).into()),
    })
}

fn part2(input: &str) -> Result<i64> {
    solve(input, |op| match op {
        '+' => Ok(0),
        '*' => Ok(1),
        _ => Err(format!("unexpected operator: '{}'", op).into()),
    })
}

solution_result!(part1 => 16332191652452, part2 => 351175492232654);
