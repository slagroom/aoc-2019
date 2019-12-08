use std::io;
use std::io::BufRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coordinate {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum Direction { Left, Right, Up, Down }

#[derive(Debug)]
struct Movement {
    direction: Direction,
    distance: u16,
}

impl Movement {
    fn apply(&self, to: &Coordinate) -> Coordinate {
        let m = self.distance as i64;
        return match self.direction {
            Direction::Left => Coordinate { x: to.x - m, .. *to },
            Direction::Right => Coordinate { x: to.x + m, .. *to },
            Direction::Up => Coordinate { y: to.y + m, .. *to },
            Direction::Down => Coordinate { y: to.y - m, .. *to },
        };
    }
}

#[derive(Debug)]
struct ParseMovementError {
    input: String,
}

impl ParseMovementError {
    fn new(input: &str) -> ParseMovementError {
        return ParseMovementError { input: input.to_string() };
    }
}

impl std::str::FromStr for Movement {

    type Err = ParseMovementError;

    fn from_str(s: &str) -> std::result::Result<Movement, ParseMovementError> {

        if s.len() < 2 {
            return Err(ParseMovementError::new(&s));
        }

        let direction = match s.chars().nth(0) {
            Some('L') => Ok(Direction::Left),
            Some('U') => Ok(Direction::Up),
            Some('R') => Ok(Direction::Right),
            Some('D') => Ok(Direction::Down),
            _ => Err(ParseMovementError::new(&s)),
        }?;

        let distance = match s[1..].parse::<u16>() {
            Ok(x) => Ok(x),
            _ => Err(ParseMovementError::new(&s)),
        }?;

        return Ok(Movement { direction: direction, distance: distance });
    }
}

fn read_input() -> Vec<Vec<Movement>> {

    fn parse_movement_vec(s: String) -> Result<Vec<Movement>, ParseMovementError> {
        return s.split(",")
            .map(|i| i.parse::<Movement>())
            .collect();
    }

    return io::stdin().lock()
        .lines()
        .map(|l| l.unwrap())
        .map(parse_movement_vec)
        .map(|m| m.unwrap())
        .collect();
}

struct Magic<'a, T, U> {
    current: Option<T>,
    input: &'a mut dyn Iterator<Item=U>,
    next_fn: fn(&T, &U) -> T,
}

impl<'a, T, U> Magic<'a, T, U> {
    fn new(
        first: T,
        input: &'a mut dyn Iterator<Item=U>,
        next: fn(&T, &U) -> T
    ) -> Magic<T, U> {
        return Magic {
            current: Some(first),
            input: input,
            next_fn: next,
        };
    }
}

impl<'a, T, U> Iterator for Magic<'a, T, U> {

    type Item = T;

    fn next(&mut self) -> Option<T> {
        let ret = self.current.take()?;
        self.current = match self.input.next() {
            Some(u) => Some((self.next_fn)(&ret, &u)),
            None => None,
        };
        return Some(ret);
    }
}

trait MagicIter<'a, T, U> {
    fn magic(&'a mut self, T, fn(&T, &U) -> T) -> Magic<'a, T, U>;
}

impl<'a, T, U, I> MagicIter<'a, T, U> for I where I: Iterator<Item=U> {
    fn magic(&'a mut self, t: T, next: fn(&T, &U) -> T) -> Magic<'a, T, U> {
        return Magic::new(t, self, next);
    }
}

#[derive(Debug)]
struct LineSegment {
    start: Coordinate,
    end: Coordinate,
}

#[derive(Debug)]
struct InvalidLineSegmentError {
    start: Coordinate,
    end: Coordinate,
}

impl InvalidLineSegmentError {
    fn new(start: Coordinate, end: Coordinate) -> InvalidLineSegmentError {
        return InvalidLineSegmentError { start, end };
    }
}

impl LineSegment {
    fn new(start: Coordinate, end: Coordinate) -> Result<LineSegment, InvalidLineSegmentError> {

        if start == end {
            return Err(InvalidLineSegmentError::new(start, end));
        }

        if ! ((start.x == end.x) || (start.y == end.y)) {
            return Err(InvalidLineSegmentError::new(start, end));
        }

        return Ok(LineSegment { start, end });
    }
}

fn intersection(a: &LineSegment, b: &LineSegment) -> Option<Coordinate> {

    fn is_vertical(ls: &LineSegment) -> bool { ls.start.x == ls.end.x }

    let (v, h) = match (is_vertical(a), is_vertical(b)) {
        (true, false) => Some((a, b)),
        (false, true) => Some((b, a)),
        _ => None,
    }?;

    fn sorted(arr: &mut [i64; 3]) -> [i64; 3] {
        arr.sort();
        return *arr;
    }

    let x_coords = sorted(&mut [ h.start.x, v.start.x, h.end.x ]);
    let y_coords = sorted(&mut [ v.start.y, h.start.y, v.end.y ]);

    if x_coords[1] == v.start.x && y_coords[1] == h.start.y {
        return Some(Coordinate { x: v.start.x, y: h.start.y });
    }

    return None;
}

fn main() {

    let paths = read_input().iter()
        .map(|p| {
            p.iter()
                .magic(Coordinate { x: 0, y: 0 }, | c, m | m.apply(c))
                .collect::<Vec<Coordinate>>()
                .windows(2)
                .map(|w| LineSegment::new(w[0], w[1]).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let [first, second] = match &paths[..] {
        [a, b] => [a, b],
        _ => panic!("expected 2 paths from input"),
    };

    fn rectilinear_distance(c: Coordinate) -> u64 {
        return (c.x.abs() + c.y.abs()) as u64;
    }

    let intersections = first.iter()
        .flat_map(|a| second.iter().map(move |b| intersection(a,b)))
        .filter_map(|x| x)
        .map(rectilinear_distance)
        .filter(|x| x != &0u64);

    let closest = intersections
        .min()
        .unwrap();

    println!("part 1: {}", closest);
}