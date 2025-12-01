mod part1;
mod part2;

use std::fs;

fn main() {
    let input = fs::read_to_string("input")
        .expect("Error during reading the file");

    let mut vec1:Vec<i32> = Vec::new();
    let mut vec2:Vec<i32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let num1: i32 = parts[0].parse().expect("First number do not parse");
        let num2: i32 = parts[1].parse().expect("Second number do not parse");

        vec1.push(num1);
        vec2.push(num2);
    }

    println!("{}", part1::calculate(vec1.clone(), vec2.clone()));
    println!("{}", part2::calculate(vec1.clone(), vec2.clone()));
}
