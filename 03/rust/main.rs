use std::collections::HashMap;
use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum Direction { Left, Right, Up, Down }

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: u16,
}

impl Instruction {
    fn steps(&self) -> Steps {
        return Steps::new(self.distance, self.direction);
    }
}

impl std::str::FromStr for Instruction {

    type Err = ParseInstructionError;

    fn from_str(s: &str) -> std::result::Result<Instruction, ParseInstructionError> {

        if s.len() < 2 {
            return Err(ParseInstructionError::new(&s));
        }

        let direction = match s.chars().nth(0) {
            Some('L') => Ok(Direction::Left),
            Some('U') => Ok(Direction::Up),
            Some('R') => Ok(Direction::Right),
            Some('D') => Ok(Direction::Down),
            _ => Err(ParseInstructionError::new(&s)),
        }?;

        let distance = match s[1..].parse::<u16>() {
            Ok(x) => Ok(x),
            _ => Err(ParseInstructionError::new(&s)),
        }?;

        return Ok(Instruction { direction: direction, distance: distance });
    }
}

#[derive(Debug)]
struct ParseInstructionError {
    input: String,
}

impl ParseInstructionError {
    fn new(input: &str) -> ParseInstructionError {
        return ParseInstructionError { input: input.to_string() };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    fn new(x: i64, y: i64) -> Coordinate {
        return Coordinate { x, y };
    }
}

type Step = fn(from: &Coordinate) -> Coordinate;

struct Steps {
    steps: u16,
    direction: Direction,
}

impl Steps {
    fn new(steps: u16, direction: Direction) -> Steps {
        return Steps { steps, direction };
    }
}

impl Iterator for Steps {
    type Item = Step;
    fn next(&mut self) -> Option<Step> {
        if self.steps == 0 {
            return None;
        }
        self.steps -= 1;
        return match self.direction {
            Direction::Left => Some(|c| Coordinate::new(c.x - 1, c.y)),
            Direction::Right => Some(|c| Coordinate::new(c.x + 1, c.y)),
            Direction::Up => Some(|c| Coordinate::new(c.x, c.y + 1)),
            Direction::Down => Some(|c| Coordinate::new(c.x, c.y - 1)),
        };
    }
}

fn read_input() -> Vec<Vec<Instruction>> {

    fn parse_instruction_vec(s: String) -> Result<Vec<Instruction>, ParseInstructionError> {
        return s.split(",")
            .map(|i| i.parse::<Instruction>())
            .collect();
    }

    return io::stdin().lock()
        .lines()
        .map(|l| l.unwrap())
        .map(parse_instruction_vec)
        .map(|m| m.unwrap())
        .collect();
}

fn coordinates<'a>(path: &'a Vec<Instruction>) -> HashMap<Coordinate, usize> {

    let mut coords = HashMap::new();

    path.iter()
        .flat_map(|m| m.steps())
        .scan(Coordinate::new(0, 0), | coord, step_fn | {
            *coord = step_fn(coord);
            return Some(*coord)
        })
        .enumerate()
        .for_each(|(v, k)| {
            if !coords.contains_key(&k) {
                coords.insert(k, v);
            }
        });

    return coords;
}

fn main() {

    let paths = read_input();

    let [first, second] = match &paths[..] {
        [a, b] => [coordinates(a), coordinates(b)],
        _ => panic!("expected 2 paths from input"),
    };

    let mut intersections = HashMap::new();

    for (key, val) in first {
        if let Some(v) = second.get(&key) {
            intersections.insert(key, val + v + 2);
        }
    }

    let part1 = intersections.keys()
        .map(|k| k.x + k.y)
        .min()
        .unwrap();

    let part2 = intersections.values().min().unwrap();

    println!("part 1: {}", part1);

    println!("part 2: {}", part2);
}