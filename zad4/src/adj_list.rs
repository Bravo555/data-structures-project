use std::mem;

use crate::Graph;

type NodeIndex = u32;
type Weight = i32;

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

    fn num_neighbours(&self, n: crate::NodeIndex) -> usize {
        todo!()
    }
}
