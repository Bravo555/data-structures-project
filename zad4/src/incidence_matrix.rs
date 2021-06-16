use rand::{
    distributions::Uniform,
    prelude::{Distribution, SliceRandom, SmallRng},
    Rng,
};

use crate::{Graph, NodeIndex, Weight};

pub struct IncidenceMatrix {
    nodes: NodeIndex,
    edges: usize,
    mat: Vec<Weight>,
}

impl IncidenceMatrix {
    pub fn new() -> Self {
        IncidenceMatrix {
            mat: Vec::new(),
            nodes: 0,
            edges: 0,
        }
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

impl Graph for IncidenceMatrix {
    fn len_nodes(&self) -> crate::NodeIndex {
        self.nodes
    }

    fn add_node(&mut self, node: crate::NodeIndex) {
        if node < self.nodes {
            return;
        }

        self.mat.resize(self.nodes as usize * self.edges, 0);
        self.nodes = node + 1;
    }

    fn connect(&mut self, n1: crate::NodeIndex, n2: crate::NodeIndex, weight: i32) {
        assert!(n1 < self.nodes);
        assert!(n2 < self.nodes);
        assert!(weight != 0);

        let n1 = n1 as usize;
        let n2 = n2 as usize;

        let mut found = false;
        self.mat
            .chunks_exact_mut(self.nodes as usize)
            .for_each(|edge| {
                // let mut connected_nodes = edge.iter_mut().enumerate().filter(|(_, w)| **w != 0);
                // let (m1, w1) = connected_nodes.next().unwrap();
                // let (m2, w2) = connected_nodes.next().unwrap();

                let w1 = edge[n1];
                let w2 = edge[n2];

                if w1 == w2 && w1 != 0 {
                    edge[n1] = weight;
                    edge[n2] = weight;
                    found = true;
                }
            });

        if !found {
            let mut new_edge = vec![0 as Weight; self.nodes as usize];
            new_edge[n1] = weight;
            new_edge[n2] = weight;
            self.mat.extend_from_slice(&new_edge);
            self.edges += 1;
        }
    }

    fn nodes_connected(&self, _: crate::NodeIndex, _: crate::NodeIndex) -> bool {
        todo!()
    }

    fn distance(&self, n1: crate::NodeIndex, n2: crate::NodeIndex) -> crate::Weight {
        let n1 = n1 as usize;
        let n2 = n2 as usize;

        for edge in self.mat.chunks_exact(self.nodes as usize) {
            // let mut connected = edge.iter().enumerate().filter(|(_, w)| **w != 0);
            // let (m1, w) = connected.next().unwrap();
            // let (m2, w) = connected.next().unwrap();

            let m1 = edge[n1];
            let m2 = edge[n2];

            if m1 == m2 && m1 != 0 {
                return m1 as Weight;
            }
        }
        return 0;
    }

    fn memory(&self) -> usize {
        todo!()
    }

    fn num_neighbours(&self, n: crate::NodeIndex) -> usize {
        todo!()
    }

    fn node_neighbours(&self, n: crate::NodeIndex) -> Vec<(NodeIndex, Weight)> {
        let nodes = self.nodes as usize;
        let n = n as usize;
        self.mat
            .chunks_exact(nodes)
            .filter_map(|edge| {
                let w = edge.get(n)?;
                let n2 = edge
                    .iter()
                    .enumerate()
                    .position(|(i, w)| *w != 0 && i != n)
                    .unwrap();

                Some((n2 as NodeIndex, *w))
            })
            .collect::<Vec<_>>()
    }

    fn graph_connected(&self) -> bool {
        todo!()
    }
}
