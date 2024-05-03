use std::collections::{BinaryHeap, HashSet};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
//use petgraph::algo::dijkstra;

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    node: NodeIndex,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn dijkstra(graph: &DiGraph<usize, ()>, start: NodeIndex, goal: NodeIndex) -> Option<usize> {
    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(State { cost: 0, node: start });

    while let Some(State { cost, node }) = heap.pop() {
        if node == goal {
            return Some(cost);
        }

        if visited.contains(&node) {
            continue;
        }

        visited.insert(node);

        for edge in graph.edges(node) {
            let next_node = edge.target();
            if !visited.contains(&next_node) {
                heap.push(State { cost: cost + 1, node: next_node });
            }
        }
    }

    None
}