mod part1;
mod part2;
// mod part2;

use std::fs;

fn main() {
    let input = fs::read_to_string("input")
        .expect("Error during reading the file");

    let mut ranges:Vec<(i64, i64)> = Vec::new();

    for string_range in input.split(',') {
        let range_string_nums:Vec<&str> = string_range.split("-").collect();
        let begin:i64 = range_string_nums.get(0).unwrap().parse().expect("Problem with translating begin of range");
        let end:i64 = range_string_nums.get(1).unwrap().parse().expect("Problem with translating begin of range");

        ranges.push((begin, end));
    }

    println!("{}", part1::calc(&ranges));
    println!("{}", part2::calc(&ranges));
    // println!("{:?}", part2::get_dividers(128));
    // println!("{:?}", part2::get_dividers(9));
    // println!("{:?}", part2::get_dividers(6));
    // println!("{:?}", part2::get_dividers(2));
    // println!("{}", part2::is_invalid_for_specific_pattern_length(&String::from("abcabcabd"), 3))
}
