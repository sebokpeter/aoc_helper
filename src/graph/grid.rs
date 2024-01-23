#![allow(dead_code)]
use std::fmt::Display;

use crate::direction::{relative_direction::RelativeDirection, Direction};

use super::{vec_graph::VecGraph, EdgeIndex, Graph, GraphIntoIterator, NodeIndex};

// A grid is a specialized form of a graph, where each node can connect to two (if the node is on the corners), three (if the node is on the edge), or four other nodes.
pub struct Grid<T: Clone> {
    pub node_indices: Option<Vec<Vec<NodeIndex>>>,
    graph: VecGraph<T>,
}

impl<T: Clone> Graph for Grid<T> {
    type DataType = T;
    type NodeReference = NodeIndex;
    type EdgeReference = EdgeIndex;

    fn new() -> Self
    where
        Self: Sized,
    {
        Grid {
            graph: VecGraph::new(),
            node_indices: None,
        }
    }

    fn add_node(&mut self, data: Self::DataType) -> Self::NodeReference {
        self.graph.add_node(data)
    }

    fn add_edge(&mut self, source: Self::NodeReference, target: Self::NodeReference) {
        // TODO: actually enforce grid restriction
        self.graph.add_edge(source, target)
    }

    fn get_data(&self, node: &Self::NodeReference) -> Option<&Self::DataType> {
        self.graph.get_data(node)
    }

    fn dijkstra<F>(
        &self,
        start: Self::NodeReference,
        target: Self::NodeReference,
        cost_fn: F,
    ) -> Vec<Self::NodeReference>
    where
        F: Fn(&Self::DataType) -> usize,
    {
        self.graph.dijkstra(start, target, cost_fn)
    }

    fn dijkstra_search_with_closure<S, D, C>(
        &self,
        frontier_fn: S,
        target_fn: D,
        cost_fn: C,
    ) -> Vec<Self::NodeReference>
    where
        S: Fn(&Self::DataType) -> bool,
        D: Fn(&Self::DataType) -> bool,
        C: Fn(&Self::DataType) -> usize,
    {
        self.graph
            .dijkstra_search_with_closure(frontier_fn, target_fn, cost_fn)
    }

    fn find<F>(&self, predicate: F) -> Option<Self::NodeReference>
    where
        F: Fn(&Self::DataType) -> bool,
    {
        self.graph.find(predicate)
    }

    fn get_data_mut(&mut self, node: &Self::NodeReference) -> Option<&mut Self::DataType> {
        self.graph.get_data_mut(node)
    }

    fn get_neighbors(&self, node: &Self::NodeReference) -> Vec<Self::NodeReference> {
        self.graph.get_neighbors(node)
    }

    fn find_nodes<F>(&self, predicate: F) -> Vec<Self::NodeReference>
    where
        F: Fn(&Self::DataType) -> bool,
    {
        self.graph.find_nodes(predicate)
    }

    fn to_dot_file<N, S>(&self, node_name_fn: N, node_style_fn: S) -> String
    where
        N: Fn(&Self::DataType) -> String,
        S: Fn(&Self::DataType) -> String,
    {
        self.graph.to_dot_file(node_name_fn, node_style_fn)
    }
}

impl<T: Clone> Grid<T> {
    /// Create a new grid from a vector of vectors.
    pub fn new_from_data(data: Vec<Vec<T>>) -> Grid<T> {
        let mut graph = VecGraph::new();
        let mut nodes = Vec::new();

        for row in 0..data.len() {
            nodes.push(Vec::new());

            for col in 0..data[row].len() {
                let current = graph.add_node(data[row][col].clone());

                nodes[row].push(current);
            }
        }

        for row in 0..nodes.len() {
            for col in 0..nodes[row].len() {
                let current = nodes[row][col];
                let neighbors = get_neighbors(&nodes, col, row);

                for neighbor in neighbors {
                    graph.add_edge(current, neighbor);
                }
            }
        }

        Grid {
            graph,
            node_indices: Some(nodes),
        }
    }

