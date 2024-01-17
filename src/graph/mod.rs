// Represents a set of nodes connected by edges
pub trait Graph {
    type DataType; // Type of the data contained in each node
    type NodeReference;
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

    /// Retrieve the data stored in the node specified by 'node'.
    ///
    /// # Arguments
    ///
    /// * `node` - Index of reference of the node which contains the data.
    fn get_data(&self, node: &Self::NodeReference) -> &Self::DataType;

    /// Search the graph for the shortest route between 'start' and 'target', using Dijkstra’s Algorithm.
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
        F: Fn(&Self::DataType) -> usize;

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
    fn dijkstra_search_with_delegate<S, T, C>(&self, frontier_fn: S, target_fn: T, cost_fn: C) -> Vec<Self::NodeReference> 
    where 
        S: Fn(&Self::DataType) -> bool,
        T: Fn(&Self::DataType) -> bool,
        C: Fn(&Self::DataType) -> usize;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeIndex(pub usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeIndex(pub usize);

#[derive(Clone)]
struct NodeData<T> {
    data: T,
    index: NodeIndex,
    first_outgoing_edge: Option<EdgeIndex>,
}

struct EdgeData {
    target: NodeIndex,
    next_outgoing_edge: Option<EdgeIndex>,
}

pub mod vec_graph;
pub mod grid;