use crate::{Graph, part1};

pub fn calc(p0: &Graph) -> usize {
    println!(
        "{} {} {}",
        part1::calc(p0, &"svr".to_string(), &"dac".to_string()),
        part1::calc(p0, &"dac".to_string(), &"fft".to_string()),
        part1::calc(p0, &"fft".to_string(), &"out".to_string()),
    );

    println!(
        "{} {} {}",
        part1::calc(p0, &"svr".to_string(), &"fft".to_string()),
        part1::calc(p0, &"fft".to_string(), &"dac".to_string()),
        part1::calc(p0, &"dac".to_string(), &"out".to_string()),
    );

    part1::calc(p0, &"svr".to_string(), &"fft".to_string()) * part1::calc(p0, &"fft".to_string(), &"dac".to_string()) * part1::calc(p0, &"dac".to_string(), &"out".to_string())
}
