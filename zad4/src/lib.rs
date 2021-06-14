mod adj_list;
mod adj_matrix;

pub use adj_list::AdjList;
pub use adj_matrix::AdjMatrix;

type NodeIndex = u32;
type Weight = i32;

pub trait Graph {
    /// Returns the amount of nodes in the graph
    fn len(&self) -> NodeIndex;

    /// Ensures that the node of the provided index is in the graph
    /// Depending on the implementation, it might add nodes of smaller indexes as well
    fn add_node(&mut self, node: NodeIndex);

    /// Connects nodes of given indices together in both directions
    /// Panics if the nodes we're trying to connect do not exist in the graph
    fn connect(&mut self, n1: NodeIndex, n2: NodeIndex, weight: i32);

    /// Checks if there is a connection from `n1` to `n2`
    fn connected(&self, n1: NodeIndex, n2: NodeIndex) -> bool;

    /// Returns a weight of a connection from `n1` to `n2` if it exists
    /// If it doesn't exist, the function panics.
    fn distance(&self, n1: NodeIndex, n2: NodeIndex) -> Weight;

    /// Return an amount of heap allocated memory necessary to store the graph's nodes and vertices.
    /// This is usually calculated by multiplying the underlying vector's length by the size of the stored type.
    /// Depending on an actual memory allocator used, the real amount of memory taken may vary.
    // TODO: consider whether to print vector's length or capacity (latter one is actual memory used and we can explain amortisation)
    fn memory(&self) -> usize;

    fn num_neighbours(&self, n: NodeIndex) -> usize;

    /// Returns best paths to all the nodes from the selected start node using Dijkstra's algorithm
    fn dijkstra(&self, start: NodeIndex) -> Vec<(NodeIndex, NodeIndex, Weight)> {
        let mut nodes: Vec<_> = vec![(start, start, 0)];
        let mut finished = vec![];

        while nodes.len() > 0 {
            let (i, _) = nodes
                .iter()
                .enumerate()
                .min_by_key(|(_, (_, _, d))| d)
                .unwrap();
            let node = nodes.remove(i);
            let (u, _, d) = node;

            let neighbours = (0..self.len()).filter(|idx| {
                self.connected(u, *idx) && finished.iter().find(|(v, _, _)| *v == *idx).is_none()
            });

            for neighbour in neighbours {
                // TODO: looking for a value 2 times, fix
                let pos = nodes.iter().position(|(v, _, _)| *v == neighbour);
                match pos {
                    None => {
                        nodes.push((neighbour, u, d + self.distance(u, neighbour)));
                    }
                    Some(pos) => {
                        let n_entry = nodes.get_mut(pos).unwrap();
                        if d + self.distance(u, neighbour) < n_entry.2 {
                            n_entry.1 = u;
                            n_entry.2 = d + self.distance(u, neighbour);
                        }
                    }
                }
            }

            finished.push(node);
        }

        finished
    }
}
