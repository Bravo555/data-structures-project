use rand::{distributions::Uniform, prelude::*};
use std::{fmt::Debug, mem};

use crate::Graph;

type NodeIndex = u32;
type Weight = i32;

pub struct AdjMatrix {
    mat: Vec<Weight>,
    len: NodeIndex,
}

impl Debug for AdjMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max = self.mat.iter().max().unwrap_or(&1).to_string();
        let mut table = String::new();
        table.push_str("\t");
        table.push_str(
            &(0..self.len)
                .map(|i| format!("{:1$}", i, max.len()))
                .collect::<Vec<_>>()
                .join(" "),
        );
        table.push_str("\n\n");
        let mat = self
            .mat
            .iter()
            .map(|i| format!("{:1$}", i, max.len()))
            .collect::<Vec<_>>();
        let chunks = mat.chunks(self.len as usize).collect::<Vec<_>>();
        for (i, chunk) in chunks.into_iter().enumerate() {
            table.push_str(&i.to_string());
            table.push_str("\t");
            table.push_str(&chunk.join(" "));
            table.push_str("\n");
        }

        write!(f, "AdjMatrix {{\nsize: {},\nmatrix:\n{}", self.len, table)
    }
}

impl AdjMatrix {
    pub fn new() -> Self {
        AdjMatrix {
            mat: Vec::new(),
            len: 0,
        }
    }

    pub fn random_connected(num_nodes: usize, edge_probability: f32, rng: &mut SmallRng) -> Self {
        let mut graph = Self::new();
        graph.add_node((num_nodes - 1) as NodeIndex);

        // first we connect all unordered pairs of the graph so that it is connected
        let mut unvisited_set = Vec::new();

        for node in 0..graph.len() {
            unvisited_set.push(node);
        }
        unvisited_set.shuffle(rng);

        let mut cur_vertex = unvisited_set.pop().expect("no nodes in the graph");
        let weight_dist = Uniform::from(1..=20);

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

    fn nodes_connected(&self, n1: NodeIndex, n2: NodeIndex) -> bool {
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

    fn num_neighbours(&self, n: NodeIndex) -> usize {
        let start = (n * self.len) as usize;
        let end = (start + self.len as usize) as usize;
        self.mat[start..end]
            .iter()
            .filter(|neighbour| **neighbour != 0)
            .count() as usize
    }

    fn graph_connected(&self) -> bool {
        let mut connected = true;
        for node in 0..self.len {
            if self.num_neighbours(node) == 0 {
                connected = false;
                break;
            }
        }
        connected
    }

    fn node_neighbours(&self, n: crate::NodeIndex) -> Vec<NodeIndex> {
        let start_index = (n * self.len) as usize;
        let end_index = (start_index + self.len as usize) as usize;
        self.mat[start_index..end_index]
            .iter()
            .enumerate()
            .filter(|(i, w)| **w != 0)
            .map(|(i, w)| i as NodeIndex)
            .collect()
    }
}
