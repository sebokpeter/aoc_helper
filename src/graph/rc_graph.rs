#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

use crate::iter_ext::IterExt;

use super::{Graph, GraphIntoIterator, GraphIterator, NodeIndex};

/// A [`Graph`] implementation using [`Rc<RefCell<_>>`].
pub struct RcGraph<T: Clone> {
    nodes: Vec<Node<T>>,
}

impl<T: Clone> Graph for RcGraph<T> {
    type DataType = T;

    type NodeReference = NodeIndex;

    type EdgeReference = ();

    fn new() -> Self
    where
        Self: Sized,
    {
        RcGraph { nodes: Vec::new() }
    }

    fn add_node(&mut self, data: Self::DataType) -> Self::NodeReference {
        let index = NodeIndex(self.nodes.len());

        let node = Node {
            data,
            index,
            neighbors: Vec::new(),
        };
        self.nodes.push(node);

        index
    }

    /// Add a directed edge between the nodes represented by `source` and `target`.
    ///
    /// # Panics
    ///
    /// Panics if either `source` or `target` references an invalid node.
    ///
    /// # Example
    /// ```
    /// use aoc_helper::graph::{Graph, rc_graph::RcGraph};
    ///
    /// let mut graph = RcGraph::new();
    ///
    /// let n1 = graph.add_node(0);
    /// let n2 = graph.add_node(1);
    /// let n3 = graph.add_node(2);
    /// let n4 = graph.add_node(3);
    ///
    /// graph.add_edge(n1, n2);
    /// graph.add_edge(n1, n3);
    /// graph.add_edge(n1, n4);
    /// ```
    fn add_edge(&mut self, source: Self::NodeReference, target: Self::NodeReference) {
        let t = self.nodes[target.0].clone();
        let s = &mut self.nodes[source.0];

        s.neighbors.push(Rc::new(RefCell::new(t)));
    }

    fn get_data(&self, node: &Self::NodeReference) -> Option<&Self::DataType> {
        if let Some(n) = self.nodes.get(node.0) {
            Some(&n.data)
        } else {
            None
        }
    }

    fn get_data_mut(&mut self, node: &Self::NodeReference) -> Option<&mut Self::DataType> {
        if let Some(n) = self.nodes.get_mut(node.0) {
            Some(&mut n.data)
        } else {
            None
        }
    }

    /// Get the neighbors of `node`.
    ///
    /// # Panics
    ///
    /// Panics if `node` references an invalid [`Node`].
    ///
    /// # Example
    ///
    /// ```
    /// use aoc_helper::graph::{Graph, rc_graph::RcGraph};
    ///
    /// let mut graph = RcGraph::new();
    ///
    /// let n0 = graph.add_node(0);
    /// let n1 = graph.add_node(1);
    /// let n2 = graph.add_node(2);
    ///
    /// graph.add_edge(n0, n1);
    /// graph.add_edge(n0, n2);
    ///
    /// let n0_neighbors = graph.get_neighbors(&n0);
    ///
    /// assert_eq!(&n0_neighbors, &[n1, n2]);
    /// ```
    fn get_neighbors(&self, node: &Self::NodeReference) -> Vec<Self::NodeReference> {
        self.nodes[node.0]
            .neighbors
            .iter()
            .map(|n| n.borrow().index)
            .collect_vec()
    }

    fn find<F>(&self, predicate: F) -> Option<Self::NodeReference>
    where
        F: Fn(&Self::DataType) -> bool,
    {
        for node in &self.nodes {
            if predicate(&node.data) {
                return Some(node.index);
            }
        }

        None
    }

    fn find_nodes<F>(&self, predicate: F) -> Vec<Self::NodeReference>
    where
        F: Fn(&Self::DataType) -> bool,
    {
        self.nodes
            .iter()
            .filter(|node| predicate(&node.data))
            .map(|node| node.index)
            .collect()
    }
}

impl<T: Clone> RcGraph<T> {
    pub fn iter(&self) -> GraphIterator<RcGraph<T>> {
        GraphIterator { graph: self, index: 0 }
    }
}

