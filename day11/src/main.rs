mod part1;
mod part2;

use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Node {
    id: String,
    childs: Vec<String>
}

struct Graph {
    nodes: HashMap<String, Node>
}

impl Node {
    fn new(id: String) -> Node {
        Node {
            id,
            childs: Vec::new()
        }
    }
}

impl Graph {
    fn new() -> Graph {
        Graph {
            nodes: HashMap::new()
        }
    }

    fn add_device(&mut self, device_id: String, next_devices: Vec<String>) {
        if !self.nodes.contains_key(&device_id) {
            self.nodes.insert(device_id.clone(), Node::new(device_id.clone()));
        }
        for next_device in next_devices.iter() {
            if !self.nodes.contains_key(next_device) {
                self.nodes.insert(next_device.clone(), Node::new(next_device.clone()));
            }
        }

        self.nodes.entry(device_id).and_modify(|x| {x.childs = next_devices});
    }
}

fn convert_line(line: &str) -> (String, Vec<String>) {
    let parts: Vec<&str> = line.split(": ").collect();
    let device_id: String = String::from(parts[0]);
    let mut next_devices: Vec<String> = Vec::new();

    for next_device in parts[1].split_whitespace() {
        next_devices.push(String::from(next_device));
    }

    return (device_id, next_devices);
}

fn main() {
    let input: String = fs::read_to_string("input").expect("Error during reading the file");
    let converted_input: Vec<(String, Vec<String>)> = input.lines().map(|x| {convert_line(x)}).collect();
    let mut graph = Graph::new();
    for device in converted_input {
        graph.add_device(device.0, device.1);
    }
    println!("{}", part1::calc(&graph, &"you".to_string(), &"out".to_string()));
    println!("{}", part2::calc(&graph));
}
