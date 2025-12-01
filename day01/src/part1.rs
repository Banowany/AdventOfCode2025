fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

pub fn calc(rotations: &Vec<i32>) -> i32 {
    let mut curr = 50;
    let mut res = 0;

    for &rotate in rotations.iter() {
        curr = curr + rotate;

        curr = modulo(curr, 100);

        if curr == 0 {
            res += 1;
        }
    }

    return res;
}