impl<'a, T:Clone> Iterator for GraphIterator<'a, RcGraph<T>> {
    type Item = &'a <RcGraph<T> as Graph>::NodeReference;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.graph.nodes.len() {
            self.index += 1;
            Some(&self.graph.nodes[self.index - 1].index)
        } else {
            None
        }
    }
}

impl<T: Clone> Iterator for GraphIntoIterator<RcGraph<T>> {
    type Item = <RcGraph<T> as Graph>::NodeReference;

    fn next(&mut self) -> Option<Self::Item> {
        if self.graph.nodes.is_empty() {
            return None;
        }

        Some(self.graph.nodes.remove(0).index)
    }
}

impl<T: Clone> IntoIterator for RcGraph<T> {
    type Item = <Self as Graph>::NodeReference;

    type IntoIter = GraphIntoIterator<RcGraph<T>>;

    fn into_iter(self) -> Self::IntoIter {
        GraphIntoIterator {graph: self}
    }
}

#[derive(Clone)]
struct Node<T>
where
    T: Clone,
{
    data: T,
    index: <RcGraph<T> as Graph>::NodeReference,
    neighbors: Vec<Rc<RefCell<Node<T>>>>,
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn can_create_new_rc_graph() {
        let graph: RcGraph<usize> = RcGraph::new();

        assert!(graph.nodes.is_empty());
    }

    #[test]
    fn can_add_nodes() {
        let mut graph = RcGraph::new();

        let n1 = graph.add_node(0);
        let n2 = graph.add_node(1);

        assert_eq!(n1.0, 0);
        assert_eq!(n2.0, 1);
        assert_eq!(graph.nodes.len(), 2);
        assert_eq!(graph.nodes[0].data, 0);
        assert_eq!(graph.nodes[1].data, 1);
    }

    #[test]
    fn can_add_edge() {
        let mut graph = RcGraph::new();

        let n1 = graph.add_node(0);
        let n2 = graph.add_node(1);
        let n3 = graph.add_node(2);
        let n4 = graph.add_node(3);

        graph.add_edge(n1, n2);
        graph.add_edge(n1, n3);
        graph.add_edge(n1, n4);

        assert_eq!(graph.nodes[0].neighbors.len(), 3);
        assert_eq!(graph.nodes[0].neighbors[0].borrow().data, 1);
        assert_eq!(graph.nodes[0].neighbors[1].borrow().data, 2);
        assert_eq!(graph.nodes[0].neighbors[2].borrow().data, 3);

        {
            let mut neighbor_one = graph.nodes[0].neighbors[0].borrow_mut();
            neighbor_one.data = 100;
        }

        assert_eq!(graph.nodes[0].neighbors[0].borrow().data, 100);
    }

    #[test]
    fn get_data_invalid_index_returns_none() {
        let mut graph: RcGraph<usize> = RcGraph::new();

        let d1 = graph.get_data(&NodeIndex(usize::MIN));
        let d2 = graph.get_data(&NodeIndex(usize::MAX));

        assert!(d1.is_none());
        assert!(d2.is_none());

        graph.add_node(0);
        graph.add_node(1);

        let d1 = graph.get_data(&NodeIndex(2));
        let d2 = graph.get_data(&NodeIndex(usize::MAX));

        assert!(d1.is_none());
        assert!(d2.is_none());
    }

    #[test]
    fn get_data_valid_index_returns_reference() {
        let mut graph: RcGraph<usize> = RcGraph::new();

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);

        let d1 = graph.get_data(&NodeIndex(0));
        let d2 = graph.get_data(&NodeIndex(1));
        let d3 = graph.get_data(&NodeIndex(2));

        assert!(d1.is_some());
        assert!(d2.is_some());
        assert!(d3.is_some());

        assert_eq!(*d1.unwrap(), 0);
        assert_eq!(*d2.unwrap(), 1);
        assert_eq!(*d3.unwrap(), 2);
    }

    #[test]
    fn get_data_mut_invalid_index_returns_none() {
        let mut graph: RcGraph<usize> = RcGraph::new();

        let d1 = graph.get_data_mut(&NodeIndex(usize::MIN));
        assert!(d1.is_none());

        let d2 = graph.get_data_mut(&NodeIndex(usize::MAX));
        assert!(d2.is_none());

        graph.add_node(0);
        graph.add_node(1);

        let d1 = graph.get_data_mut(&NodeIndex(2));
        assert!(d1.is_none());

        let d2 = graph.get_data_mut(&NodeIndex(usize::MAX));
        assert!(d2.is_none());
    }

    #[test]
    fn get_data_mut_valid_index_returns_reference() {
        let mut graph: RcGraph<usize> = RcGraph::new();

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);

        let d1 = graph.get_data_mut(&NodeIndex(0));
        assert!(d1.is_some());
        assert_eq!(*d1.unwrap(), 0);

        let d2 = graph.get_data_mut(&NodeIndex(1));
        assert!(d2.is_some());
        assert_eq!(*d2.unwrap(), 1);

        let d3 = graph.get_data_mut(&NodeIndex(2));
        assert!(d3.is_some());
        assert_eq!(*d3.unwrap(), 2);
    }

    #[test]
    fn get_data_mut_valid_index_can_mutate_reference() {
        #[derive(Clone)]
        struct TestData {
            d1: usize,
            d2: usize,
        }

        let mut graph: RcGraph<TestData> = RcGraph::new();

        graph.add_node(TestData { d1: 0, d2: 0 });

        let test = graph.get_data_mut(&NodeIndex(0)).unwrap();
        test.d1 = usize::MAX;
        test.d2 = usize::MAX;

        assert_eq!(graph.nodes[0].data.d1, usize::MAX);
        assert_eq!(graph.nodes[0].data.d2, usize::MAX);
    }

    #[test]
    fn can_get_neighbors() {
        let mut graph = RcGraph::new();

        let n0 = graph.add_node(0);
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);

        graph.add_edge(n0, n1);
        graph.add_edge(n0, n2);

        let n0_neighbors = graph.get_neighbors(&n0);

        assert_eq!(&n0_neighbors, &[n1, n2]);
    }

    #[test]
    fn find_no_match_returns_none() {
        let mut graph: RcGraph<usize> = RcGraph::new();

        let d1 = graph.find(|&d| d == 0);
        assert!(d1.is_none());

        graph.add_node(0);
        graph.add_node(1);

        let d2 = graph.find(|&d| d == 2);
        let d3 = graph.find(|&d| d == usize::MAX);

        assert!(d2.is_none());
        assert!(d3.is_none());
    }

    #[test]
    fn can_find_node_data() {
        let mut graph = RcGraph::new();

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);

        let d1 = graph.find(|&d| d == 0);
        assert!(d1.is_some());
        assert_eq!(d1.unwrap(), NodeIndex(0));

        let d3 = graph.find(|&d| d == 2);
        assert!(d3.is_some());
        assert_eq!(d3.unwrap(), NodeIndex(2));

        let mut graph = RcGraph::new();
        graph.add_node("Hello");
        graph.add_node("Graph");
        graph.add_node("!");

        let d1 = graph.find(|&d| d == "Hello");
        assert!(d1.is_some());
        assert_eq!(d1.unwrap(), NodeIndex(0));

        let d2 = graph.find(|&d| d == "Graph");
        assert!(d2.is_some());
        assert_eq!(d2.unwrap(), NodeIndex(1));

        let d3 = graph.find(|&d| d == "!");
        assert!(d3.is_some());
        assert_eq!(d3.unwrap(), NodeIndex(2));
    }

    #[test]
    fn find_nodes_no_match_returns_empty_vec() {
        let mut graph = RcGraph::new();

        let vec_one = graph.find_nodes(|&d| d > 100);
        assert!(vec_one.is_empty());

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(100);

        let vec_one = graph.find_nodes(|&d| d > 100);
        assert!(vec_one.is_empty());
    }

    #[test]
    fn can_find_nodes() {
        let mut graph = RcGraph::new();

        graph.add_node("Hello World!");
        graph.add_node("Hello Graph!");
        graph.add_node("Hello AoC");

        let hellos = graph.find_nodes(|&v| v.starts_with("Hello"));
        assert_eq!(hellos.len(), 3);

        let aoc = graph.find_nodes(|&v| v.ends_with("AoC"));

        assert_eq!(aoc.len(), 1);
        assert_eq!(aoc[0].0, 2);
    }

    #[test]
    fn can_use_iter() {
        let mut graph = RcGraph::new();

        graph.add_node("One!");
        graph.add_node("Two!");
        graph.add_node("Three!");
    
        let mut values = Vec::new();

        for index in graph.iter() {
            values.push(*graph.get_data(index).unwrap());
        }

        assert_eq!(values.len(), 3);
        assert_eq!(&values, &["One!", "Two!", "Three!"]);
    }

    #[test]
    fn can_use_into_iter() {
        let mut graph = RcGraph::new();

        graph.add_node("One!");
        graph.add_node("Two!");
        graph.add_node("Three!");
    
        let mut indices = Vec::new();

        for index in graph {
            indices.push(index);
        }

        assert_eq!(indices.len(), 3);
    }

    #[test]
    fn dijkstra_no_path_returns_empty_vec() {
        let mut graph = RcGraph::new();

        let start = graph.add_node(0);
        let n1 = graph.add_node(1000);
        let n2 = graph.add_node(1);
        let n3 = graph.add_node(2);
        let destination = graph.add_node(3);

        // No connection to `destination`
        graph.add_edge(start, n1);
        graph.add_edge(start, n2);
        graph.add_edge(n2, n3);
        graph.add_edge(n1, n3);
        graph.add_edge(n3, start);

        let cost_fn = |data: &usize| *data;
        let path = graph.dijkstra(start, destination, cost_fn);

        assert!(path.is_empty());
    }

    #[test]
    fn can_find_shortest_path_with_dijkstra() {
        let mut graph = RcGraph::new();

        let start = graph.add_node(0);
        let n1 = graph.add_node(1000);
        let n2 = graph.add_node(1);
        let n3 = graph.add_node(2);
        let destination = graph.add_node(3);

        // Shortest path: start -> n2 -> n3 -> destination
        graph.add_edge(start, n1);
        graph.add_edge(start, n2);
        graph.add_edge(n2, n3);
        graph.add_edge(n1, n3);
        graph.add_edge(n3, destination);

        let cost_fn = |data: &usize| *data;
        let path = graph.dijkstra(start, destination, cost_fn);

        assert_eq!(path.len(), 4);
        assert_eq!(&path, &[start, n2, n3, destination]);
    }

    #[test]
    fn dijkstra_with_closure_no_path_returns_empty_vec() {
        let mut graph = RcGraph::new();

        let start = graph.add_node(0);
        let n1 = graph.add_node(1000);
        let n2 = graph.add_node(1);
        let n3 = graph.add_node(2);
        let _destination = graph.add_node(3);

        // No connection to `destination`
        graph.add_edge(start, n1);
        graph.add_edge(start, n2);
        graph.add_edge(n2, n3);
        graph.add_edge(n1, n3);
        graph.add_edge(n3, start);

        let frontier_fn = |data: &usize| *data == 0;
        let target_fn = |data: &usize| *data == 3;
        let cost_fn = |data: &usize| *data;
        let path = graph.dijkstra_search_with_closure(frontier_fn, target_fn, cost_fn);

        assert!(path.is_empty());
    }

    #[test]
    fn can_find_shortest_path_with_dijkstra_with_closure() {
        let mut graph = RcGraph::new();

        let start = graph.add_node(0);
        let n1 = graph.add_node(1000);
        let n2 = graph.add_node(1);
        let n3 = graph.add_node(2);
        let destination = graph.add_node(3);

        // Shortest path: start -> n2 -> n3 -> destination
        graph.add_edge(start, n1);
        graph.add_edge(start, n2);
        graph.add_edge(n2, n3);
        graph.add_edge(n1, n3);
        graph.add_edge(n3, destination);

        let frontier_fn = |data: &usize| *data == 0; // The start node is the node where the node's data is 0
        let target_fn = |data: &usize| *data == 3; // The end node us the node where the node's data is 3
        let cost_fn = |data: &usize| *data;
        let path = graph.dijkstra_search_with_closure(frontier_fn, target_fn, cost_fn);

        assert_eq!(path.len(), 4);
        assert_eq!(&path, &[start, n2, n3, destination]);
    }
}
