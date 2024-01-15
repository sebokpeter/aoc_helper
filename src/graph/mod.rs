// Represents a set of nodes connected by edges
pub trait Graph {
    type DataType; // Type of the data contained by each node
    type NodeReference;
    type EdgeReference;

    
    /// Create a new Graph.
    fn new() -> Self where Self: Sized;

    
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
    fn get_data(&self, node: Self::NodeReference) -> &Self::DataType;
}

pub mod vec_graph;