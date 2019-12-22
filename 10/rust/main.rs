use std::collections::HashMap;
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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Slope {
    run: i32,
    rise: i32,
}

impl Slope {
    fn from(a: Point, b: Point) -> Slope {
        let mut run = b.x - a.x;
        let mut rise = b.y - a.y;
        let scale = gcf(run, rise);
        if scale != 0 {
            run /= scale;
            rise /= scale;
        }
        return Slope { run, rise };
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
            let mut paths = HashMap::new();
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

