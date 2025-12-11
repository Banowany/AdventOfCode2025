use std::cmp::max;
use crate::Point;

pub fn calc(points: &Vec<Point>) -> i128 {
    let mut res = 0;

    for i in 0..points.len() {
        for j in (i+1)..points.len() {
            res = max(res, points.get(i).unwrap().filed_with(points.get(j).unwrap()))
        }
    }

    res
}