fn get_neighbours(paper_map: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<char> {
    let n = paper_map.len();
    let mut neighbours_indexes: Vec<(usize, usize)> = Vec::new();

    //left up
    if i >= 1 && j >= 1 {
        neighbours_indexes.push((i - 1, j - 1));
    }
    //left
    if j >= 1 {
        neighbours_indexes.push((i, j - 1));
    }
    //left down
    if i <= n - 2 && j >= 1 {
        neighbours_indexes.push((i + 1, j - 1));
    }
    //down
    if i <= n - 2 {
        neighbours_indexes.push((i + 1, j));
    }
    //right down
    if i <= n - 2 && j <= n - 2 {
        neighbours_indexes.push((i + 1, j + 1));
    }
    //right
    if j <= n - 2 {
        neighbours_indexes.push((i, j + 1));
    }
    //right up
    if i >= 1 && j <= n - 2 {
        neighbours_indexes.push((i - 1, j + 1));
    }
    //up
    if i >= 1 {
        neighbours_indexes.push((i - 1, j));
    }

    return neighbours_indexes
        .iter()
        .map(|&(i1, j1)| paper_map[i1][j1])
        .collect();
}

pub fn calc(paper_map: &Vec<Vec<char>>) -> i32 {
    let n = paper_map.len();
    let mut res = 0;

    for i in 0..n {
        for j in 0..n {
            let how_many_neighbours: usize = get_neighbours(paper_map, i, j)
                .iter()
                .map(|&x| if x == '@' { 1 } else { 0 })
                .sum();
            res += if how_many_neighbours < 4 && paper_map[i][j] == '@' { 1 } else { 0 };
        }
    }

    return res;
}
