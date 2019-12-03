use std::io::prelude::*;
use std::io;
use std::str::FromStr;

fn main() {

    println!("{}", io::stdin().lock().lines()
        .map(|line| i64::from_str(&line.unwrap()).unwrap())
        .map(|mass| (mass / 3) - 2)
        .sum::<i64>());
}
