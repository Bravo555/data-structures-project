#![feature(map_first_last)]

mod adj_list;
mod adj_matrix;
mod bundle;
mod incidence_matrix;

use std::collections::{BTreeMap, HashMap};

pub use adj_list::AdjList;
pub use adj_matrix::AdjMatrix;
pub use bundle::Bundle;
pub use incidence_matrix::IncidenceMatrix;

type NodeIndex = u32;
type Weight = i32;

pub trait Graph {
    /// Returns the amount of nodes in the graph
    fn len_nodes(&self) -> NodeIndex;

    /// Ensures that the node of the provided index is in the graph
    /// Depending on the implementation, it might add nodes of smaller indexes as well
    fn add_node(&mut self, node: NodeIndex);

    /// Connects nodes of given indices together in both directions
    /// Panics if the nodes we're trying to connect do not exist in the graph
    fn connect(&mut self, n1: NodeIndex, n2: NodeIndex, weight: i32);

    /// Checks if there is a connection from `n1` to `n2`
    fn nodes_connected(&self, n1: NodeIndex, n2: NodeIndex) -> bool;

    /// Returns a weight of a connection from `n1` to `n2` if it exists
    /// If it doesn't exist, the function panics.
    fn distance(&self, n1: NodeIndex, n2: NodeIndex) -> Weight;

    /// Return an amount of heap allocated memory necessary to store the graph's nodes and vertices.
    /// This is usually calculated by multiplying the underlying vector's length by the size of the stored type.
    /// Depending on an actual memory allocator used, the real amount of memory taken may vary.
    // TODO: consider whether to print vector's length or capacity (latter one is actual memory used and we can explain amortisation)
    fn memory(&self) -> usize;

    fn num_neighbours(&self, n: NodeIndex) -> usize;

    fn node_neighbours(&self, n: NodeIndex) -> Vec<(NodeIndex, Weight)>;

    fn graph_connected(&self) -> bool;

    /// Returns best paths to all the nodes from the selected start node using Dijkstra's algorithm
    fn dijkstra(&self, start: NodeIndex) -> HashMap<NodeIndex, (NodeIndex, Weight)> {
        let mut nodes: BTreeMap<NodeIndex, (NodeIndex, Weight)> = BTreeMap::new();
        nodes.insert(start, (start, 0));
        let mut finished = HashMap::new();

        while !nodes.is_empty() {
            let (u, (v, d)) = nodes.pop_first().unwrap();

            let neighbours = self
                .node_neighbours(u)
                .into_iter()
                .filter(|(idx, _)| !finished.contains_key(idx));

            for (neighbour, weight) in neighbours {
                if let None = nodes.get_mut(&neighbour).map(|pos| {
                    if d + weight < pos.1 {
                        pos.0 = u;
                        pos.1 = d + weight;
                    }
                    pos
                }) {
                    nodes.insert(neighbour, (u, d + weight));
                }
            }

            finished.insert(u, (v, d));
        }

        assert_eq!(finished.len(), self.len_nodes() as usize);
        finished
    }
}
