use std::collections::HashMap;
use crate::{Graph, Node};

fn dfs_to_sort(curr_id: String, visited: &mut HashMap<String, bool>, stack: &mut Vec<String>, graph: &Graph) {
    visited.entry(curr_id.clone()).and_modify(|x| {*x = true});
    for next in graph.nodes.get(&curr_id).unwrap().childs.iter() {
        if !visited.get(next).unwrap() {
            dfs_to_sort(next.clone(), visited, stack, graph);
        }
    }
    stack.push(curr_id);
}

pub fn calc(graph: &Graph, start: &String, stop: &String) -> usize {
    let mut res = 0;

    let mut sorted_nodes: Vec<String> = Vec::new();
    let mut visited: HashMap<String, bool> = graph.nodes.keys().map(|x| {(x.clone(), false)}).collect();

    dfs_to_sort(start.clone(), &mut visited, &mut sorted_nodes, graph);
    sorted_nodes.reverse();

    let mut counter_of_ways: HashMap<String, usize> = graph.nodes.keys().map(|x| {(x.clone(), 0usize)}).collect();
    counter_of_ways.entry(start.clone()).and_modify(|x1| {*x1 = 1});

    for u in sorted_nodes.iter() {
        let val_u = *counter_of_ways.get(u).unwrap();
        for v in graph.nodes.get(u).unwrap().childs.iter() {
            counter_of_ways.entry(v.clone()).and_modify(|x2| {*x2 += val_u});
        }
    }

    res = *counter_of_ways.get(stop).unwrap();

    res
}