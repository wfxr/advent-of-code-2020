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
                1 => {
                    let (a, b) = (self.load_param(), self.load_param());
                    self.save(a + b);
                }
                2 => {
                    let (a, b) = (self.load_param(), self.load_param());
                    self.save(a * b);
                }
                3 => self.save(input),
                4 => res = self.deref_load(),
                99 => break,
                _ => unreachable!("unknown opcode {} (pos: {})", self.opcode, self.pos),
            }
        }
        res
    }

    pub fn finished(&self) -> bool {
        self.pos >= self.program.len()
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
    test(&[1,0,0,0,99],          0,   &[2,0,0,0,99],          0);
    test(&[2,3,0,3,99],          0,   &[2,3,0,6,99],          0);
    test(&[2,4,4,5,99,0],        0,   &[2,4,4,5,99,9801],     0);
    test(&[1,1,1,4,99,5,6,0,99], 0,   &[30,1,1,4,2,5,6,0,99], 0);
    test(&[3,0,4,0,99],          302, &[302,0,4,0,99],        302);

    let inputs: Vec<_> = io::stdin().lock().lines().next().unwrap().unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut computer = IntcodeComputer::new(&inputs);
    let result = computer.run(1);
    println!("part 1 result: {}", result);
    assert_eq!(8332629, result);
}

fn test(program: &[i64], input: i64, expected_program: &[i64], expected_output: i64) {
    let mut computer = IntcodeComputer::new(program);
    let actual_output = computer.run(input);

    assert_eq!(expected_program, computer.program);
    assert_eq!(expected_output, actual_output);
}
