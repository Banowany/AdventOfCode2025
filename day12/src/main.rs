use std::fs;

#[derive(Debug)]
struct Region {
    row_size: usize,
    col_size: usize,
    needed_shapes: Vec<usize>
}

impl Region {
    fn parse_line(line: &str) -> Region {
        let parts: Vec<&str> = line.split(": ").collect();
        let row_size: usize = parts[0].split("x").nth(0).unwrap().parse().unwrap();
        let col_size: usize = parts[0].split("x").nth(1).unwrap().parse().unwrap();

        let mut needed_shapes: Vec<usize> = Vec::new();
        for x in parts[1].split_whitespace() {
            needed_shapes.push(x.parse().unwrap());
        }

        Region {
            row_size,
            col_size,
            needed_shapes
        }
    }

    fn is_maybe_fit(&self) -> bool {
        let row_possible: usize = self.row_size / 3;
        let col_possible: usize = self.col_size / 3;

        return self.needed_shapes.iter().sum::<usize>() <= row_possible*col_possible;
    }
}

fn main() {
    let regions_input: String = fs::read_to_string("input_regions").expect("Error during reading the file");

    let regions: Vec<Region> = regions_input.lines().map(|x| {Region::parse_line(x)}).collect();

    let how_many_maybe_fit = regions.iter().filter(|&x1| {x1.is_maybe_fit()}).count();

    println!("{}", how_many_maybe_fit);
}
