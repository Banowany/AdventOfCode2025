mod part1;
mod part2;

use std::fs;

fn main() {
    let input = fs::read_to_string("input")
        .expect("Error during reading the file");

    let mut rotations:Vec<i32> = Vec::new();

    for line in input.lines() {
        let direction: String = line.chars().take(1).collect();
        let mut value: i32 = line.chars().skip(1).collect::<String>().parse().expect("Something gone wrong during number parisng");

        if direction == "L" {
            value *= -1;
        }

        rotations.push(value);
    }

    println!("{}", part1::calc(&rotations));
    println!("{}", part2::calc(&rotations));
    // println!("{:?}", rotations);
}
