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

//All test functions below 
#[cfg(test)]
mod tests {
    //necessary import statements 
    use super::algorithm::{dijkstras_algorithm, average_distance_from_source, average_distance_across_graph, bfs_distances, clustering_coefficient, graph_density}; 
    use super::graph::Graph;
    use super::*;
    use std::collections::{HashMap, HashSet, VecDeque};

    //Test Dijktra's Algorithm Function 
    #[test]
    fn test_dijkstra() {
        // Define small sample graph
        let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
        graph.insert(0, vec![(1, 5), (2, 3)]);
        graph.insert(1, vec![(3, 6)]);
        graph.insert(2, vec![(1, 2), (3, 7)]);
        graph.insert(3, vec![]); // End node

        // Test case 1: Start and end nodes are in the graph
        assert_eq!(dijkstras_algorithm(&graph, 0, 3), Some(10));

        // Test case 2: Start node is not in the graph
        assert_eq!(dijkstras_algorithm(&graph, 4, 3), None);

        // Test case 3: End node is not reachable from the start node
        assert_eq!(dijkstras_algorithm(&graph, 0, 4), None);
    }

    //Test for average_distance_from_source function 
    #[test]
    fn test_average_distance_from_source() {
        // Create sample graph
        let mut graph = Graph::new();
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 2);
        graph.add_edge(2, 3, 1);

        // Calculate the average distance from node 1
        let avg_distance = average_distance_from_source(&graph, 1);

        // Make sure function outputs the expected result 
        assert_eq!(avg_distance, 0.5);
    }

    //Test function for average_distance_across_graph function 
    #[test]
    fn test_average_distance_across_graph() {
        // Create sample graph
        let mut graph = Graph::new();
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 2);
        graph.add_edge(2, 3, 1);

        // Calculate average distance across the graph
        let avg_distance = average_distance_across_graph(&graph);

        // Make sure function outputs the expected result
        assert_eq!(avg_distance, 0.6);
    }

    //Test for bfs_distances function 
    #[test]
    fn test_bfs_distances() {
        // Create sample graph 
        let mut graph = Graph::new();
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 2);
        graph.add_edge(2, 3, 1);

        // Calculate distances using BFS from node 1 to others 
        let distances = bfs_distances(&graph, 1);

        // Expected distances
        let mut expected_distances = HashMap::new();
        expected_distances.insert(1, 0);
        expected_distances.insert(2, 1);
        expected_distances.insert(3, 1);

        // make sure output is what is expected 
        assert_eq!(distances, expected_distances);
    }

    //Clustering coefficient function test 
    #[test]
    fn test_clustering_coefficient() {
        // Create sample graph 
        let mut graph = Graph::new();
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 1);
        graph.add_edge(2, 3, 1);

        // Calculate the clustering coefficient
        let coefficient = clustering_coefficient(&graph);

        // Make sure output is expected 
        assert_eq!(coefficient, 1.0);
    }

    //Test for graph density function 
    #[test]
    fn test_graph_density() {
        // Create sample graph 
        let mut graph = Graph::new();
        graph.add_edge(1, 2, 1);
        graph.add_edge(1, 3, 1);
        graph.add_edge(2, 3, 1);

        // Calculate the graph density
        let density = graph_density(&graph);

        // Is the output expected? 
        assert_eq!(density, 3.0);
    }

}