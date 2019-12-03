use std::io::prelude::*;
use std::io;
use std::str::FromStr;

fn fuel_req(mass: i64) -> i64 {
    let fuel = (mass / 3) - 2;
    if fuel <= 0 {
        return 0
    }
    return fuel + fuel_req(fuel)
}

fn main() {

    println!("{}", io::stdin().lock().lines()
        .map(|line| i64::from_str(&line.unwrap()).unwrap())
        .map(|mass| fuel_req(mass))
        .sum::<i64>());
}
