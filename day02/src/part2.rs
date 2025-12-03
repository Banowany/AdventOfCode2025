pub fn get_dividers(num: i64) -> Vec<i64> {
    let mut dividers: Vec<i64> = Vec::new();
    let mut i: i64 = 1;

    while i * i <= num {
        if num % i == 0 {
            dividers.push(i);
            if i != num / i && i != 1 {
                dividers.push(num / i)
            }
        }
        i += 1;
    }

    return dividers;
}

pub fn is_invalid_for_specific_pattern_length(num: &String, pattern_length: usize) -> bool {
    let how_much_parts: usize = num.chars().count() / pattern_length;
    let mut parts: Vec<&str> = Vec::new();

    for i in 0..how_much_parts {
        parts.push(&num[(i * pattern_length)..((i + 1) * pattern_length)])
    }

    return parts.windows(2).all(|x| x[0] == x[1]);
}

fn is_invalid(num: i64) -> bool {
    let num_as_string = num.to_string();
    let length = num_as_string.chars().count();
    let possible_patterns_lengths = get_dividers(length as i64);

    return possible_patterns_lengths
        .iter()
        .map(|&x| is_invalid_for_specific_pattern_length(&num_as_string, x as usize))
        .any(|x1| x1 == true);
}

pub fn calc(ranges: &Vec<(i64, i64)>) -> i128 {
    let mut res: i128 = 0;

    for &(b, e) in ranges {
        for x in b..e + 1 {
            if is_invalid(x) && x >= 10 {
                res += x as i128;
            }
        }
    }

    return res;
}
