use fixedbitset::FixedBitSet;

use crate::{bitgraph::Bitgraph, census::match_child, isomorphism::Conditions, node::GtrieNode};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Gtrie {
    root: GtrieNode,
    max_depth: usize,
}
impl Gtrie {
    pub fn new(max_depth: usize) -> Self {
        Gtrie {
            root: GtrieNode::new(0),
            max_depth,
        }
    }

    pub fn insert(&mut self, graph: &Bitgraph, conditions: Option<&Conditions>) {
        assert!(graph.n_nodes() <= self.max_depth);
        if let Some(conditions) = conditions {
            Self::insert_recursively_conditional(graph, &mut self.root, 0, conditions);
        } else {
            Self::insert_recursively(graph, &mut self.root, 0);
        }
        Self::insert_recursively(graph, &mut self.root, 0);
    }

    fn insert_recursively(graph: &Bitgraph, node: &mut GtrieNode, k: usize) {
        if k == graph.n_nodes() {
            node.set_graph(true);
            return;
        } else {
            for c in node.iter_children_mut() {
                if Self::depth_eq(c, graph, k) {
                    Self::insert_recursively(graph, c, k + 1);
                    return;
                }
            }
            let mut child = GtrieNode::new(k + 1);
            child.update_adjacency(graph, k + 1);
            Self::insert_recursively(graph, &mut child, k + 1);
            node.insert_child(child);
        }
    }

    fn insert_recursively_conditional(
        graph: &Bitgraph,
        node: &mut GtrieNode,
        k: usize,
        conditions: &Conditions,
    ) {
        if k == graph.n_nodes() {
            node.set_graph(true);
            return;
        } else {
            for c in node.iter_children_mut() {
                if Self::depth_eq(c, graph, k) {
                    Self::insert_recursively_conditional(graph, c, k + 1, conditions);
                    return;
                }
            }
            let mut child = if conditions.is_empty() {
                GtrieNode::new(k + 1)
            } else {
                node.intersect_conditions(conditions);
                GtrieNode::new_conditional(k + 1, conditions)
            };
            child.update_adjacency(graph, k + 1);
            Self::insert_recursively_conditional(graph, &mut child, k + 1, conditions);
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
    pub fn pprint(&self) {
        self.root.pprint();
    }

    pub fn census(&mut self, graph: &Bitgraph) {
        let mut used = Vec::with_capacity(self.max_depth);
        let mut candidates = FixedBitSet::with_capacity(graph.n_nodes());
        let mut connections = FixedBitSet::with_capacity(graph.n_nodes());
        for c in self.root.iter_children_mut() {
            match_child(c, &mut used, &mut candidates, &mut connections, &graph);
        }
    }
}
