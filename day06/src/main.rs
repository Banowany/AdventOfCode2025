use std::fs;
use regex::Regex;

mod part1;
mod part2;
fn main() {
    let input: String = fs::read_to_string("input").expect("Error during reading the file");

    let lines: Vec<&str> = input.lines().collect();
    let elements: Vec<Vec<&str>> = lines
        .iter()
        .map(|&x| x.split_whitespace().collect())
        .collect();

    // println!("{:?}", elements);
    println!("{}", part1::calc(&elements));

    let mut columns: Vec<String> = Vec::new();
    let max_line_length = lines.iter().map(|&x1| x1.len()).max().unwrap();
    for i in 0..max_line_length {
        let column: String = lines
            .iter()
            .map(|&x2| {
                let sign = x2.chars()
                    .nth(i)
                    .unwrap_or(' ');
                if sign == '+' || sign == '*' {' '} else {sign}
            })
            .collect();
        columns.push(column);
    }
    columns.push(String::from(" ")); //additional empty column for easier parsing

    let mut operation_data: Vec<Vec<usize>> = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    let mut stack: Vec<usize> = Vec::new();
    for col in columns {
        match re.find(&col) {
            Some(mat) => {
                stack.push(mat.as_str().parse().expect("column parsing"))
            }
            None => {
                operation_data.push(stack);
                stack = Vec::new();
            }
        }
    }

    println!("{}", part2::calc(&operation_data, elements.last().unwrap()));
}
