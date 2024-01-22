#![allow(dead_code)]
use crate::{
    direction::relative_direction::RelativeDirection, geometry::point::Point2D, iter_ext::IterExt,
};

use super::{EdgeIndex, Graph, GraphIntoIterator, GraphIterator, NodeIndex};

// An implementation of a graph datastructure, using vectors to store nodes and edges.
// Based on: https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/
pub struct VecGraph<T> {
    nodes: Vec<NodeData<T>>,
    edges: Vec<EdgeData>,
}

impl<T> Default for VecGraph<T> {
    fn default() -> Self {
        Self::new()
    }
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
            index: NodeIndex(index),
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

    fn get_data(&self, node: &Self::NodeReference) -> Option<&Self::DataType> {
        if let Some(node_data) = self.nodes.get(node.0) {
            Some(&node_data.data)
        } else {
            None
        }
    }

    fn get_data_mut(&mut self, node: &Self::NodeReference) -> Option<&mut Self::DataType> {
        if let Some(node_data) = self.nodes.get_mut(node.0) {
            Some(&mut node_data.data)
        } else {
            None
        }
    }

    fn get_neighbors(&self, node: &Self::NodeReference) -> Vec<Self::NodeReference> {
        self.successors(*node).collect_vec()
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

impl<'a, T> Iterator for GraphIterator<'a, VecGraph<T>> {
    type Item = &'a <VecGraph<T> as Graph>::NodeReference;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.graph.nodes.len() {
            self.index += 1;
            Some(&self.graph.nodes[self.index - 1].index)
        } else {
            None
        }
    }
}

impl<T> Iterator for GraphIntoIterator<VecGraph<T>> {
    type Item = <VecGraph<T> as Graph>::NodeReference;

    fn next(&mut self) -> Option<Self::Item> {
        if self.graph.nodes.is_empty() {
            return None;
        }

        Some(self.graph.nodes.remove(0).index)
    }
}

impl<T> IntoIterator for VecGraph<T> {
    type Item = <VecGraph<T> as Graph>::NodeReference;

    type IntoIter = GraphIntoIterator<VecGraph<T>>;

    fn into_iter(self) -> Self::IntoIter {
        GraphIntoIterator {graph: self}
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
            Successors {
                graph: self,
                current_edge_index: n.first_outgoing_edge,
            }
        } else {
            panic!("Source not not found!");
        }
    }

    pub fn iter(&self) -> GraphIterator<VecGraph<T>> {
        GraphIterator { graph: self, index: 0 }
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
            }
            None => None,
        }
    }
}

pub struct Successors<'graph, T> {
    graph: &'graph VecGraph<T>,
    current_edge_index: Option<EdgeIndex>,
}

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

#[cfg(test)]
pub mod test {
    use std::collections::HashMap;

