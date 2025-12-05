mod part1;
mod part2;

use std::cmp::max;
use std::fs;

#[derive(Clone, Copy)]
struct IdRange {
    begin: usize,
    end: usize,
}

impl IdRange {
    fn from_str(s: &str) -> IdRange {
        let parts: Vec<&str> = s.split('-').collect();
        return IdRange {
            begin: parts[0]
                .parse()
                .expect("Error, during translating start in range as &str"),
            end: parts[1]
                .parse()
                .expect("Error, during translating end in range as &str"),
        };
    }

    fn is_contains(&self, el: usize) -> bool {
        return self.begin <= el && el <= self.end;
    }

    fn connect_if_intersection(el1: IdRange, el2: IdRange) -> Vec<IdRange> {
        if el1.begin <= el2.begin && el2.begin <= el1.end {
            return vec![
                IdRange {
                    begin: el1.begin,
                    end: max(el1.end, el2.end)
                }
            ]
        }
        return vec![el1, el2];
    }

    fn count(&self) -> usize {
        return self.end - self.begin + 1;
    }
}
fn main() {
    let ranges: Vec<IdRange> = fs::read_to_string("input-ranges")
        .expect("Error during reading the file")
        .lines()
        .map(|x1| IdRange::from_str(x1))
        .collect();

    let ids: Vec<usize> = fs::read_to_string("input-elements")
        .expect("Error during reading the file")
        .lines()
        .map(|x1| x1.parse().expect("Error during translating id"))
        .collect();

    println!("{}", part1::calc(&ranges, &ids));
    println!("{}", part2::calc(&ranges));
}
