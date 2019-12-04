use std::io;
use std::io::Read;
use std::str::FromStr;

struct IntcodeMachine {
    pc: usize,
    memory: Vec<usize>,
}

impl IntcodeMachine {

    fn new(memory: Vec<usize>) -> IntcodeMachine {
        return IntcodeMachine { pc: 0, memory: memory };
    }

    fn run(&mut self) {

        const ADD: usize = 1;
        const MUL: usize = 2;
        const HALT: usize = 99;

        while self.memory[self.pc] != HALT {

            let arg1 = self.memory[self.memory[self.pc + 1]];
            let arg2 = self.memory[self.memory[self.pc + 2]];
            let out = self.memory[self.pc + 3];

            match self.memory[self.pc] {
                ADD => self.memory[out] = arg1 + arg2,
                MUL => self.memory[out] = arg1 * arg2,
                x => panic!("invalid instruction {}", x)
            }

            self.pc += 4;
        }
    }
}

fn main() {

    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();

    let program: Vec<usize> = input.split(",")
        .map(|t| usize::from_str(&t).unwrap())
        .collect();

    let target = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {

            let mut computer = IntcodeMachine::new(program.to_vec());

            computer.memory[1] = noun;
            computer.memory[2] = verb;

            computer.run();

            if computer.memory[0] == target {
                println!("{}", 100 * noun + verb);
                return
            }
        }
    }

    panic!("no noun/verb combo found");
}