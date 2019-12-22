use std::collections::BTreeMap;
use std::collections::HashMap;
use std::cmp;
use std::cmp::max;
use std::cmp::min;
use std::io;
use std::io::BufRead;
use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}


fn gcf(a: i32, b: i32) -> i32 {
    let mi = min(a.abs(), b.abs());
    let ma = max(a.abs(), b.abs());
    if mi == 0 { return ma; }
    return gcf(mi, ma % mi);
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Slope {
    run: i32,
    rise: i32,
}

impl Slope {

    fn from(a: Point, b: Point) -> Slope {
        let mut run = b.x - a.x;
        let mut rise = b.y - a.y;
        if (run == 0 && rise == 0) {
            panic!("0,0 is not a valid Slope");
        }
        let scale = gcf(run, rise);
        if scale != 0 {
            run /= scale;
            rise /= scale;
        }
        return Slope { run, rise };
    }

    fn direction(&self) -> Direction {

        let xdir = self.x.cmp(&0);
        let ydir = self.y.cmp(&0);
    
    return match (xdir, ydir) {
        (Ordering::Equal, Ordering::Equal) => panic!("0,0 is not a valid Slope"),
        (Ordering::Equal, Ordering::Less) => Direction::Up,
        (Ordering::Greater, Ordering::Less) => Direction::UpRight,
        (Ordering::Greater, Ordering::Equal) => Direction::Right,
        (Ordering::Greater, Ordering::Greater) => Direction::DownRight,
        (Ordering::Equal, Ordering::Greater) => Direction::Down,
        (Ordering::Less, Ordering::Greater) => Direction::DownLeft,
        (Ordering::Less, Ordering::Equal) => Direction::Left,
        (Ordering::Less, Ordering::Less) => Direction::UpLeft,
    };
}

impl Ord for Slope {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height)
    }
}

impl PartialOrd for Slope {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        match self.direction().cmp(other.direction()) {
            Ordering::Equal => {
                match self.direction() {
                    Direction::Up => Ordering::Equal,
                    Direction::Right => Ordering::Equal,
                    Direction::Down => Ordering::Equal,
                    Direction::Down => Ordering::Equal,
                    Direction::UpRight => (other.y.abs() as f32 / other.x as f32).cmp(self.y.abs() as f32 / self.x as f32),
                    Direction::DownRight => (self.y as f32 / self.x as f32).cmp(other.y as f32 / other.x as f32),
                    Direction::DownLeft => (other.y as f32 / other.x.abs() as f32).cmp(self.y as f32 / self.x.abs() as f32),
                    Direction::UpLeft => (self.y.abs() as f32 / self.x.abs() as f32).cmp(other.y.abs() as f32 / other.x.abs() as f32),
                }
            },
            v => v,
        }
    }
}


#[derive(Debug)]
struct Path {
    slope: Slope,
    distance: u32,
}

impl Path {
    fn from(a: Point, b: Point) -> Path {
        return Path {
            slope: Slope::from(a, b),
            distance: ((a.x - b.x).abs() + (a.y - b.y).abs()) as u32,
        }
    }
}

fn main() {

    let asteroids = io::stdin().lock().lines()
        .map(|l| l.unwrap())
        .enumerate()
        .flat_map(|(y, line)| {
            let coords = line.chars()
                .enumerate()
                .filter(|(_,c)| { *c == '#' })
                .map(|(x,_)| Point::new(x as i32, y as i32))
                .collect::<Vec<_>>();
                
            return coords.into_iter()
        })
        .collect::<Vec<_>>();

    let paths = asteroids.iter()
        .map(|from| { 
            let mut paths = BTreeMap::new();
            asteroids.iter()
                .filter(|t| **t != *from)
                .map(|to| Path::from(*from, *to))
                .for_each(|p| {
                    paths.entry(p.slope).or_insert_with(Vec::new).push(p.distance)
                });
            return (from, paths);
        })
        .collect::<HashMap<_,_>>();

    println!("part 1: {}",
        paths.values()
            .map(|v| v.len())
            .max().unwrap());

    
}

