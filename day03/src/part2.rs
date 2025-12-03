fn convert_vec_to_num(vec: &Vec<i32>, skip: i32) -> i64 {
    let mut res: i64 = 0;

    for (i, &x) in vec.iter().enumerate() {
        if (i as i32) != skip {
            res = res*10 + (x as i64);
        }
    }

    return res;
}

fn get_bigger_number_combination(vec: Vec<i32>, candidate: i32) -> Vec<i32> {
    let mut biggest: Vec<i32> = vec;

    let mut another: Vec<i32> = biggest.iter().copied().collect();
    another.push(candidate);

    for i in 0..another.len() {
        if convert_vec_to_num(&biggest, -1) < convert_vec_to_num(&another, i as i32) {
            biggest = another.iter().enumerate().filter(|&(j, _)| {j != i}).map(|(_, &val)|{val}).collect();
        }
    }

    return biggest;
}

pub fn calc(banks: &Vec<Vec<i32>>) -> i128 {
    let mut res = 0;

    for bank in banks {
        let mut curr: Vec<i32> = bank.iter().take(12).copied().collect();
        for i in 12..bank.len() {
            curr = get_bigger_number_combination(curr, bank[i]);
        }
        let num = convert_vec_to_num(&curr, -1) as i128;

        // println!("{}", num);

        res += num

    }

    return res;
}