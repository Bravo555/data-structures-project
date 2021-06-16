use rand::{
    distributions::Uniform,
    prelude::{Distribution, SliceRandom, SmallRng},
};

use crate::Graph;

type NodeIndex = crate::NodeIndex;
type Weight = crate::Weight;

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

impl Graph for IncidenceMatrix {
    fn len(&self) -> crate::NodeIndex {
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
        let mat = self.mat.chunks_exact_mut(self.nodes as usize).map(|edge| {
            let mut connected_nodes = edge.iter_mut().enumerate().filter(|(_, w)| **w != 0);
            let (m1, w1) = connected_nodes.next().unwrap();
            let (m2, w2) = connected_nodes.next().unwrap();

            if (n1 == m1 && n2 == m2) || (n1 == m2 && n2 == m1) {
                *w1 = weight;
                *w2 = weight;
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

        assert_eq!(self.distance(n1 as NodeIndex, n2 as NodeIndex), weight);
    }

    fn nodes_connected(&self, n1: crate::NodeIndex, n2: crate::NodeIndex) -> bool {
        todo!()
    }

    fn distance(&self, n1: crate::NodeIndex, n2: crate::NodeIndex) -> crate::Weight {
        let n1 = n1 as usize;
        let n2 = n2 as usize;

        for edge in self.mat.chunks_exact(self.nodes as usize) {
            let mut connected = edge.iter().enumerate().filter(|(_, w)| **w != 0);
            let (m1, w) = connected.next().unwrap();
            let (m2, w) = connected.next().unwrap();

            if n1 == m1 && n2 == m2 || n1 == m2 && n2 == m1 {
                return *w;
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

    fn node_neighbours(&self, n: crate::NodeIndex) -> Vec<NodeIndex> {
        let nodes = self.nodes as usize;
        let n = n as usize;
        self.mat
            .chunks_exact(nodes)
            .filter_map(|edge| {
                let mut connected = edge.iter().enumerate().filter(|(_, w)| **w != 0);

                let (n1, _) = connected.next().unwrap();
                let (n2, _) = connected.next().unwrap();

                if n1 == n {
                    Some(n2 as NodeIndex)
                } else if n2 == n {
                    Some(n1 as NodeIndex)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn graph_connected(&self) -> bool {
        todo!()
    }
}