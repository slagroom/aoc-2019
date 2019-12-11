use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MemoryAccessError {
    address: usize,
}

impl MemoryAccessError {
    fn new(address: &usize) -> MemoryAccessError { 
        return MemoryAccessError { address: *address };
    }
}

impl fmt::Display for MemoryAccessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Invalid memory access @ {}", self.address);
    }
}

impl Error for MemoryAccessError {}

#[derive(Debug)]
struct InvalidInstructionError {
    instruction: isize,
    address: usize,
}

impl InvalidInstructionError {
    fn new (instruction: &isize, address: &usize) -> InvalidInstructionError {
        return InvalidInstructionError { instruction: *instruction, address: *address };
    }
}

impl fmt::Display for InvalidInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Invalid instruction {} @ {}", self.instruction, self.address);
    }
}

impl Error for InvalidInstructionError {}

struct IntcodeProcessor {
    program_counter: usize,
    memory: HashMap<usize, isize>,
}

impl IntcodeProcessor {

    fn new() -> IntcodeProcessor {
        return IntcodeProcessor { program_counter: 0, memory: HashMap::new() };
    }

    fn store(&mut self, address: usize, word: isize) {
        self.memory.insert(address, word);
    }

    fn fetch(&self, address: &usize) -> isize {
        return self.memory.get(address)
            .map(|v| *v)
            .ok_or(MemoryAccessError::new(address))
            .unwrap();
    }

    fn run(&mut self) {
        loop {
            match self.opcode() {
                1 => self.add(),
                2 => self.mul(),
                99 => break,
                x => return Err(InvalidInstructionError::new(&x, &self.program_counter)).unwrap(),
            }
        }
        println!("halt @ {}", self.program_counter);
    }

    fn opcode(&self) -> isize {
        return self.fetch(&self.program_counter) % 100;
    }

    fn add(&mut self) {
        
        let arg1 = self.fetch(&(self.fetch(&(self.program_counter + 1)) as usize));
        let arg2 = self.fetch(&(self.fetch(&(self.program_counter + 2)) as usize));
        let dst = self.fetch(&(self.program_counter + 3));

        self.store(dst as usize, arg1 + arg2);

        self.program_counter += 4;
    }

    fn mul(&mut self) {

        let arg1 = self.fetch(&(self.fetch(&(self.program_counter + 1)) as usize));
        let arg2 = self.fetch(&(self.fetch(&(self.program_counter + 2)) as usize));
        let dst = self.fetch(&(self.program_counter + 3));

        self.store(dst as usize, arg1 * arg2);

        self.program_counter += 4;
    }
}

fn main() {

    let mut computer = IntcodeProcessor::new();

    let program = vec![1,1,1,4,99,5,6,0,99];
    for (addr, word) in program.iter().enumerate() {
        computer.store(addr, *word);
    }
    
    computer.run();

    println!("{:?}", (0..computer.memory.len()).map(|addr| computer.memory[&addr]).collect::<Vec<_>>());
}

