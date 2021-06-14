use rand::{distributions::Uniform, prelude::*, Rng};
use std::{collections::HashSet, mem};

use crate::Graph;

type NodeIndex = u32;
type Weight = i32;

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

    fn num_neighbours(&self, n: NodeIndex) -> usize {
        todo!()
    }
}
