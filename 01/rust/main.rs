use std::io::prelude::*;
use std::io;

fn read_input() -> Vec<i64> {
    return io::stdin().lock()
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap())
        .collect::<Vec<_>>();
}

fn fuel_req(mass: &i64) -> i64 { (mass / 3) - 2 }

fn fuel_req_recursive(mass: &i64) -> i64 { 
    return match fuel_req(mass) {
        x if x <= 0 => 0,
        v => v + fuel_req_recursive(&v),
    };
}

fn total_fuel_req(masses: &Vec<i64>, fuel_fn: fn(&i64) -> i64) -> i64 {
    return masses.iter()
        .map(fuel_fn)
        .sum();
}

fn main() {

    let masses = read_input();

    let part1 = total_fuel_req(&masses, fuel_req);

    let part2 = total_fuel_req(&masses, fuel_req_recursive);

    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}