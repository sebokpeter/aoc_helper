use std::{collections::HashMap, hash::Hash};

use priority_queue::DoublePriorityQueue;

// Represents a set of nodes connected by edges
pub trait Graph {
    type DataType; // Type of the data contained in each node
    type NodeReference: Hash + Eq + Clone + Copy;
    type EdgeReference;

    /// Create a new Graph.
    fn new() -> Self
    where
        Self: Sized;

    /// Add a new node to the graph.
    ///
    /// # Arguments
    ///
    /// * `data` - The data that the node will contain.
    fn add_node(&mut self, data: Self::DataType) -> Self::NodeReference;

    /// Add an edge between two nodes, 'source', and 'target'.
    ///
    /// # Arguments
    ///
    /// * `source` - The source node.
    /// * `target` - The target node.
    fn add_edge(&mut self, source: Self::NodeReference, target: Self::NodeReference);

    /// Retrieve immutable reference to the data stored in the node specified by `node`.
    ///
    /// # Arguments
    ///
    /// * `node` - Index of reference of the node which contains the data.
    fn get_data(&self, node: &Self::NodeReference) -> Option<&Self::DataType>;

    /// Retrieve mutable reference to the data stored in the node specified by `node`.
    ///
    /// # Arguments
    ///
    /// * `node` - Index of reference of the node which contains the data.
    fn get_data_mut(&mut self, node: &Self::NodeReference) -> Option<&mut Self::DataType>;

    /// Get the [NodeReferences] of all neighbors of [node].
    fn get_neighbors(&self, node: &Self::NodeReference) -> Vec<Self::NodeReference>;

    /// Search the graph for the shortest route between `start` and `target`, using Dijkstra’s Algorithm.
    /// Each node in the graph must have a cost associated with it.
    ///
    /// # Arguments
    ///
    /// * `start`       - The node where the search starts.
    /// * `target`      - The target node, where the search will terminate.
    /// * `cost_fn`     - A function that calculates the cost of traversing given the data stored in a node.
    fn dijkstra<F>(
        &self,
        start: Self::NodeReference,
        target: Self::NodeReference,
        cost_fn: F,
    ) -> Vec<Self::NodeReference>
    where
        F: Fn(&Self::DataType) -> usize,
        Self: Sized
    {
        let mut frontier = DoublePriorityQueue::new();
        frontier.push(start, 0);

        let mut came_from = HashMap::new();
        came_from.insert(start, start);

        let mut cost_so_far = HashMap::new();
        cost_so_far.insert(start, 0);

        while !frontier.is_empty() {
            let (current, _) = frontier.pop_min().unwrap();

            if current == target {
                break;
            }

            for next in self.get_neighbors(&current) {
                let data = self.get_data(&next).unwrap();
                let new_cost = cost_fn(data) + cost_so_far[&current];

                if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                    cost_so_far.insert(next, new_cost);
                    came_from.insert(next, current);
                    frontier.push(next, new_cost);
                }
            }
        }

        reconstruct_path::<Self>(came_from, start, target)
    }

    /// Search the graph for the shortest route between two nodes, using Dijkstra’s Algorithm.
    /// Instead of specifying the start and target nodes, this function takes two [`Fn`]s.
    /// The first, 'frontier_fn' specifies if a node should be part of the initial frontier.
    /// The second, 'target_fn' checks if a given node is a target node.
    ///
    /// # Arguments
    ///
    /// * `start_fn`: An [`Fn`] that checks if a given node should be included in the initial frontier.
    /// * `target_fn`: An [`Fn`] that checks if a given node is a target node.
    /// * `cost_fn`: A [`Fn`] that calculates the cost of traversing given the data stored in a node.
    fn dijkstra_search_with_closure<S, D, C>(
        &self,
        frontier_fn: S,
        target_fn: D,
        cost_fn: C,
    ) -> Vec<Self::NodeReference>
    where
        S: Fn(&Self::DataType) -> bool,
        D: Fn(&Self::DataType) -> bool,
        C: Fn(&Self::DataType) -> usize;

    /// Searches for a node that satisfies [predicate].
    ///
    /// # Arguments
    ///
    /// * `predicate` - A closure that is applied to each node in the graph. If it returns [true], then [find()] returns [Some(current_node_index)]. If no nodes match, [find()] will return [None].
    fn find<F>(&self, predicate: F) -> Option<Self::NodeReference>
    where
        F: Fn(&Self::DataType) -> bool;
}

fn reconstruct_path<G>(came_from: HashMap<G::NodeReference, G::NodeReference>, start: G::NodeReference, target: G::NodeReference) -> Vec<G::NodeReference>
where
    G: Graph + Sized,
{
    let mut path = Vec::new();

    if !came_from.contains_key(&target) {
        return path;
    }

    let mut current = target;

    while current != start {
        path.push(current);
        current = came_from[&current]
    }

    path.push(start);
    path.reverse();

    path
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeIndex(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeIndex(pub usize);

pub mod grid;
pub mod rc_graph;
pub mod vec_graph;
