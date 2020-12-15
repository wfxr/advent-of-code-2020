use std::io::{self, BufRead};

fn p1_solve(program: &[i64], phases: &[i64], init: i64) -> i64 {
    let mut max = i64::MIN;
    for phases in permutations(phases) {
        let mut x = init;
        for phase in &phases {
            x = IntcodeComputer::new(program).run(vec![*phase, x].iter()).unwrap()
        }
        max = max.max(x);
    }
    max
}

fn p2_solve(program: &[i64], phases: &[i64], init: i64) -> i64 {
    let mut max = i64::MIN;
    for phases in permutations(phases) {
        let mut amps = vec![IntcodeComputer::new(program); 5];
        let (mut signal, mut phases) = (init, phases.iter());
        for i in (0..amps.len()).cycle() {
            match amps[i].run(vec![phases.next(), Some(&signal)].into_iter().filter_map(|n| n)) {
                Some(output) => signal = output,
                None => break,
            }
        }
        max = max.max(signal);
    }
    max
}

fn do_permutations(v: &mut Vec<i64>, pos: usize, result: &mut Vec<Vec<i64>>) {
    if pos == v.len() {
        result.push(v.clone());
    } else {
        for i in pos..v.len() {
            v.swap(pos, i);
            do_permutations(v, pos + 1, result);
            v.swap(pos, i);
        }
    }
}

fn permutations(v: &[i64]) -> Vec<Vec<i64>> {
    let mut results = Vec::new();
    do_permutations(&mut v.to_vec(), 0, &mut results);
    results
}

#[rustfmt::skip]
fn main() {
    let inputs: Vec<_> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let phases: Vec<_> = (0..5).collect();
    let result = p1_solve(&inputs, &phases, 0);
    println!("part 1 result: {}", result);
    assert_eq!(65464, result);

    let phases: Vec<_> = (5..10).collect();
    let result = p2_solve(&inputs, &phases, 0);
    println!("part 2 result: {}", result);
    assert_eq!(1518124, result);
}

#[derive(Debug, Clone)]
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

    pub fn run<'a>(&mut self, mut input: impl Iterator<Item = &'a i64>) -> Option<i64> {
        while !self.finished() {
            self.load_instruction();
            match self.opcode {
                1 => self.op_binary(|v1, v2| v1 + v2),
                2 => self.op_binary(|v1, v2| v1 * v2),
                3 => self.save(*input.next().expect("failed get input")),
                4 => return Some(self.deref_load()),
                5 => self.jump_if(|x| x != 0),
                6 => self.jump_if(|x| x == 0),
                7 => self.set_if(|v1, v2| v1 < v2),
                8 => self.set_if(|v1, v2| v1 == v2),
                99 => break,
                _ => unreachable!("unknown opcode {} (pos: {})", self.opcode, self.pos),
            }
        }
        None
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

#[cfg(test)]
mod computer_tests {
    use super::*;

    macro_rules! test_computer {
        ($program:expr, $input:expr, $expect:expr) => {
            let mut computer = IntcodeComputer::new($program);
            let actual = computer.run(vec![$input].iter());
            assert_eq!($expect, actual);
        };
    }

    #[test]
    fn test_day2() {
        test_computer!(&[1, 0, 0, 0, 99], 0, None);
        test_computer!(&[2, 3, 0, 3, 99], 0, None);
        test_computer!(&[2, 4, 4, 5, 99, 0], 0, None);
        test_computer!(&[1, 1, 1, 4, 99, 5, 6, 0, 99], 0, None);
    }

    #[test]
    fn test_day5() {
        let input = &[3, 0, 4, 0, 99];
        test_computer!(input, 302, Some(302));
        test_computer!(input, -1, Some(-1));

        let input = &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        test_computer!(input, 7, Some(0));
        test_computer!(input, 8, Some(1));

        let input = &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        test_computer!(input, 8, Some(0));
        test_computer!(input, 7, Some(1));

        let input = &[3, 3, 1108, -1, 8, 3, 4, 3, 99];
        test_computer!(input, 7, Some(0));
        test_computer!(input, 8, Some(1));

        let input = &[3, 3, 1107, -1, 8, 3, 4, 3, 99];
        test_computer!(input, 8, Some(0));
        test_computer!(input, 7, Some(1));
    }

    #[test]
    fn test_rerun1() {
        let program = &[3, 0, 4, 0, 4, 0, 99];
        let mut computer = IntcodeComputer::new(program);
        let output = computer.run(vec![302].iter());
        assert_eq!(output, Some(302));
        let output = computer.run(vec![302].iter());
        assert_eq!(output, Some(302));

        let output = computer.run(vec![302].iter());
        assert_eq!(output, None);
        let output = computer.run(vec![302].iter());
        assert_eq!(output, None);
    }

    #[test]
    fn test_rerun2() {
        #[rustfmt::skip]
        let program = &[
            3, 26,
            1001, 26, -4, 26,
            3, 27,
            1002, 27, 2, 27,
            1, 27, 26, 27,
            4, 27,
            1001, 28, -1, 28,
            1005, 28, 6,
            99,
            0, 0, 5,
        ];
        let mut computer = IntcodeComputer::new(program);

        let output = computer.run(vec![100, 200].iter());
        assert_eq!(computer.pos, 18);
        assert_eq!(output, Some(496));

        let output = computer.run(vec![496].iter());
        assert_eq!(computer.pos, 18);
        assert_eq!(output, Some(1088));

        let output = computer.run(vec![1088].iter());
        assert_eq!(computer.pos, 18);
        assert_eq!(output, Some(2272));
    }
}
