use std::mem;

use rand::{
    distributions::Uniform,
    prelude::{Distribution, SliceRandom, SmallRng},
    Rng,
};

use crate::Graph;

type NodeIndex = crate::NodeIndex;
type Weight = crate::Weight;

#[derive(Debug)]
pub struct AdjList {
    nodes: Vec<Vec<(NodeIndex, Weight)>>,
}

impl AdjList {
    pub fn new() -> Self {
        AdjList { nodes: Vec::new() }
    }

    pub fn random_connected(num_nodes: usize, edge_probability: f32, rng: &mut SmallRng) -> Self {
        let mut graph = Self::new();
        graph.add_node((num_nodes - 1) as NodeIndex);

        // first we connect all unordered pairs of the graph so that it is connected
        let mut unvisited_set = Vec::new();

        for node in 0..graph.len_nodes() {
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

        let len = graph.len_nodes();

        let possible_edges =
            (0..len).flat_map(|n1| (0..len).filter(move |n2| n1 != *n2).map(move |n2| (n1, n2)));

        possible_edges.for_each(|(n1, n2)| {
            if rng.gen_bool(edge_probability as f64) {
                let weight = weight_dist.sample(rng);
                graph.connect(n1, n2, weight);
            }
        });

        graph
    }
}

impl Graph for AdjList {
    fn len_nodes(&self) -> NodeIndex {
        self.nodes.len() as NodeIndex
    }

    fn add_node(&mut self, node: NodeIndex) {
        if node < self.nodes.len() as NodeIndex {
            return;
        }
        self.nodes.resize(node as usize + 1, Vec::new());
    }

    fn connect(&mut self, n1: NodeIndex, n2: NodeIndex, weight: i32) {
        let n1 = n1 as usize;
        let n2 = n2 as usize;
        if n1 > self.nodes.len() || n2 > self.nodes.len() {
            panic!("node does not exist")
        }

        if self.nodes[n1]
            .iter_mut()
            .find(|(u, _)| *u as usize == n2)
            .map(|(_, w)| *w = weight)
            .is_none()
        {
            self.nodes[n1].push((n2 as NodeIndex, weight));
        }

        if self.nodes[n2]
            .iter_mut()
            .find(|(u, _)| *u as usize == n1)
            .map(|(_, w)| *w = weight)
            .is_none()
        {
            self.nodes[n2].push((n1 as NodeIndex, weight));
        }
    }

    fn nodes_connected(&self, n1: NodeIndex, n2: NodeIndex) -> bool {
        assert!((n1 as usize) < self.nodes.len());
        assert!((n2 as usize) < self.nodes.len());
        self.nodes[n1 as usize]
            .iter()
            .find(|(u, _)| *u == n2)
            .is_some()
    }

    fn distance(&self, n1: NodeIndex, n2: NodeIndex) -> i32 {
        assert!((n1 as usize) < self.nodes.len());
        assert!((n2 as usize) < self.nodes.len());
        self.nodes[n1 as usize]
            .iter()
            .find(|(u, _)| *u == n2)
            .unwrap()
            .1
    }

    fn memory(&self) -> usize {
        self.nodes.len() * mem::size_of::<(NodeIndex, NodeIndex, Weight)>()
    }

    fn num_neighbours(&self, _n: crate::NodeIndex) -> usize {
        todo!()
    }

    fn graph_connected(&self) -> bool {
        todo!()
    }

    fn node_neighbours(&self, n: NodeIndex) -> Vec<NodeIndex> {
        self.nodes[n as usize].iter().map(|(n2, _)| *n2).collect()
    }
}
