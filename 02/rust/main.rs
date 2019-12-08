use std::io;
use std::io::Read;

fn read_input() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.split(",")
        .map(|t| t.parse::<usize>().unwrap())
        .collect();
}

struct IntcodeProcessor {
    pc: usize,
    memory: Vec<usize>,
}

impl IntcodeProcessor {

    fn new(memory: &Vec<usize>) -> IntcodeProcessor {
        return IntcodeProcessor { pc: 0, memory: memory.to_vec() };
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

fn part_1(memory_layout: &Vec<usize>) -> usize {

    let mut computer = IntcodeProcessor::new(&memory_layout);

    // restore "1202 program alarm" state
    computer.memory[1] = 12;
    computer.memory[2] = 2;

    computer.run();

    return computer.memory[0];
}

fn part_2(memory_layout: &Vec<usize>) -> usize {

    let target = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {

            let mut computer = IntcodeProcessor::new(&memory_layout);

            computer.memory[1] = noun;
            computer.memory[2] = verb;

            computer.run();

            if computer.memory[0] == target {
                return (100 * noun) + verb;
            }
        }
    }

    panic!("no noun/verb combo found");
}

fn main() {

    let memory_layout = read_input();

    let part1 = part_1(&memory_layout);
    let part2 = part_2(&memory_layout);

    println!("part 1: {}", part1);

    println!("part 2: {}", part2);
}