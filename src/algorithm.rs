//necessary inport statements 
use std::collections::{HashMap, HashSet, BinaryHeap, VecDeque};
use std::cmp::Ordering;
use crate::graph::Graph;

//create the Node Struct for building the graph 
#[derive(Eq, PartialEq)]
struct Node {
    id: usize,
    distance: usize,
}

//Define both methods for node for graph and Dijkstra's algorithm implementation 
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

//dijkstras_algorithm Function - Take in graph, start, end and return shortest distance between nodes 
pub fn dijkstras_algorithm(graph: &HashMap<usize, Vec<(usize, usize)>>, start: usize, end: usize) -> Option<usize> {
    //declare two empty data structures 
    let mut distances: HashMap<usize, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    //push initial positions to hashmap and binary heap
    distances.insert(start, 0);
    heap.push(Node { id: start, distance: 0 });

    //loop over all paths until find the end node 
    while let Some(Node { id, distance }) = heap.pop() {
        if id == end {
            return Some(distance); //id is the end node return the distance 
        }
        //Here I assumed that the weight is always 1 - weights not given in the dataset 
        if let Some(neighbors) = graph.get(&id) {
            for &(neighbor, weight) in neighbors {
                let total_distance = distance + weight;
                let entry = distances.entry(neighbor).or_insert(usize::MAX);
                if total_distance < *entry {
                    *entry = total_distance;
                    heap.push(Node { id: neighbor, distance: total_distance });
                }
            }
        }
    }

    None
}

//Compute the average distance from a source node to all other nodes in the graph - return a float 
pub fn average_distance_from_source(graph: &Graph, source_node: usize) -> f64 {
    let mut total_distance = 0;
    let mut total_nodes = 0;

    //Iterate dijkstra's algorithm with start node as the source then find the distance to ALL other nodes 
    for &destination_node in graph.adjacency_list.keys() {
        if let Some(dist) = dijkstras_algorithm(&graph.adjacency_list, source_node, destination_node) {
            total_distance += dist;
            total_nodes += 1;
        }
    }

    if total_nodes == 0 {
        return 0.0; // Avoid division by zero
    }

    total_distance as f64 / total_nodes as f64 //Finally return average distance 
}

pub fn average_distance_across_graph(graph: &Graph) -> f64 {
    let mut total_distance = 0;
    let mut total_nodes = 0;

    //Use Breadth First Search to find the distance from all nodes to all other nodes in the subgraph 
    for &start_node in graph.adjacency_list.keys() {
        let distances = bfs_distances(graph, start_node);
        total_distance += distances.iter().fold(0, |acc, (_, dist)| acc + dist);
        total_nodes += distances.len();
    }

    if total_nodes == 0 {
        return 0.0; // Avoid division by zero
    }

    total_distance as f64 / total_nodes as f64 //return average distance between nodes 
}

//Implement Breadth First Search Algorithm (code slightly modified from Lecture Notes)
pub fn bfs_distances(graph: &Graph, start_node: usize) -> HashMap<usize, usize> {
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_node, 0));
    visited.insert(start_node);

    while let Some((node, dist)) = queue.pop_front() {
        distances.insert(node, dist);

        if let Some(neighbors) = graph.adjacency_list.get(&node) {
            for &(neighbor, _) in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }
    }

    distances
}

//Clustering coefficient algorithm 
pub fn clustering_coefficient(graph: &Graph) -> f64 {
    let mut total_coefficient = 0.0;
    let mut total_nodes = 0;

    //loop through all nodes and neighbors in the graph 
    for (&node, neighbors) in &graph.adjacency_list {
        let num_neighbors = neighbors.len();
        if num_neighbors >= 2 {
            let mut num_triangles = 0;
            let mut num_possible_triangles = 0;

            // Check for triangles among the neighbors
            for i in 0..num_neighbors {
                let neighbor_i = neighbors[i].0;
                for j in (i + 1)..num_neighbors {
                    let neighbor_j = neighbors[j].0;
                    if graph.adjacency_list.contains_key(&neighbor_i) && graph.adjacency_list[&neighbor_i].iter().any(|&(k, _)| k == neighbor_j) {
                        num_triangles += 1;
                    }
                    num_possible_triangles += 1;
                }
            }

            // Calculate the clustering coefficient for the nodes
            if num_possible_triangles > 0 {
                let coefficient = num_triangles as f64 / num_possible_triangles as f64;
                total_coefficient += coefficient;
                total_nodes += 1;
            }
        }
    }

    // Avoid division by zero
    if total_nodes == 0 {
        return 0.0;
    }

    total_coefficient / total_nodes as f64 //return average clustering coefficient 
}

//graph density function -> returns a float of the density of the graph 
pub fn graph_density(graph: &Graph) -> f64 {
    let num_nodes = graph.adjacency_list.len();
    let num_edges = graph.adjacency_list.values().map(|edges| edges.len()).sum::<usize>();

    // Maximum possible number of edges in an undirected graph
    let max_edges = num_nodes * (num_nodes - 1) / 2;

    // Avoid division by zero
    if max_edges == 0 {
        return 0.0;
    }

    num_edges as f64 / max_edges as f64 //return average density 
}
