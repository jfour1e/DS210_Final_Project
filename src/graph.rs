//Necessary imports 
use std::collections::{HashMap};
use std::fs::File;
use std::fs;
use std::io::{BufRead, BufReader};

//Read txt file to convert the .txt file into an adjacency list graph 
pub fn read_txt(filename: &str) -> Result<Vec<(usize, usize)>, std::io::Error> {
    let contents = fs::read_to_string(filename)?;
    let mut edges = Vec::new();

    //iterate through all lines and add edges and nodes (skipping first 4 lines)
    for line in contents.lines().skip(4) {
        let mut split = line.split_whitespace();
        let from_node: usize = split.next().unwrap().parse().unwrap();
        let to_node: usize = split.next().unwrap().parse().unwrap();
        edges.push((from_node, to_node));
    }

    //Make sure that there are no errors in this function, return the Adjacency list 
    Ok(edges)
}

// Function to read the dataset and filter nodes and edges within the specified range 
//Necessary to lower computation time to derive more insights 
pub fn filter_graph(graph: &mut Graph, file_path: &str, start_node: usize, end_node: usize) {
    // Open the .txt file 
    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);

        // Iterate through each line of the dataset
        for line in reader.lines() {
            if let Ok(edge) = line {
                let parts: Vec<usize> = edge
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                // Extract source and destination node IDs
                if let Some((source, destinations)) = parts.split_first() {
                    let source_node = *source;
                    
                    // Check if source node is within the specified range
                    if source_node >= start_node && source_node <= end_node {
                        for &destination in destinations {
                            // Filter destination nodes that are within the specified range
                            if destination >= start_node && destination <= end_node {
                                // Add edges to the graph
                                graph.add_edge(source_node, destination, 1); // Assuming weight is 1 for simplicity (like in dijkstra's algorithm)
                            }
                        }
                    }
                }
            }
        }
    }
}

//Define struct for graph -> need for converting .txt file into adjacency list 
#[derive(Debug)]
pub struct Graph {
    pub adjacency_list: HashMap<usize, Vec<(usize, usize)>>,
}

//Graph method with built in functions to convert txt file into adjacency list 
impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from_node: usize, to_node: usize, weight: usize) {
        self.adjacency_list.entry(from_node).or_insert(Vec::new()).push((to_node, weight));
    }
}