    use crate::{direction::Direction, geometry::point::Point2D};

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
        let values = s1
            .iter()
            .map(|i| *grid.get_data(i).unwrap())
            .collect::<Vec<_>>();
        assert_eq!(&values, &["four", "three", "two", "one"]);
    }

    #[test]
    fn get_neighbors_works() {
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

        let neighbors = grid.get_neighbors(&n0);

        assert_eq!(&neighbors, &[n4, n3, n2, n1]);

        grid.add_edge(n1, n4);
        grid.add_edge(n1, n3);
        grid.add_edge(n1, n2);
        grid.add_edge(n1, n0);

        let neighbors = grid.get_neighbors(&n1);

        assert_eq!(&neighbors, &[n0, n2, n3, n4]);
    }

    #[test]
    fn find_works() {
        let mut graph = VecGraph::new();

        let one = graph.add_node("One");
        let two = graph.add_node("Two");
        let three = graph.add_node("Three");

        graph.add_edge(one, two);
        graph.add_edge(two, three);
        graph.add_edge(three, one);

        let find_one = graph.find(|&d| d == "One");
        assert!(find_one.is_some());
        assert_eq!(one, find_one.unwrap());

        let find_two = graph.find(|&d| d == "Two");
        assert!(find_two.is_some());
        assert_eq!(two, find_two.unwrap());

        let find_three = graph.find(|&d| d == "Three");
        assert!(find_three.is_some());
        assert_eq!(three, find_three.unwrap());
    }

    #[test]
    fn find_nodes_works() {
        let mut graph = VecGraph::new();

        graph.add_node(0);
        graph.add_node(2);
        graph.add_node(4);

        let o1 = graph.add_node(1);
        let o2 = graph.add_node(3);
        let o3 = graph.add_node(5);

        let odd = graph.find_nodes(|n| n % 2 == 1);

        assert_eq!(odd.len(), 3);
        assert_eq!(&odd, &[o1, o2, o3]);
    }

    #[test]
    fn can_iterate_node_indices_reference() {
        let mut graph: VecGraph<usize> = VecGraph::new();

        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);

        let mut graph_data = Vec::new();
        for node in graph.iter() {
            graph_data.push(*graph.get_data(node).unwrap());
        }

        assert_eq!(graph_data.len(), 4);
        assert_eq!(&graph_data, &[1, 2, 3, 4]);
    }

    #[test]
    fn can_iterate_with_into_iter() {
        let mut graph: VecGraph<usize> = VecGraph::new();

        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);

        let mut indices = Vec::new();

        for node in graph {
            indices.push(node);
        }

        assert_eq!(indices.len(), 4);
    }

    #[test]
    fn simple_dijkstra_works() {
        let mut graph: VecGraph<usize> = VecGraph::new();

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

    #[test]
    fn dijkstra_search_with_closure_works() {
        // Example data from AoC 2023 Day 17
        let example = r"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

        let values = parse_values(example);

        let (mut graph, indices) = create_graph(&values);

        for (state, index) in &indices {
            let connections = get_connections(state, &indices);

            connections.iter().for_each(|c| {
                graph.add_edge(*index, *c);
            });
        }

        let last_row = values.len() - 1;
        let last_col = values.last().unwrap().len() - 1;

        let frontier_fn = |n: &State| n.pos.x == 0 && n.pos.y == 0; // Closure that select states corresponding to the starting position.
        let target_fn = |n: &State| n.pos.x == last_col && n.pos.y == last_row; // We only care about the position of the target node.
        let cost_fn = |n: &State| n.heat_loss; // The 'cost' of moving to a state is the heat loss at that state.

        let path = &graph.dijkstra_search_with_closure(frontier_fn, target_fn, cost_fn)[1..]; // First element is the start state, which we don't need to count
        let states = path.iter().map(|p| graph.get_data(p)).collect_vec();

        let total_heat_loss = states
            .iter()
            .map(|s| s.unwrap())
            .map(|s| s.heat_loss)
            .sum::<usize>();

        //print_path(&states, &values);

        assert_eq!(total_heat_loss, 102);
    }

    #[allow(clippy::ptr_arg)]
    fn print_path(path: &[&State], values: &Vec<Vec<usize>>) {
        for (row, row_values) in values.iter().enumerate() {
            for (col, heat_loss) in row_values.iter().enumerate() {
                if let Some(s) = path
                    .iter()
                    .find(|&state| state.pos.x == col && state.pos.y == row)
                {
                    print!("{}", get_char_to_print(s));
                } else {
                    print!("{}", heat_loss);
                }
            }
            println!();
        }
    }

    fn get_char_to_print(s: &State) -> char {
        match s.incoming_direction {
            RelativeDirection::Up => 'v',
            RelativeDirection::Right => '<',
            RelativeDirection::Down => '^',
            RelativeDirection::Left => '>',
        }
    }

    /// Create the graph by creating all possible states.
    /// The states (and the connections between them) encode the rules defined in the challenge
    /// For example, if we are in a state where: { pos: (1, 2), continuous_steps: 2, incoming_direction: Up }, then we know that there are 3 possible connections:
    /// Moving straight: {pos: (1, 3), continuous_steps: 3, incoming_direction: Up}
    /// Moving left: {pos: (0, 2), continuous_steps: 1, incoming_direction: Right}
    /// Moving right: {pos: (2, 2), continuous_steps: 1, incoming_direction: Left}
    #[allow(clippy::ptr_arg)]
    fn create_graph(values: &Vec<Vec<usize>>) -> (VecGraph<State>, HashMap<State, NodeIndex>) {
        let mut graph = VecGraph::new();
        let mut indices = HashMap::new();

        for (row_index, row) in values.iter().enumerate() {
            for (col_index, &heat_loss) in row.iter().enumerate() {
                if col_index == 0 && row_index == 0 {
                    continue; // Skip the first position
                }

                // We can enter a position from any direction
                for direction in RelativeDirection::all() {
                    // We can take 1, 2, or 3 steps in a row
                    for step_count in 1..=3 {
                        let state = State {
                            heat_loss,
                            pos: Point2D {
                                x: col_index,
                                y: row_index,
                            },
                            continuous_steps: step_count,
                            incoming_direction: direction,
                        };

                        let index = graph.add_node(state.clone());
                        indices.insert(state, index);
                    }
                }
            }
        }

        // Manually create starting states

        let start_up = State {
            continuous_steps: 0,
            pos: Point2D { x: 0, y: 0 },
            heat_loss: values[0][0],
            incoming_direction: RelativeDirection::Up,
        };
        let start_left = State {
            continuous_steps: 0,
            pos: Point2D { x: 0, y: 0 },
            heat_loss: values[0][0],
            incoming_direction: RelativeDirection::Left,
        };

        let start_up_index = graph.add_node(start_up.clone());
        let start_left_index = graph.add_node(start_left.clone());

        indices.insert(start_up, start_up_index);
        indices.insert(start_left, start_left_index);

        (graph, indices)
    }

    /// Create connections between states.
    /// This helps encode the rules described above.
    fn get_connections(state: &State, indices: &HashMap<State, NodeIndex>) -> Vec<NodeIndex> {
        // For a given state, there are three directions we need to check (we cannot move backwards):
        // 1. Moving forward - in the direction opposite of the current state's incoming_direction. Only possible if the current state's continuous_steps is less than 3. Increment step count by 1.
        // 2. Moving left - reset step count to 1
        // 3. Moving right - reset step count to 1

        let mut res = Vec::with_capacity(3);

        let incoming_dir = state.incoming_direction;

        let forward = RelativeDirection::get_opposite(&incoming_dir);
        let left = RelativeDirection::get_left(&forward);
        let right = RelativeDirection::get_right(&forward);

        if let (Some(forward_pos), true) = (
            apply_direction(&state.pos, &forward),
            state.continuous_steps < 3,
        ) {
            add_next_state(
                &mut res,
                indices,
                forward_pos,
                incoming_dir,
                state.continuous_steps + 1,
            );
        }

        if let Some(left_pos) = apply_direction(&state.pos, &left) {
            add_next_state(
                &mut res,
                indices,
                left_pos,
                RelativeDirection::get_opposite(&left),
                1,
            );
        }

        if let Some(right_pos) = apply_direction(&state.pos, &right) {
            add_next_state(
                &mut res,
                indices,
                right_pos,
                RelativeDirection::get_opposite(&right),
                1,
            );
        }

        res
    }

    fn add_next_state(
        res: &mut Vec<NodeIndex>,
        indices: &HashMap<State, NodeIndex>,
        next_pos: Point2D<usize>,
        incoming_dir: RelativeDirection,
        steps: usize,
    ) {
        let opt = search_hash_map(indices, next_pos, incoming_dir, steps);

        if let Some(index) = opt {
            res.push(index);
        }
    }

    /// A *VERY* inefficient way of finding a NodeIndex. The method will iterate over the entire HashMap.
    /// TODO: faster solution - perhaps order states (put them into buckets?) when creating them?
    fn search_hash_map(
        indices: &HashMap<State, NodeIndex>,
        next_pos: Point2D<usize>,
        incoming_dir: RelativeDirection,
        steps: usize,
    ) -> Option<NodeIndex> {
        let next_state_index = indices.iter().find_map(|(key, value)| {
            if key.pos == next_pos
                && key.incoming_direction == incoming_dir
                && key.continuous_steps == steps
            {
                Some(value)
            } else {
                None
            }
        });

        next_state_index.copied()
    }

    /// Check if moving in 'dir' would move us out of bounds. If yes, return None, otherwise calculate and return the new position
    fn apply_direction(pos: &Point2D<usize>, dir: &RelativeDirection) -> Option<Point2D<usize>> {
        let (row_offset, col_offset) = RelativeDirection::get_offset(dir);

        if col_offset == -1 && pos.x == 0 {
            return None;
        }

        if row_offset == -1 && pos.y == 0 {
            return None;
        }

        // Cast to isize so that we can add a possibly negative number
        let new_x = ((pos.x as isize) + (col_offset as isize)) as usize;
        let new_y = ((pos.y as isize) + (row_offset as isize)) as usize;

        Some(Point2D { x: new_x, y: new_y })
    }

    fn parse_values(example: &str) -> Vec<Vec<usize>> {
        example
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect_vec()
            })
            .collect_vec()
    }
}

// Encodes a possible state we can be in while traversing the graph
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    pos: Point2D<usize>,                   // Original position in graph/grid
    heat_loss: usize,                      // Original heat loss value for the given position
    incoming_direction: RelativeDirection, // From which direction did we enter into this position
    continuous_steps: usize, // How many steps have we taken before entering this position
}
