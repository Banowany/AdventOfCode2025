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

    fn is_all_in_one_circuit(&self) -> bool {
        let first = *self.parent.keys().nth(0).unwrap();
        let first_parent = self.find(first);
        for &other in self.parent.keys() {
            let parent = self.find(other);
            if first_parent != parent {
                return false;
            }
        }
        true
    }
}

pub fn calc(points: &Vec<Point>) -> i128 {
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

    for &(first, second, dis) in pairs.iter() {
        union_find.union(first, second);
        if union_find.is_all_in_one_circuit() {
            res = first.x * second.x;
            break;
        }
    }

    res
}