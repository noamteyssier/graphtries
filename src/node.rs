use crate::{
    bitgraph::Bitgraph,
    symmetry::{Condition, Conditions},
};
use fixedbitset::FixedBitSet;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct GtrieNode {
    children: Vec<GtrieNode>,
    n_nodes: usize,
    depth: usize,
    is_dir: bool,
    is_graph: bool,
    edge_in: FixedBitSet,
    edge_out: FixedBitSet,
    total_in: usize,
    total_out: usize,
    total_edges: usize,
    frequency: usize,
    connections: Vec<usize>,
    conditions: Option<Conditions>,
}
impl Display for GtrieNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        s.push('[');
        for u in 0..self.n_nodes {
            if self.edge_out.contains(u) {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        s.push_str("][");
        for v in 0..self.n_nodes {
            if self.edge_in.contains(v) {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        s.push(']');
        if let Some(conditions) = &self.conditions {
            s.push_str(" |");
            for (idx, c) in conditions.iter().enumerate() {
                if idx > 0 {
                    s.push(' ');
                }
                s.push_str(&format!("{}", c));
            }
            s.push('|');
        }

        if self.is_graph {
            s.push_str(&format!(" -> {}", self.frequency));
        }
        write!(f, "{}", s)
    }
}
impl GtrieNode {
    pub fn new(depth: usize) -> Self {
        GtrieNode {
            children: Vec::new(),
            n_nodes: depth,
            is_dir: false,
            is_graph: false,
            edge_in: FixedBitSet::with_capacity(depth),
            edge_out: FixedBitSet::with_capacity(depth),
            total_in: 0,
            total_out: 0,
            total_edges: 0,
            frequency: 0,
            conditions: None,
            connections: Vec::with_capacity(depth),
            depth,
        }
    }

    pub fn new_conditional(depth: usize, conditions: &Conditions) -> Self {
        let possible_conditions = conditions
            .iter()
            .filter(|c| c.max() < depth)
            .cloned()
            .collect::<Vec<Condition>>();
        if possible_conditions.is_empty() {
            return GtrieNode::new(depth);
        }
        let possible_conditions = Conditions::from_vec(possible_conditions);
        GtrieNode {
            children: Vec::new(),
            n_nodes: depth,
            is_dir: false,
            is_graph: false,
            edge_in: FixedBitSet::with_capacity(depth),
            edge_out: FixedBitSet::with_capacity(depth),
            total_in: 0,
            total_out: 0,
            total_edges: 0,
            frequency: 0,
            conditions: Some(possible_conditions),
            connections: Vec::with_capacity(depth),
            depth,
        }
    }

    #[allow(dead_code)]
    pub fn from_bitgraph(graph: &Bitgraph) -> Self {
        let n_nodes = graph.n_nodes();
        let is_dir = graph.is_dir();
        let mut edge_in = FixedBitSet::with_capacity(n_nodes);
        let mut edge_out = FixedBitSet::with_capacity(n_nodes);
        let mut total_in = 0;
        let mut total_out = 0;
        let mut total_edges = 0;
        for u in 0..n_nodes {
            for v in 0..n_nodes {
                if graph.is_connected(u, v) {
                    edge_in.insert(v * n_nodes + u);
                    edge_out.insert(u * n_nodes + v);
                    total_in += 1;
                    total_out += 1;
                    total_edges += 1;
                }
            }
        }
        GtrieNode {
            children: Vec::new(),
            n_nodes,
            is_dir,
            is_graph: true,
            edge_in,
            edge_out,
            total_in,
            total_out,
            total_edges,
            frequency: 0,
            conditions: None,
            connections: Vec::with_capacity(n_nodes),
            depth: graph.n_nodes(),
        }
    }

    pub fn in_contains(&self, v: usize) -> bool {
        self.edge_in.contains(v)
    }

    pub fn out_contains(&self, v: usize) -> bool {
        self.edge_out.contains(v)
    }

    pub fn update_adjacency(&mut self, graph: &Bitgraph, k: usize) {
        for u in 0..k {
            let fwd_conn = graph.is_connected(u, k - 1);
            let bwd_conn = graph.is_connected(k - 1, u);

            if fwd_conn || bwd_conn {
                self.connections.push(u);
            }

            if fwd_conn {
                self.edge_out.insert(u);
                self.total_out += 1;
                self.total_edges += 1;
            }

            if bwd_conn {
                self.edge_in.insert(u);
                self.total_in += 1;
                self.total_edges += 1;
            }
        }
    }

    pub fn insert_child(&mut self, child: Self) {
        self.children.push(child);
    }

    pub fn set_graph(&mut self, is_graph: bool) {
        self.is_graph = is_graph;
    }

    pub fn iter_children_mut(&mut self) -> impl Iterator<Item = &mut Self> {
        self.children.iter_mut()
    }

    #[allow(dead_code)]
    pub fn iter_children(&self) -> impl Iterator<Item = &Self> {
        self.children.iter()
    }

    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn is_graph(&self) -> bool {
        self.is_graph
    }

    pub fn increment_frequency(&mut self) {
        self.frequency += 1;
    }

    pub fn intersect_conditions(&mut self, conditions: &Conditions) {
        if self.conditions.is_none() {
            return;
        }
        self.conditions.iter_mut().for_each(|c| {
            c.retain(|x| conditions.contains(x));
        });
        if self.conditions.as_ref().unwrap().is_empty() {
            self.conditions = None;
        }
    }

    pub fn conditions(&self) -> Option<&Conditions> {
        self.conditions.as_ref()
    }

    pub fn active_nodes(&self) -> impl Iterator<Item = &usize> {
        self.connections.iter()
    }

    #[allow(dead_code)]
    pub fn pprint(&self, frequency: bool) {
        print!("{}:", self.depth);
        for _ in 0..self.depth {
            print!("  ");
        }
        println!("{}", self.custom_display(frequency));
        for child in self.iter_children() {
            child.pprint(frequency);
        }
    }

    fn custom_display(&self, frequency: bool) -> String {
        let mut s = String::new();
        s.push('[');
        for u in 0..self.n_nodes {
            if self.edge_out.contains(u) {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        s.push_str("][");
        for v in 0..self.n_nodes {
            if self.edge_in.contains(v) {
                s.push('1');
            } else {
                s.push('0');
            }
        }
        s.push(']');
        if let Some(conditions) = &self.conditions {
            s.push_str(" |");
            for (idx, c) in conditions.iter().enumerate() {
                if idx > 0 {
                    s.push(' ');
                }
                s.push_str(&format!("{}", c));
            }
            s.push('|');
        }

        if frequency && self.is_graph {
            s.push_str(&format!(" -> {}", self.frequency));
        }
        s
    }
}
