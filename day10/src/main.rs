mod part1;
mod part2;

use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    light: Vec<usize>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>
}

fn convert_light_input(light_input: &str) -> Vec<usize> {
    let n = light_input.len();
    let without_brackets = &light_input[1..n-1];

    let mut converted: Vec<usize> = Vec::new();
    for sign in without_brackets.chars() {
        if sign == '#' {
            converted.push(1);
        }
        else {
            converted.push(0);
        }
    }

    converted
}

fn convert_button_input(button_input: &str) -> Vec<usize> {
    let mut converted: Vec<usize> = Vec::new();

    let re = Regex::new(r"(\d+)").unwrap();

    for mat in re.find_iter(button_input) {
        let num: usize = mat.as_str().parse().unwrap();
        converted.push(num)
    }

    converted
}

fn convert_joltage_input(button_input: &str) -> Vec<usize> {
    let mut converted: Vec<usize> = Vec::new();

    let re = Regex::new(r"(\d+)").unwrap();

    for mat in re.find_iter(button_input) {
        let num: usize = mat.as_str().parse().unwrap();
        converted.push(num)
    }

    converted
}

fn main() {
    let input: String = fs::read_to_string("input").expect("Error during reading the file");

    let mut machines: Vec<Machine> = Vec::new();
    for line in input.lines() {
        let mut light: Vec<usize> = Vec::new();
        let mut buttons: Vec<Vec<usize>> = Vec::new();
        let mut joltage: Vec<usize> = Vec::new();
        for el in line.split_whitespace() {
            if el.chars().nth(0).unwrap() == '[' {
                light = convert_light_input(el);
            }
            else if el.chars().nth(0).unwrap() == '(' {
                buttons.push(convert_button_input(el));
            }
            else {
                joltage = convert_joltage_input(el);
            }
        }
        machines.push(Machine { light, buttons, joltage });
    }

    // println!("{:?}", machines);
    // println!("{}", part1::calc(&machines));
    println!("{}", part2::calc(&machines));
}
