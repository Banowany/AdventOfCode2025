use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use crate::Point;

// struct MathVec {
//     diff_col: i128,
//     diff_row: i128
// }
//
// impl MathVec {
//     fn new(a: &Point, b: &Point) -> MathVec {
//         MathVec {
//             diff_col: b.col - a.col,
//             diff_row: b.row - a.row
//         }
//     }
//
//     fn product(&self, math_vec: &MathVec) -> i128 {
//         self.diff_col*math_vec.diff_row - self.diff_row*math_vec.diff_col
//     }
// }

// struct Segment {
//     col: i128,
//     row: i128,
//     diff_col: i128,
//     diff_row: i128
// }
//
// impl Segment {
//     fn new(a: &Point, b: &Point) -> Segment {
//         Segment {
//             col: a.col,
//             row: a.row,
//             diff_col: b.col -
//         }
//     }
// }

fn is_inside(points: &Vec<Point>, point: &Point) -> bool {
    let mut inside = false;

    let n = points.len();
    for i in 0..n {
        let a = points.get(i).unwrap();
        let b = points.get((i+1)%n).unwrap();
        let c = points.get((i+2)%n).unwrap();

        if a.col <= point.col && b.col <= point.col  {
            continue;
        }

        if a.row == b.row && b.row == point.row && c.row > point.row {
            inside = !inside;
        } else if a.row > b.row && b.row == c.row && c.row == point.row {
            inside = !inside;
        } else if min(a.row, b.row) < point.row && point.row < max(a.row, b.row) {
            inside = !inside;
        }
    }

    return inside;
}

fn create_grid(points: &Vec<Point>) -> Vec<Vec<bool>> {
    let mut grid: Vec<Vec<bool>> = Vec::new();

    let max_col = points.iter().map(|x| {x.col}).max().unwrap();
    let max_row = points.iter().map(|x| {x.row}).max().unwrap();

    for col in 0..max_col+1 {
        let mut full_col: Vec<bool> = Vec::new();
        for row in 0..max_row + 1 {
            full_col.push(false);
        }
        grid.push(full_col);
    }

    let n = points.len();
    for i in 0..n {

        let a = points.get(i).unwrap();
        let b = points.get((i+1)%n).unwrap();

        for col in min(a.col, b.col)..max(a.col, b.col)+1 {
            for row in min(a.row, b.row)..max(a.row, b.row)+1 {
                grid[col as usize][row as usize] = true;
            }
        }
    }

    for col in 0..max_col+1 {
        for row in 0..max_row+1 {
            if !grid[col as usize][row as usize] {
                let point = Point::new(col, row);
                grid[col as usize][row as usize] = is_inside(points, &point);
            }
        }
    }

    return grid;

    // for col in 0..(max_col+1) {
    //     let mut curr_full_col: Vec<bool> = Vec::new();
    //     for row in 0..(max_row+1) {
    //         let a = Point::new(col, row);
    //
    //         if is_inside(&points, &a) {
    //             curr_full_col.push(true)
    //         }
    //         else {
    //             curr_full_col.push(false)
    //         }
    //     }
    //     grid.push(curr_full_col);
    // }
}

fn compress(points: &Vec<Point>) -> Vec<Point> {
    let mut unique_col: Vec<i128> = points.iter().map(|x| {x.col}).collect::<HashSet<i128>>().iter().map(|x1| {*x1}).collect();
    unique_col.sort();
    let col_map: HashMap<i128, i128> = unique_col.iter().enumerate().map(|x3| {(*x3.1, x3.0 as i128)}).collect();
    let mut unique_row: Vec<i128> = points.iter().map(|x| {x.row}).collect::<HashSet<i128>>().iter().map(|x1| {*x1}).collect();
    unique_row.sort();
    let row_map: HashMap<i128, i128> = unique_row.iter().enumerate().map(|x3| {(*x3.1, x3.0 as i128)}).collect();

    let mut compressed_points: Vec<Point> = Vec::new();
    for point in points {
        compressed_points.push(
            Point::new(
                col_map[&point.col] as i128,
                row_map[&point.row] as i128
            )
        )
    }
    compressed_points
}

pub fn calc(points: &Vec<Point>) -> i128 {
    let mut res = 0;

    let compressed_points = compress(points);
    let grid = create_grid(&compressed_points);

    // for column in grid.iter() {
    //     println!("{:?}", column);
    // }

    for i in 0..compressed_points.len() {
        for j in (i+1)..compressed_points.len() {
            let a = compressed_points.get(i).unwrap();
            let b = compressed_points.get(j).unwrap();

            let mut some_point_out = false;
            let min_col = min(a.col, b.col);
            let max_col = max(a.col, b.col);
            let min_row = min(a.row, b.row);
            let max_row = max(a.row, b.row);

            for col in min_col..max_col+1 {
                for row in min_row..max_row+1 {
                    if !grid[col as usize][row as usize] {
                        some_point_out = true;
                        break;
                    }
                }
            }

            if !some_point_out {
                res = max(res, points.get(i).unwrap().filed_with(points.get(j).unwrap()))
            }
        }
    }

    res
}