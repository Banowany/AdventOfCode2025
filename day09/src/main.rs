mod part1;
mod part2;

use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Point {
    col: i128,
    row: i128
}

impl Point {
    fn new(x: i128, y: i128) -> Point {
        Point {
            col: x,
            row: y
        }
    }

    fn filed_with(&self, point: &Point) -> i128 {
        let x_diff = (self.col - point.col).abs() + 1;
        let y_diff = (self.row - point.row).abs() + 1;
        x_diff * y_diff
    }
}

fn main() {
    let input: String = fs::read_to_string("input").expect("Error during reading the file");

    let re = Regex::new(r"(\d+),(\d+)").unwrap();

    let mut points: Vec<Point> = Vec::new();

    for line in input.lines() {
        let capture = re.captures(line).unwrap();
        let x: i128 = capture[1].parse().unwrap();
        let y: i128 = capture[2].parse().unwrap();

        points.push(Point::new(x,y));
    }
    println!("{}", part1::calc(&points));
    println!("{}", part2::calc(&points));
}
