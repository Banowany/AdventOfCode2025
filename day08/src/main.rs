mod part1;
mod part2;

use regex::Regex;
use std::fs;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i128,
    y: i128,
    z: i128
}

impl Point {
    fn new(x: i128, y: i128, z: i128) -> Point {
        Point {
            x, y, z
        }
    }

    fn count_distance(&self, point: &Point) -> i128 {
        let mut res: i128 = 0;

        res += (self.x - point.x)*(self.x - point.x);
        res += (self.y - point.y)*(self.y - point.y);
        res += (self.z - point.z)*(self.z - point.z);

        res
    }
}

fn main() {
    let input: String = fs::read_to_string("input").expect("Error during reading the file");

    let re = Regex::new(r"(\d+),(\d+),(\d+)").unwrap();

    let mut points: Vec<Point> = Vec::new();

    for line in input.lines() {
        let capture = re.captures(line).unwrap();
        let x: i128 = capture[1].parse().unwrap();
        let y: i128 = capture[2].parse().unwrap();
        let z: i128 = capture[3].parse().unwrap();

        points.push(Point::new(x,y,z));
    }

    println!("{}", part1::calc(&points, 1000));
    println!("{}", part2::calc(&points));
}
