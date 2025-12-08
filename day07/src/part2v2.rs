pub fn calc(p0: &Vec<&str>, p1: (usize, usize)) -> usize {
    let mut res: usize = 0;

    let mut array2d: Vec<Vec<usize>> = Vec::new();

    for &line in p0.iter() {
        array2d.push(line.chars().map(|_| {0}).collect());
    }

    *array2d.get_mut(p1.0).unwrap().get_mut(p1.1).unwrap() = 1;

    for (row, &line) in p0.iter().enumerate().skip(1) {
        for (col, sign) in line.chars().enumerate() {
            *array2d.get_mut(row).unwrap().get_mut(col).unwrap() += *array2d.get(row-1).unwrap().get(col).unwrap();

            if sign == '^' {
                *array2d.get_mut(row).unwrap().get_mut(col-1).unwrap() += *array2d.get(row).unwrap().get(col).unwrap();
                *array2d.get_mut(row).unwrap().get_mut(col+1).unwrap() += *array2d.get(row).unwrap().get(col).unwrap();
                *array2d.get_mut(row).unwrap().get_mut(col).unwrap() = 0;
            }
        }
    }

    res += array2d.iter().last().unwrap().iter().map(|&x| {x}).sum::<usize>();

    res
}