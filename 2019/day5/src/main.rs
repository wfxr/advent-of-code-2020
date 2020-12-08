use std::io::{self, BufRead};

struct IntcodeComputer {
    program: Vec<i64>,
    pos:     usize,
    modes:   usize,
    opcode:  usize,
}

impl IntcodeComputer {
    pub fn new(program: &[i64]) -> Self {
        IntcodeComputer {
            program: program.to_vec(),
            pos:     0,
            modes:   0,
            opcode:  0,
        }
    }

    pub fn run(&mut self, input: i64) -> i64 {
        let mut res = 0;
        while !self.finished() {
            self.load_instruction();
            match self.opcode {
                1 => self.op_binary(|v1, v2| v1 + v2),
                2 => self.op_binary(|v1, v2| v1 * v2),
                3 => self.save(input),
                4 => res = self.deref_load(),
                5 => self.jump_if(|x| x != 0),
                6 => self.jump_if(|x| x == 0),
                7 => self.set_if(|v1, v2| v1 < v2),
                8 => self.set_if(|v1, v2| v1 == v2),
                99 => break,
                _ => unreachable!("unknown opcode {} (pos: {})", self.opcode, self.pos),
            }
        }
        res
    }

    pub fn finished(&self) -> bool {
        self.pos >= self.program.len()
    }

    fn jump(&mut self) {
        self.pos = self.load_param() as usize;
    }

    fn jump_if(&mut self, cond: fn(i64) -> bool) {
        match cond(self.load_param()) {
            true => self.jump(),
            false => self.skip(1),
        }
    }

    fn set_if(&mut self, cond: fn(i64, i64) -> bool) {
        match cond(self.load_param(), self.load_param()) {
            true => self.save(1),
            false => self.save(0),
        }
    }

    fn op_binary(&mut self, op: fn(i64, i64) -> i64) {
        let (a, b) = (self.load_param(), self.load_param());
        self.save(op(a, b));
    }

    fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    fn load(&mut self) -> i64 {
        let res = self.program[self.pos];
        self.pos += 1;
        res
    }

    fn deref_load(&mut self) -> i64 {
        let addr = self.load() as usize;
        self.program[addr]
    }

    fn load_param(&mut self) -> i64 {
        let data = match self.modes % 10 {
            0 => self.deref_load(),
            1 => self.load(),
            _ => unreachable!("unknown modes {} (pos: {})", self.modes, self.pos),
        };
        self.modes /= 10;
        data
    }

    fn load_instruction(&mut self) {
        let instruction = self.load() as usize;
        self.modes = instruction / 100;
        self.opcode = instruction % 100;
    }

    fn save(&mut self, value: i64) {
        let addr = self.load() as usize;
        self.program[addr] = value;
    }
}

#[rustfmt::skip]
fn main() {
    let inputs: Vec<_> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut computer = IntcodeComputer::new(&inputs);
    let result = computer.run(1);
    println!("part 1 result: {}", result);
    assert_eq!(8332629, result);

    let mut computer = IntcodeComputer::new(&inputs);
    let result = computer.run(5);
    println!("part 5 result: {}", result);
    assert_eq!(8805067, result);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_computer {
        ($program:expr, $input:expr, $expect:expr) => {
            let mut computer = IntcodeComputer::new($program);
            let actual = computer.run($input);
            assert_eq!($expect, actual);
        };
    }

    #[test]
    fn test_day2() {
        test_computer!(&[1, 0, 0, 0, 99], 0, 0);
        test_computer!(&[2, 3, 0, 3, 99], 0, 0);
        test_computer!(&[2, 4, 4, 5, 99, 0], 0, 0);
        test_computer!(&[1, 1, 1, 4, 99, 5, 6, 0, 99], 0, 0);
    }

    #[test]
    fn test_day5() {
        let input = &[3, 0, 4, 0, 99];
        test_computer!(input, 302, 302);
        test_computer!(input, -1, -1);

        let input = &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        test_computer!(input, 7, 0);
        test_computer!(input, 8, 1);

        let input = &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        test_computer!(input, 8, 0);
        test_computer!(input, 7, 1);

        let input = &[3, 3, 1108, -1, 8, 3, 4, 3, 99];
        test_computer!(input, 7, 0);
        test_computer!(input, 8, 1);

        let input = &[3, 3, 1107, -1, 8, 3, 4, 3, 99];
        test_computer!(input, 8, 0);
        test_computer!(input, 7, 1);
    }
}
