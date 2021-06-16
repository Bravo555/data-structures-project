use std::mem;

use rand::{
    distributions::Uniform,
    prelude::{Distribution, SliceRandom, SmallRng},
};

use crate::{Graph, NodeIndex, Weight};

#[derive(Debug)]
pub struct Bundle {
    node_indexes: Vec<usize>,
    adjs: Vec<(NodeIndex, NodeIndex, Weight)>,
    max_node: NodeIndex,
}

impl Bundle {
    pub fn new() -> Self {
        Bundle {
            max_node: 0,
            adjs: Vec::new(),
            node_indexes: Vec::new(),
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

impl Graph for Bundle {
    fn len(&self) -> NodeIndex {
        self.max_node + 1
    }

    fn add_node(&mut self, node: NodeIndex) {
        if node > self.max_node {
            self.max_node = node;
            self.node_indexes
                .resize((node + 1) as usize, self.adjs.len());
        }
    }

    fn connect(&mut self, n1: NodeIndex, n2: NodeIndex, weight: i32) {
        if n1 > self.max_node || n2 > self.max_node {
            panic!("node does not exist")
        }

        if self
            .adjs
            .iter_mut()
            .skip(self.node_indexes[n1 as usize])
            .find(|(u, v, _)| *u == n1 && *v == n2)
            .map(|(_, _, w)| *w = weight)
            .is_none()
        {
            self.adjs
                .insert(self.node_indexes[n1 as usize], (n1, n2, weight));
            self.node_indexes
                .iter_mut()
                .skip(n1 as usize + 1)
                .for_each(|i| *i += 1);
        }

        if self
            .adjs
            .iter_mut()
            .skip(self.node_indexes[n2 as usize])
            .find(|(v, u, _)| *u == n1 && *v == n2)
            .map(|(_, _, w)| *w = weight)
            .is_none()
        {
            self.adjs
                .insert(self.node_indexes[n2 as usize], (n2, n1, weight));
            self.node_indexes
                .iter_mut()
                .skip(n2 as usize + 1)
                .for_each(|i| *i += 1);
        }
    }

    fn nodes_connected(&self, n1: NodeIndex, n2: NodeIndex) -> bool {
        self.adjs
            .iter()
            .take(
                *self
                    .node_indexes
                    .get(n1 as usize + 1)
                    .unwrap_or(&self.adjs.len()),
            )
            .skip(self.node_indexes[n1 as usize])
            .find(|(u, v, _)| (*u == n1 && *v == n2) || (*u == n2 && *v == n1))
            .is_some()
    }

    fn distance(&self, n1: NodeIndex, n2: NodeIndex) -> i32 {
        self.adjs
            .iter()
            .take(
                *self
                    .node_indexes
                    .get(n1 as usize + 1)
                    .unwrap_or(&self.adjs.len()),
            )
            .skip(self.node_indexes[n1 as usize])
            .find(|(u, v, _)| (*u == n1 && *v == n2) || (*u == n2 && *v == n1))
            .unwrap()
            .2
    }

    fn memory(&self) -> usize {
        self.adjs.len() * mem::size_of::<(NodeIndex, NodeIndex, Weight)>()
    }

    fn num_neighbours(&self, _n: crate::NodeIndex) -> usize {
        todo!()
    }

    fn graph_connected(&self) -> bool {
        todo!()
    }

    fn node_neighbours(&self, n: NodeIndex) -> Vec<NodeIndex> {
        self.adjs
            .iter()
            .take(
                *self
                    .node_indexes
                    .get(n as usize + 1)
                    .unwrap_or(&self.adjs.len()),
            )
            .skip(self.node_indexes[n as usize])
            .filter(|(n1, _, _)| n == *n1)
            .map(|(_, n2, _)| *n2)
            .collect()
    }
}
