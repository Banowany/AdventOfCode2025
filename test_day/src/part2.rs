use std::collections::HashMap;

pub fn calculate(vec1:Vec<i32>, vec2:Vec<i32>) -> i32 {
    let mut appearance:HashMap<i32, i32> = HashMap::new();

    for &x in vec2.iter() {
        appearance.entry(x).and_modify(|x1| {*x1 += 1}).or_insert(1);
    }

    let res:i32 = vec1.iter().map(|a| {*a * appearance.get(a).unwrap_or(&0)}).sum();

    return res;
}