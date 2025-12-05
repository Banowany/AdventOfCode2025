use crate::IdRange;

pub fn calc(ranges: &Vec<IdRange>) -> usize {
    let mut ranges2: Vec<IdRange> = ranges.clone();
    ranges2.sort_by_key(|x| {x.begin});

    let mut normalized_ranges: Vec<IdRange> = Vec::new();

    for el in ranges2 {
        if normalized_ranges.is_empty() {
            normalized_ranges.push(el);
        }
        else {
            let last_range = normalized_ranges.pop().unwrap();
            let result_of_connection = IdRange::connect_if_intersection(last_range, el);
            normalized_ranges.extend(result_of_connection);
        }
    }
    return normalized_ranges.iter().map(|x1| {x1.count()}).sum();
}