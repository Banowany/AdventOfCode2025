use std::collections::HashSet;

fn get_neighbours(paper_map: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<(usize, usize)> {
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

    return neighbours_indexes;
}

fn is_available(paper_map: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let neighbours_paper: Vec<(usize, usize)> = get_neighbours(paper_map, i, j)
        .iter()
        .filter(|&&(x, y)| paper_map[x][y] == '@')
        .copied()
        .collect();

    let how_many_paper_around = neighbours_paper.iter().count();
    return paper_map[i][j] == '@' && how_many_paper_around < 4;
}

pub fn calc(paper_map: &Vec<Vec<char>>) -> usize {
    let mut paper_map_clone = paper_map.clone();
    let n = paper_map.len();
    let mut res = 0;
    let mut vertices: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            vertices.insert((i, j));
        }
    }

    vertices = vertices
        .iter()
        .filter(|&&(i, j)| is_available(&paper_map_clone, i, j))
        .copied()
        .collect();

    while !vertices.is_empty() {
        res += vertices.len();

        for &(i, j) in vertices.iter() {
            paper_map_clone[i][j] = '.';
        }

        vertices = vertices
            .iter()
            .flat_map(|&(i, j)| get_neighbours(&paper_map_clone, i, j))
            .filter(|&(i, j)| is_available(&paper_map_clone, i, j))
            .collect();
    }

    return res;
}
