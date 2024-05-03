#[cfg(test)]

mod tests {
    use super::*;
    use super::super::graph::Graph; 
    use std::collections::HashMap;

    #[test]
    fn test_dijkstra() {
        // Create a test graph
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(1, 3);

        // Call Dijkstra's algorithm on the graph
        let start_node = 1;
        let shortest_distances = Graph::dijkstra(&graph, start_node);

        // Verify the expected shortest distances
        let expected_distances: HashMap<usize, usize> = [(1, 0), (2, 1), (3, 1)].iter().cloned().collect();
        assert_eq!(shortest_distances, expected_distances);
    }
}