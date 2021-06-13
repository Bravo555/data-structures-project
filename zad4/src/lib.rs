use rand::{distributions::Uniform, prelude::*};
use std::{collections::HashSet, mem};

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

#[derive(Debug)]
pub struct AdjMatrix {
    mat: Vec<Weight>,
    len: NodeIndex,
}

impl AdjMatrix {
    pub fn new() -> Self {
        AdjMatrix {
            mat: Vec::new(),
            len: 0,
        }
    }

    pub fn random_connected(num_nodes: usize, edge_probability: f32, rng: &mut impl Rng) -> Self {
        let mut graph = Self::new();
        graph.add_node((num_nodes - 1) as NodeIndex);

        // first we connect all unordered pairs of the graph so that it is connected
        let mut unvisited_set = HashSet::new();

        for node in 0..graph.len() {
            unvisited_set.insert(node);
        }

        let mut unvisited_set = unvisited_set.into_iter().collect::<Vec<_>>();
        unvisited_set.shuffle(rng);
        let mut cur_vertex = unvisited_set.pop().expect("no nodes in the graph");
        let weight_dist = Uniform::from(0..=20);

        while !unvisited_set.is_empty() {
            let adj_vertex = unvisited_set.pop().unwrap();
            let weight = weight_dist.sample(rng);
            graph.connect(cur_vertex, adj_vertex, weight);
            cur_vertex = adj_vertex;
        }

        graph
    }
}

impl Graph for AdjMatrix {
    fn len(&self) -> NodeIndex {
        self.len
    }

    fn add_node(&mut self, idx: NodeIndex) {
        if idx as usize > self.mat.len() {
            self.mat.resize(((idx + 1) * (idx + 1)) as usize, 0);
            self.len = idx + 1;
        }
    }

    fn connect(&mut self, n1: NodeIndex, n2: NodeIndex, weight: i32) {
        let id1 = n1 * self.len + n2;
        let id2 = n2 * self.len + n1;

        self.mat[id1 as usize] = weight;
        self.mat[id2 as usize] = weight;
    }

    fn connected(&self, n1: NodeIndex, n2: NodeIndex) -> bool {
        let id1 = n1 * self.len + n2;
        self.mat[id1 as usize] != 0
    }

    fn distance(&self, n1: NodeIndex, n2: NodeIndex) -> i32 {
        let id1 = n1 * self.len + n2;
        self.mat[id1 as usize]
    }

    fn memory(&self) -> usize {
        self.mat.len() * mem::size_of::<Weight>()
    }
}

#[derive(Debug)]
pub struct AdjList {
    max_node: NodeIndex,
    adjs: Vec<(NodeIndex, NodeIndex, Weight)>,
}

impl AdjList {
    pub fn new() -> Self {
        AdjList {
            max_node: 0,
            adjs: Vec::new(),
        }
    }
}

impl Graph for AdjList {
    fn len(&self) -> NodeIndex {
        self.max_node + 1
    }

    fn add_node(&mut self, node: NodeIndex) {
        if node > self.max_node {
            self.max_node = node;
        }
    }

    fn connect(&mut self, n1: NodeIndex, n2: NodeIndex, weight: i32) {
        if n1 > self.max_node || n2 > self.max_node {
            panic!("node does not exist")
        }

        if self
            .adjs
            .iter_mut()
            .find(|(u, v, _)| *u == n1 && *v == n2)
            .map(|(_, _, w)| *w = weight)
            .is_none()
        {
            self.adjs.push((n1, n2, weight));
        }
    }

    fn connected(&self, n1: NodeIndex, n2: NodeIndex) -> bool {
        self.adjs
            .iter()
            .find(|(u, v, _)| (*u == n1 && *v == n2) || (*u == n2 && *v == n1))
            .is_some()
    }

    fn distance(&self, n1: NodeIndex, n2: NodeIndex) -> i32 {
        self.adjs
            .iter()
            .find(|(u, v, _)| (*u == n1 && *v == n2) || (*u == n2 && *v == n1))
            .unwrap()
            .2
    }

    fn memory(&self) -> usize {
        self.adjs.len() * mem::size_of::<(NodeIndex, NodeIndex, Weight)>()
    }
}
