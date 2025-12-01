fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

pub fn calc(rotations: &Vec<i32>) -> i32 {
    let mut curr = 50;
    let mut res = 0;

    for &rotate in rotations.iter() {
        let full_rotations:i32 = (rotate/100).abs();
        res += full_rotations;
        let normalized_ration:i32 = rotate%100;

        if curr != 0 {
            curr = curr + normalized_ration;
            if curr <= 0 || curr >= 100 {
                res += 1;
            }
            curr = modulo(curr, 100);
        }
        else {
            curr = curr + normalized_ration;
            curr = modulo(curr, 100);
        }
    }

    return res;
}