    /// Return the first [`NodeIndex`], if it exists.
    pub fn first_index(&self) -> Option<NodeIndex> {
        if let Some(indices) = &self.node_indices {
            let first_row = indices.first();
            if let Some(row) = first_row {
                row.first().copied()
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Return the last [`NodeIndex`], if it exits.
    pub fn last_index(&self) -> Option<NodeIndex> {
        if let Some(indices) = &self.node_indices {
            let first_row = indices.last();
            if let Some(row) = first_row {
                row.last().copied()
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Returns a reference to underlying graph of this [`Grid<T>`].
    pub fn get_underlying_graph(&self) -> &VecGraph<T> {
        &self.graph
    }
}

impl<T: Clone + Display> Grid<T> {
    pub fn print(&self) {
        if let Some(data) = &self.node_indices {
            for row in data {
                for d in row {
                    print!("{}", self.get_data(d).unwrap());
                }
                println!();
            }
        } else {
            println!("EMPTY");
        }
    }

    pub fn print_path(&self, path: &[NodeIndex]) {
        if let Some(data) = &self.node_indices {
            for row in data {
                for d in row {
                    if path.contains(d) {
                        print!("*");
                    } else {
                        print!("{}", self.get_data(d).unwrap())
                    }
                }
                println!();
            }
        } else {
            println!("EMPTY");
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &<Self as Graph>::NodeReference> {
        self.graph.iter() // Delegate the iteration to the underlying graph
    }
}

impl<T: Clone> IntoIterator for Grid<T> {
    type Item = <Self as Graph>::NodeReference;

    type IntoIter = GraphIntoIterator<VecGraph<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.graph.into_iter()
    }
}

fn get_neighbors<T>(grid: &Vec<Vec<T>>, col: usize, row: usize) -> Vec<T>
where
    T: Clone,
{
    assert!(!grid.is_empty() && grid.iter().all(|l| l.len() == grid[0].len()));

    let height = grid.len();
    let width = grid[0].len();

    let up = row != 0;
    let down = row < height - 1;
    let left = col != 0;
    let right = col < width - 1;

    let mut offsets = Vec::new();

    if up {
        offsets.push(RelativeDirection::get_offset(&RelativeDirection::Up))
    };
    if right {
        offsets.push(RelativeDirection::get_offset(&RelativeDirection::Right))
    };
    if down {
        offsets.push(RelativeDirection::get_offset(&RelativeDirection::Down))
    };
    if left {
        offsets.push(RelativeDirection::get_offset(&RelativeDirection::Left))
    };

    let mut res = Vec::new();

    offsets.reverse();

    for offset in offsets {
        let o_row = row as isize + offset.0 as isize;
        let o_col = col as isize + offset.1 as isize;

        res.push(grid[o_row as usize][o_col as usize].clone());
    }

    res
}

#[cfg(test)]
pub mod test {
    use crate::iter_ext::IterExt;

    use super::*;

    #[test]
    fn simple_grid_works() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let grid = Grid::new_from_data(data);

        assert!(grid.node_indices.is_some());
        assert_eq!(grid.node_indices.clone().unwrap().len(), 3);
        assert!(grid.node_indices.unwrap().iter().all(|r| r.len() == 3));
    }

    #[test]
    fn first_index_works() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let grid = Grid::new_from_data(data);

        let f_index = grid.first_index();

        assert!(f_index.is_some());
        assert_eq!(f_index.unwrap().0, 0);

        assert_eq!(*grid.get_data(&f_index.unwrap()).unwrap(), 1);
    }

    #[test]
    fn last_index_works() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let grid = Grid::new_from_data(data);

        let l_index = grid.last_index();

        assert!(l_index.is_some());
        assert_eq!(l_index.unwrap().0, 8);

        assert_eq!(*grid.get_data(&l_index.unwrap()).unwrap(), 9);
    }

    #[test]
    fn dijkstra_works() {
        let data = vec![vec![1, 1, 9], vec![9, 1, 9], vec![9, 1, 1]];

        let grid = Grid::new_from_data(data);

        let start = grid.first_index().unwrap();
        let goal = grid.last_index().unwrap();

        let path = grid.dijkstra(start, goal, |&v| v);

        assert_eq!(path.len(), 5);
        assert_eq!(path[0].0, 0);
        assert_eq!(path[1].0, 1);
        assert_eq!(path[2].0, 4);
        assert_eq!(path[3].0, 7);
        assert_eq!(path[4].0, 8);
    }

    #[test]
    fn can_use_iter() {
        let data = vec![vec![1, 1, 9], vec![9, 1, 9], vec![9, 1, 1]];

        let grid = Grid::new_from_data(data);

        let mut values = Vec::new();

        for index in grid.iter() {
            values.push(*grid.get_data(index).unwrap());
        }

        assert_eq!(values.len(), 9);
        assert_eq!(&values, &[1, 1, 9, 9, 1, 9, 9, 1, 1]);
    }

    #[test]
    fn can_use_into_iterator() {
        let data = vec![vec![1, 1], vec![9, 1]];

        let grid = Grid::new_from_data(data);

        let mut indices = Vec::new();

        for index in grid {
            indices.push(index);
        }

        assert_eq!(indices.len(), 4);
        assert_eq!(&indices.iter().map(|i| i.0).collect_vec(), &[0, 1, 2, 3]);
    }
}
