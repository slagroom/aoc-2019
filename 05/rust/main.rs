use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;
use std::io::BufRead;

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
    read: fn() -> isize,
    write: fn(isize),
}

impl IntcodeProcessor {

    fn new(read: fn() -> isize, write: fn(isize)) -> IntcodeProcessor {
        return IntcodeProcessor { 
            program_counter: 0,
            memory: HashMap::new(),
            read,
            write,
        };
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
                3 => self.input(),
                4 => self.output(),
                99 => break,
                x => return Err(InvalidInstructionError::new(&x, &self.program_counter)).unwrap(),
            }
        }
    }

    fn instruction(&self) -> isize {
        return self.fetch(&self.program_counter);
    }

    fn opcode(&self) -> isize {
        return self.fetch(&self.program_counter) % 100;
    }

    fn add(&mut self) {
        
        let arg1 = self.val_arg(1);
        let arg2 = self.val_arg(2);
        let dst = self.ref_arg(3);

        self.store(dst as usize, arg1 + arg2);

        self.program_counter += 4;
    }

    fn mul(&mut self) {
        
        let arg1 = self.val_arg(1);
        let arg2 = self.val_arg(2);
        let dst = self.ref_arg(3);

        self.store(dst as usize, arg1 * arg2);

        self.program_counter += 4;
    }

    fn input(&mut self) {

        let dst = self.ref_arg(1);

        self.store(dst as usize, (self.read)());

        self.program_counter += 2;
    }

    fn output(&mut self) {

        let arg = self.val_arg(1);

        (self.write)(arg);

        self.program_counter += 2;
    }

    fn val_arg(&self, position: usize) -> isize {
        let value = self.fetch(&(self.program_counter + position));
        let mode = self.arg_mode(position);
        let arg = match mode {
            0 => self.fetch(&(value as usize)), // position mode
            1 => value, // immediate mode
            x => panic!("invalid parameter mode {} (@ {})", x, self.program_counter),
        };
        return arg;
    }

    fn ref_arg(&self, position: usize) -> isize {
        return match self.arg_mode(position) {
            0 => self.fetch(&(self.program_counter + position)), // position mode
            x => panic!("invalid parameter mode {} for ref arg (@ {})", x, self.program_counter),
        }
    }

    fn arg_mode(&self, position: usize) -> usize {
        let mode_mask = 10_usize.pow(position as u32 + 1);
        let shifted = self.instruction() as usize / mode_mask;
        let mode = shifted % 10;
        return mode;
    }
}

#[derive(Debug)]
struct InputError {
    data: String,
}

impl InputError {
    fn new (data: &str) -> InputError {
        return InputError { data: data.to_string() };
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Invalid input: {}", self.data);
    }
}

impl Error for InputError {}

fn main() {

    fn input() -> isize {
        return io::stdin().lock().lines().next().unwrap().unwrap()
            .parse::<isize>()
            .or_else(|err| Err(InputError::new(&format!("{:?}", err))))
            .unwrap();
    }

    fn output(val: isize) {
        println!("{}", val);
    }

    let mut computer = IntcodeProcessor::new(input, output);

    io::stdin().lock().lines().next().unwrap().unwrap()
        .split(",")
        .map(|w| w.parse::<isize>().unwrap())
        .enumerate()
        .for_each(|(addr,word)| {
            computer.store(addr, word);
        });
    
    computer.run();
}

