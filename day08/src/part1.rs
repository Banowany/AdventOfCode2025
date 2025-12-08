use std::collections::HashMap;
use crate::Point;

struct UnionFind {
    parent: HashMap<Point, Point>
}

impl UnionFind {
    fn new(points: &Vec<Point>) -> UnionFind {
        let mut union_find = UnionFind {
            parent: HashMap::new()
        };
        for &x in points {
            union_find.parent.insert(x, x);
        }

        union_find
    }

    fn find(&self, point: Point) -> Point {
        let curr_parent = *self.parent.get(&point).unwrap();
        if curr_parent != point {
            return self.find(curr_parent);
        }
        curr_parent
    }

    fn union(&mut self, u: Point, v: Point) {
        let u_parent = self.find(u);
        let v_parent = self.find(v);
        self.parent.entry(u_parent).and_modify(|x| {*x = v_parent});
    }
}

pub fn calc(points: &Vec<Point>, how_many_pairs: usize) -> i128 {
    let mut res = 0;
    let mut pairs: Vec<(Point, Point, i128)> = Vec::new();

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let first = points[i];
            let second = points[j];
            let dis = first.count_distance(&second);
            pairs.push((first, second, dis));
        }
    }

    pairs.sort_by_key(|&(_, _, z)| {z});

    let mut union_find = UnionFind::new(points);

    for &(first, second, dis) in pairs.iter().take(how_many_pairs) {
        union_find.union(first, second);
    }

    let mut size_of_groups: HashMap<Point, usize> = HashMap::new();
    for &point in points {
        let parent = union_find.find(point);
        size_of_groups.entry(parent).and_modify(|x| {*x += 1}).or_insert(1);
    }

    let mut vals: Vec<i128> = size_of_groups.values().map(|&x1| {x1 as i128}).collect();
    vals.sort();
    vals.reverse();
    res = vals[0] * vals[1] * vals[2];
    res
}