//import necessary functions/ methods from algorithm and graph rust files 
mod graph;
mod algorithm;
use graph::{Graph, filter_graph, read_txt};
use algorithm::{dijkstras_algorithm, average_distance_from_source, average_distance_across_graph, graph_density, clustering_coefficient}; 
use std::io;

fn main() {

    //read the CA road .txt file using the read_txt file 
    let edges = read_txt("roadNet-CA.txt").unwrap();

    //create a graph in the list of edges format 
    let mut graph = Graph::new();
    for (from, to) in edges {
        graph.add_edge(from, to, 1); // Assume all edges have weight 1 (important for dijkstra's algorithm)
    }

    //Dijkstra's algorithm code: 

    //declare start and end Node for dijkstra's algorithm based on user input 
    let start_node: i32 = loop {
        println!("Enter the start Node (Range 1 -- 1,965,206):"); //Starting prompt 
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line"); 
        // Parse input into an integer
        match input.trim().parse() {
            Ok(node) => {
                if node < 0 {
                    println!("Node cannot be negative -> Please enter a non-negative integer."); //Catch negative number error here 
                } else {
                    break node;
                }
            }
            Err(_) => { 
                println!("Invalid input -> Please enter an integer"); //catch integer error here 
                continue;
            }
        }
    };

    let end_node: i32 = loop {
        println!("Enter the end node (Range 1 -- 1,965,206):"); //starting prompt
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line"); 
        // Parse input into an integer
        match input.trim().parse() {
            Ok(node) => {
                if node < 0 {
                    println!("Node cannot be negative. Please enter a non-negative integer."); //catch negative number error 
                } else {
                    break node;
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter an integer."); //catch integer error here 
                continue;
            }
        }
    };

    // Check if start_node and end_node are within range and throw error if not in range 
    let node_range = 0..1965206; 
    if !node_range.contains(&start_node) || !node_range.contains(&end_node) {
        println!("Error: Node must be in the range 0 to 50000.");
        return;
    }

    //Call Dijkstra's algorithm to find the shortest path between the start and end nodes passed in by the user. 
    if let Some(distance) = dijkstras_algorithm(&graph.adjacency_list, start_node.try_into().unwrap(), end_node.try_into().unwrap()) {
        println!("Shortest distance from node {} to node {} in the large graph is {}", start_node, end_node, distance);

    } else {
        println!("No path found between node {} and node {}", start_node, end_node);
    }

    //Calculate Graph density and clustering coefficients of the Graph 
    let density = graph_density(&graph);
    println!("Graph Density: {:.7}", density);

    let clustering_coeff = clustering_coefficient(&graph);
    println!("Clustering Coefficient: {:.4}", clustering_coeff);

    //Take a subset of the larger graph from roadNET-CA.txt 
    //In order to shorten computation time and derive more insights a subset must be taken 

    // // Create a new graph instance for the smaller graph and declare new graph size (in nodes)
    let mut filtered_graph = Graph::new();
    let graph_size = 20000; 

    // Filter the graph and update the filtered graph
    filter_graph(&mut filtered_graph, "roadNet-CA.txt", 1, graph_size);
    
    //Use Dijktra's algorithm to find the average distance from a specific node to all other nodes in the graph 

    //define any start in range of nodes in graph 
    let start = 50; 

    //Compute average distance from node to all other nodes and print the result 
    let distance = average_distance_from_source(&filtered_graph, start);
    println!("Average distance from start node { } to all other nodes in the subgraph: {:.2}", start, distance);

    //Finally using Breadth first search, find the average distance between all nodes in the graph 

    //Compute average distance and print the result 
    let distance = average_distance_across_graph(&filtered_graph);
    println!("Average distance between nodes in the subgraph: {:.2}. (**Note for Grader this will take about 30 seconds**).", distance);
}   