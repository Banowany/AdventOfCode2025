mod part1;
mod part2;

use std::fs;

fn main() {
    let input = fs::read_to_string("input")
        .expect("Error during reading the file");

    let paper_map:Vec<Vec<char>> = input.lines().map(|x| {x.chars().collect()}).collect();

    println!("{}", part1::calc(&paper_map));
    println!("{}", part2::calc(&paper_map));
}
