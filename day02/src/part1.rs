fn is_invalid(num: i64) -> bool {
    let num_as_string = num.to_string();
    let length = num_as_string.chars().count();

    if length % 2 == 1 {
        return false;
    }

    let first_part: String = num_as_string.chars().take(length/2).collect();
    let second_part: String = num_as_string.chars().skip(length/2).collect();
    return first_part == second_part;
}

pub fn calc(ranges: &Vec<(i64, i64)>) -> i64 {
    let mut res = 0;

    for &(b, e) in ranges {
        for x in b..e+1 {
            if is_invalid(x) {
                res += x;
            }
        }
    }

    return res;
}