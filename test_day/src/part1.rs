pub fn calculate(mut vec1:Vec<i32>, mut vec2:Vec<i32>) -> i32 {
    vec1.sort();
    vec2.sort();

    let res:i32 = vec1.iter().zip(vec2.iter()).map(|(a,b)| (a-b).abs()).sum();

    return res;
}