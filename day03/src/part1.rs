use std::cmp::max;

pub fn calc(banks: &Vec<Vec<i32>>) -> i32 {
    let mut res = 0;

    for bank in banks {
        let mut biggest = 0;
        let n = bank.len();
        for i in 0..n {
            for j in (i+1)..n {
                biggest = max(biggest, bank[i]*10 + bank[j]);
            }
        }
        res += biggest;
    }

    return res;
}