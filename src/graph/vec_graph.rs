use std::collections::HashMap;
use priority_queue::DoublePriorityQueue;

use super::Graph;

// An implementation of a graph datastructure, using vectors to store nodes and edges.
// Based on: https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
pub struct VecGraph<T> {
    nodes: Vec<NodeData<T>>,
    edges: Vec<EdgeData>,
}

impl<T> Graph for VecGraph<T> {
    type DataType = T;
    type NodeReference = NodeIndex;
    type EdgeReference = EdgeIndex;

    fn new() -> Self
    where
        Self: Sized,
    {
        VecGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, data: Self::DataType) -> Self::NodeReference {
        let index = self.nodes.len();
        self.nodes.push(NodeData {
            data,
            first_outgoing_edge: None,
        });
        NodeIndex(index)
    }

    fn add_edge(&mut self, source: Self::NodeReference, target: Self::NodeReference) {
        let edge_index = self.edges.len();

        // TODO: should we return something (E.g. Result) instead of panicking?
        if self.nodes.len() < target.0 {
            panic!("Target node not found!");
        }

        let Some(source_node) = self.nodes.get_mut(source.0) else {
            panic!("Source node not found.");
        };

        self.edges.push(EdgeData {
            target,
            next_outgoing_edge: source_node.first_outgoing_edge,
        });

        source_node.first_outgoing_edge = Some(EdgeIndex(edge_index));
    }

    fn get_data(&self, node: &Self::NodeReference) -> &Self::DataType {
        if let Some(node_data) = self.nodes.get(node.0) {
            &node_data.data
        } else {
            panic!("Node not found!");
        }
    }

    fn dijkstra<F>(&self, start: Self::NodeReference, target: Self::NodeReference, cost_fn: F) -> Vec<Self::NodeReference>
        where F: Fn(&Self::DataType) -> usize
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

            //for next in self.successors(current) {
            for next in self.successors(current) {
                let data = self.get_data(&next);
                let new_cost = cost_fn(data) + cost_so_far[&current];

                if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                    cost_so_far.insert(next, new_cost);
                    came_from.insert(next, current);
                    frontier.push(next, new_cost);
                }
            }
        }

        reconstruct_path(came_from, start, target)
    }
}

fn reconstruct_path(came_from: HashMap<NodeIndex, NodeIndex>, start: NodeIndex, target: NodeIndex) -> Vec<NodeIndex> {
    let mut path = Vec::new();
    let mut current = target;

    if !came_from.contains_key(&target) {
        return path;
    }

    while current != start {
        path.push(current);
        current = came_from[&current];
    }

    path.push(start);
    path.reverse();

    path
}

impl<T> Default for VecGraph<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'graph, T> Iterator for Successors<'graph, T> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.current_edge_index {
            Some(edge_index) => {
                if let Some(edge) = self.graph.edges.get(edge_index.0) {
                    self.current_edge_index = edge.next_outgoing_edge;
                    Some(edge.target)
                } else {
                    panic!("Edge not found!");
                }
            },
            None => None
        }
    }
}

impl<T> VecGraph<T> {

    /// Return a [`Successors`] that can be used to iterate over the nodes that are connected to 'source'.
    ///
    /// # Arguments
    ///  * 'source' - The source node.
    /// 
    /// # Panics
    ///
    /// Panics if 'source' contains an index that does not correspond to an existing node.
    pub fn successors(&self, source: NodeIndex) -> Successors<T> {
        if let Some(n) = self.nodes.get(source.0) {
            Successors { graph: self, current_edge_index: n.first_outgoing_edge}
        } else {
            panic!("Source not not found!");
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeIndex(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeIndex(pub usize);

#[derive(Clone)]
struct NodeData<T> {
    data: T,
    first_outgoing_edge: Option<EdgeIndex>,
}

struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

pub struct Successors<'graph, T> {
    graph: &'graph VecGraph<T>,
    current_edge_index: Option<EdgeIndex>
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn graph_creation_works() {
        let mut graph: VecGraph<usize> = VecGraph::new();

        let n0 = graph.add_node(1);
        let n1 = graph.add_node(2);
        let n2 = graph.add_node(3);
        let n3 = graph.add_node(4);

        graph.add_edge(n0, n1);
        graph.add_edge(n1, n2);
        graph.add_edge(n0, n3);
        graph.add_edge(n3, n2);

        let s1 = graph.successors(n0).collect::<Vec<_>>();
        assert_eq!(&s1, &[n3, n1]); 

        graph.add_edge(n3, n0);
        graph.add_edge(n3, n3);

        let s2 = graph.successors(n3).collect::<Vec<_>>();
        assert_eq!(&s2, &[n3, n0, n2]);
    }

    #[test]
    fn can_create_grid() {
        let mut grid: VecGraph<&str> = VecGraph::new();

        let n0 = grid.add_node("middle");
        let n1 = grid.add_node("one");
        let n2 = grid.add_node("two");
        let n3 = grid.add_node("three");
        let n4 = grid.add_node("four");

        grid.add_edge(n0, n1);
        grid.add_edge(n0, n2);
        grid.add_edge(n0, n3);
        grid.add_edge(n0, n4);

        let s1 = grid.successors(n0).collect::<Vec<_>>();
        assert!(s1.len() == 4);
        let values = s1.iter().map(|i| *grid.get_data(i)).collect::<Vec<_>>();
        assert_eq!(&values, &["four", "three", "two", "one"]);
    }

    #[test]
    fn simple_dijkstra_works() {
        let mut graph : VecGraph<usize> = VecGraph::new();
    
        let n0 = graph.add_node(0);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);

        graph.add_edge(n0, n1);
        graph.add_edge(n1, n0);
        graph.add_edge(n1, n2);
        graph.add_edge(n2, n3);
        graph.add_edge(n3, n4);

        let path = graph.dijkstra(n0, n4, |&v| v);

        assert_eq!(&path, &[n0, n1, n2, n3, n4]);

        graph.add_edge(n0, n3);

        let path = graph.dijkstra(n0, n4, |&v| v);
        assert_eq!(&path, &[n0, n3, n4]);

        graph.add_edge(n0, n4);
        
        let path = graph.dijkstra(n0, n4, |&v| v);
        assert_eq!(&path, &[n0, n4]);
    }
}