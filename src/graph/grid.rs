#![allow(dead_code)]
use crate::direction::{relative_direction::RelativeDirection, Direction};

use super::{
    vec_graph::{Successors, VecGraph},
    EdgeIndex, Graph, NodeIndex,
};

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

    fn get_data(&self, node: &Self::NodeReference) -> &Self::DataType {
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

    pub fn successors(&self, source: NodeIndex) -> Successors<T> {
        self.graph.successors(source)
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
}

impl<T: Clone + std::fmt::Display> Grid<T> {
    pub fn print(&self) {
        if let Some(data) = &self.node_indices {
            for row in data {
                for d in row {
                    print!("{}", self.get_data(d));
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
                        print!("{}", self.get_data(d))
                    }
                }
                println!();
            }
        } else {
            println!("EMPTY");
        }
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

        assert_eq!(*grid.get_data(&f_index.unwrap()), 1);
    }

    #[test]
    fn last_index_works() {
        let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];

        let grid = Grid::new_from_data(data);

        let l_index = grid.last_index();

        assert!(l_index.is_some());
        assert_eq!(l_index.unwrap().0, 8);

        assert_eq!(*grid.get_data(&l_index.unwrap()), 9);
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
}
