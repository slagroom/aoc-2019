use std::collections::HashMap;
use std::io;
use std::io::BufRead;

fn main() {

    let input = io::stdin().lock().lines().next().unwrap().unwrap()
        .split("-")
        .take(2)
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let start = input[0];
    let end = input[1];

    let possible_passwords = (start..end)
        .map(|n| n.to_string())
        .filter(|password| {
            let mut chars: Vec<char> = password.chars().collect();
            chars.sort();
            let sorted: String = chars.iter().collect();
            return *password == sorted;
        })
        .map(|password| {
            password.chars()
                .map(|c| {
                    (c, password.chars().filter(|x| *x == c).count())
                })
                .collect::<HashMap<char, usize>>()
        })
        .collect::<Vec<_>>();

    let part1 = possible_passwords
        .iter()
        .filter(|password| {
            password.iter().count() < 6
        })
        .count();

    let part2 = possible_passwords
        .iter()
        .filter(|password| {
            password.iter().filter(|(_, v)| **v == 2 as usize).count() > 0
        })
        .count();

    println!("part 1: {}", part1);

    println!("part 2: {}", part2);
}