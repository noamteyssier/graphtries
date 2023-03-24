use anyhow::Result;
use fixedbitset::FixedBitSet;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::io::Write;

use crate::{
    bitgraph::Bitgraph,
    census::{match_child_conditionally, Candidates},
    node::GtrieNode,
    symmetry::Conditions,
};

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Gtrie {
    root: GtrieNode,
    max_depth: usize,
    total_subgraphs: usize,
}
impl Gtrie {
    pub fn new(max_depth: usize) -> Self {
        Gtrie {
            root: GtrieNode::new(0),
            max_depth,
            total_subgraphs: 0,
        }
    }

    pub fn read_from_file(path: &str) -> Result<Self> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let gtrie = serde_json::from_reader(reader)?;
        Ok(gtrie)
    }

    pub fn insert(
        &mut self,
        graph: &Bitgraph,
        conditions: Option<&Conditions>,
        repr: Option<String>,
    ) {
        assert!(graph.n_nodes() <= self.max_depth);
        Self::insert_recursively_conditional(graph, &mut self.root, 0, conditions, repr);
    }

    fn insert_recursively_conditional(
        graph: &Bitgraph,
        node: &mut GtrieNode,
        k: usize,
        conditions: Option<&Conditions>,
        repr: Option<String>,
    ) {
        if k == graph.n_nodes() {
            node.set_graph(true);
            node.set_repr(repr);
        } else {
            for c in node.iter_children_mut() {
                if Self::depth_eq(c, graph, k) {
                    Self::insert_recursively_conditional(graph, c, k + 1, conditions, repr);
                    return;
                }
            }
            node.intersect_conditions(conditions);
            let mut child = if let Some(conditions) = conditions {
                GtrieNode::new_conditional(k + 1, conditions)
            } else {
                GtrieNode::new(k + 1)
            };
            child.update_adjacency(graph, k + 1);
            Self::insert_recursively_conditional(graph, &mut child, k + 1, conditions, repr);
            node.insert_child(child);
        }
    }

    /// Checks if a subgraph is in the trie.
    fn depth_eq(node: &GtrieNode, graph: &Bitgraph, k: usize) -> bool {
        for idx in 0..=k {
            let condition_a = node.out_contains(idx) != graph.is_connected(idx, k);
            let condition_b = node.in_contains(idx) != graph.is_connected(k, idx);
            if condition_a || condition_b {
                return false;
            }
        }
        true
    }

    /// A depth first search that prints out all nodes in the trie.
    #[allow(dead_code)]
    pub fn pprint(&self, frequency: bool) {
        self.root.pprint(frequency);
    }

    pub fn pprint_results(&self) {
        self.root.pprint_results();
    }

    pub fn write_to_file(&self, path: &str) -> Result<()> {
        let mut file = std::fs::File::create(path)?;
        self.write_to_buffer(&mut file)?;
        Ok(())
    }

    pub fn write_to_stdout(&self) -> Result<()> {
        self.write_to_buffer(&mut std::io::stdout())
    }

    pub fn write_to_buffer<W: Write>(&self, writer: &mut W) -> Result<()> {
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    pub fn census(&mut self, graph: &Bitgraph) {
        let mut used = Vec::with_capacity(self.max_depth);
        let mut candidates = Candidates::new(graph.n_nodes());
        let mut blacklist = FixedBitSet::with_capacity(graph.n_nodes());

        for c in self.root.iter_children_mut() {
            match_child_conditionally(
                c,
                &mut used,
                &mut candidates,
                &mut blacklist,
                graph,
                &mut self.total_subgraphs,
            )
        }
    }

    pub fn total_subgraphs(&self) -> usize {
        self.total_subgraphs
    }

    #[allow(dead_code)]
    pub fn get_nonzero(&self) -> HashMap<String, usize> {
        let mut map = HashMap::new();
        self.root.get_nonzero(&mut map);
        map
    }
}
