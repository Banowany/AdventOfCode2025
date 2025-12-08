mod part2v2;
use std::collections::HashMap;
use std::fs;
use regex::Regex;

mod part1;
mod part2;

// mod part2;
fn main() {
    let input: String = fs::read_to_string("input").expect("Error during reading the file");

    // key: col, val: rows
    let mut splitters: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut start: (usize, usize) = (0, 0);

    for (i, line) in input.lines().enumerate() {
        for (j, field) in line.chars().enumerate() {
            match field {
                '^' => {
                    splitters.entry(j).and_modify(|x| {x.push(i)}).or_insert(vec![i]);
                }

                'S' => {
                    start = (i, j);
                }

                _ => {}
            }
        }
    }

    // println!("{:?}", start);
    // println!("{:?}", splitters);

    // println!("{}", part1::calc(&splitters, start));
    println!("{}", part2v2::calc(&input.lines().collect::<Vec<&str>>(), start));
    // println!("{}", part2::calc(&operation_data, elements.last().unwrap()));
}
