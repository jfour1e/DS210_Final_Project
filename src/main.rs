use std::collections::{HashMap,BinaryHeap};
use std::cmp::Ordering;
use std::fs;
use plotters::prelude::*;

fn read_txt(filename: &str) -> Result<Vec<(usize, usize)>, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let mut edges = Vec::new();

    for line in contents.lines().skip(4) {
        let mut split = line.split_whitespace();
        let from_node: usize = split.next().unwrap().parse().unwrap();
        let to_node: usize = split.next().unwrap().parse().unwrap();
        edges.push((from_node, to_node));
    }

    Ok(edges)
}

#[derive(Debug)]
struct Graph {
    adjacency_list: HashMap<usize, Vec<(usize, usize)>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from_node: usize, to_node: usize, weight: usize) {
        self.adjacency_list.entry(from_node).or_insert(Vec::new()).push((to_node, weight));
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    id: usize,
    distance: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(graph: &HashMap<usize, Vec<(usize, usize)>>, start: usize, end: usize) -> Option<usize> {
    let mut distances: HashMap<usize, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    distances.insert(start, 0);
    heap.push(Node { id: start, distance: 0 });

    while let Some(Node { id, distance }) = heap.pop() {
        if id == end {
            return Some(distance);
        }

        if let Some(neighbors) = graph.get(&id) {
            for &(neighbor, weight) in neighbors {
                let total_distance = distance + weight;
                if !distances.contains_key(&neighbor) || total_distance < *distances.get(&neighbor).unwrap_or(&usize::MAX) {
                    distances.insert(neighbor, total_distance);
                    heap.push(Node { id: neighbor, distance: total_distance });
                }
            }
        }
    }

    None
}


fn main() {
    let edges = read_txt("roadNet-CA.txt").unwrap();

    let mut graph = Graph::new();
    for (from, to) in edges {
        graph.add_edge(from, to, 1); // Assuming all edges have weight 1
    }

    let start_node = 60;
    let end_node = 500;
    if let Some(distance) = dijkstra(&graph.adjacency_list, start_node, end_node) {
        println!("Shortest distance from node {} to node {} is {}", start_node, end_node, distance);

        let shortest_path = find_shortest_path(&graph, start_node, end_node);

        if let Err(err) = plot_shortest_path(&shortest_path) {
            eprintln!("Error plotting shortest path: {}", err);

    } else {
        println!("No path found between node {} and node {}", start_node, end_node);
    }
}
}