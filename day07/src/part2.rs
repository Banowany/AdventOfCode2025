use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Node {
    id: (usize, usize),
    childs: Vec<(usize, usize)>,
    parents: Vec<(usize, usize)>,
}

struct Graph {
    nodes: HashMap<(usize, usize), Node>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
        }
    }

    fn create_vertices(&mut self, splitters: &HashMap<usize, Vec<usize>>, start: (usize, usize)) {
        for (&col, rows) in splitters {
            for &row in rows {
                let key = (row, col);
                let val = Node {
                    id: key,
                    childs: Vec::new(),
                    parents: Vec::new(),
                };
                self.nodes.insert(key, val);
            }
        }

        let key = start;
        let val = Node {
            id: key,
            childs: Vec::new(),
            parents: Vec::new(),
        };

        self.nodes.insert(key, val);
    }

    fn add_edge(&mut self, begin: (usize, usize), end: (usize, usize)) {
        self.nodes.get_mut(&begin).unwrap().childs.push(end);
        self.nodes.get_mut(&end).unwrap().parents.push(begin);
    }

    fn create_edges(&mut self, splitters: &HashMap<usize, Vec<usize>>, start: (usize, usize)) {
        let first_splitter: (usize, usize) = (
            *splitters.get(&start.1).unwrap().iter().min().unwrap(),
            start.1,
        );
        self.add_edge(start, first_splitter);

        let mut curr: HashSet<(usize, usize)> = [start].iter().cloned().collect();
        let mut next: HashSet<(usize, usize)> = [first_splitter].iter().cloned().collect();

        while !next.is_empty() {
            curr = next;
            next = HashSet::new();

            for &(row, col) in curr.iter() {
                let maybe_next_splitter_row = splitters
                    .get(&(col.checked_sub(1).unwrap_or(0usize)))
                    .unwrap_or(&Vec::new())
                    .iter()
                    .filter(|&&x| x > row)
                    .min()
                    .cloned();

                match maybe_next_splitter_row {
                    None => {}
                    Some(next_splitter_row) => {
                        next.insert((next_splitter_row, col - 1));
                        self.add_edge((row, col), (next_splitter_row, col - 1));
                    }
                }

                let maybe_next_splitter_row = splitters
                    .get(&(col + 1))
                    .unwrap_or(&Vec::new())
                    .iter()
                    .filter(|&&x| x > row)
                    .min()
                    .cloned();

                match maybe_next_splitter_row {
                    None => {}
                    Some(next_splitter_row) => {
                        next.insert((next_splitter_row, col + 1));
                        self.add_edge((row, col), (next_splitter_row, col + 1));
                    }
                }
            }
        }
    }

    fn create_graph_from_data(
        splitters: &HashMap<usize, Vec<usize>>,
        start: (usize, usize),
    ) -> Graph {
        let mut graph = Graph::new();

        graph.create_vertices(splitters, start);
        graph.create_edges(splitters, start);

        graph
    }

    fn get(&self, i: (usize, usize)) -> Option<&Node> {
        self.nodes.get(&i)
    }
}

pub fn calc(splitters: &HashMap<usize, Vec<usize>>, start: (usize, usize)) -> usize {
    let mut res: usize = 0;

    let graph = Graph::create_graph_from_data(splitters, start);

    let mut counter_of_ways: HashMap<(usize, usize), usize> = HashMap::new();

    for &(i, j) in graph.nodes.keys() {
        counter_of_ways.insert((i, j), 0);
    }
    counter_of_ways.entry(start).and_modify(|x| *x = 1);

    let mut curr: HashSet<(usize, usize)> = vec![start].iter().cloned().collect();

    while !curr.is_empty() {
        let next: HashSet<(usize, usize)> = curr
            .iter()
            .flat_map(|&x1| graph.get(x1).unwrap().childs.iter().cloned())
            .collect();

        for &x in next.iter() {
            for &y in graph.get(x).unwrap().parents.iter() {
                let parents_ways = *counter_of_ways.get(&y).unwrap();
                counter_of_ways.entry(x).and_modify(|x| *x += parents_ways);
            }
        }

        curr = next;
    }

    // let nodes_with_only_one_child: Vec<(usize, usize)> = graph
    //     .nodes
    //     .iter()
    //     .filter(|&(_, node)| node.childs.iter().len() == 1)
    //     .map(|(&k, _)| k)
    //     .collect();
    //
    // let nodes_with_no_child: Vec<(usize, usize)> = graph
    //     .nodes
    //     .iter()
    //     .filter(|&(_, node)| node.childs.iter().len() == 0)
    //     .map(|(&k, _)| k)
    //     .collect();

    println!("{:?}", graph.nodes);
    // println!("{:?}", counter_of_ways);
    // println!("{:?}", nodes_with_only_one_child);
    // println!("{:?}", nodes_with_no_child);

    // for x in nodes_with_only_one_child.iter() {
    //     res += counter_of_ways.get(x).unwrap_or(&(0usize));
    // }
    //
    // for x in nodes_with_no_child.iter() {
    //     res += 2 * counter_of_ways.get(x).unwrap_or(&(0usize));
    // }

    for &col in splitters.keys() {
        let row: usize = *splitters.get(&col).unwrap().last().unwrap();
        let val = *counter_of_ways.get(&(row, col)).unwrap_or(&0usize);
        res += val;
        println!("{:?} {}", (row, col), val);

    }

    return res;
}
