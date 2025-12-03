mod part1;
mod part2;

use std::fs;

fn main() {
    let input = fs::read_to_string("input")
        .expect("Error during reading the file");

    let mut banks:Vec<Vec<i32>> = Vec::new();

    for line in input.lines() {
        let bank: Vec<i32> = line.chars().map(|x| {(x as i32) - ('0' as i32)}).collect();
        banks.push(bank);
    }

    println!("{}", part1::calc(&banks));
    println!("{}", part2::calc(&banks));
}
