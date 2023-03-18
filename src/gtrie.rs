use fixedbitset::FixedBitSet;

use crate::{bitgraph::Bitgraph, node::GtrieNode};

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

    pub fn insert(&mut self, graph: &Bitgraph) {
        assert!(graph.n_nodes() <= self.max_depth);
        // println!("---");
        // graph.pprint();
        // println!("---");
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
        let used = Vec::new();
        for c in self.root.iter_children_mut() {
            Self::match_child(c, &used, &graph);
        }
    }

    fn match_child(node: &mut GtrieNode, used: &[usize], graph: &Bitgraph) {
        let vertices = Self::matching_vertices(node, &used, graph);
        for v in vertices {
            let mut used_2 = used.to_vec();
            used_2.push(v);
            if node.is_graph() {
                node.increment_frequency();
            } else {
                for c in node.iter_children_mut() {
                    Self::match_child(c, &used_2, graph);
                }
            }
        }
    }

    fn matching_vertices(node: &GtrieNode, used: &[usize], graph: &Bitgraph) -> Vec<usize> {
        let cand = Self::build_candidates(graph, used);
        let mut vertices = Vec::new();
        for v in cand {
            let mut condition_a = true;
            let mut condition_b = true;
            for (i, u) in used.iter().enumerate() {
                condition_a &= node.out_contains(i) == graph.is_connected(*u, v);
                condition_b &= node.in_contains(i) == graph.is_connected(v, *u);
            }
            if condition_a && condition_b {
                vertices.push(v);
            }
        }
        vertices
    }

    fn build_candidates(graph: &Bitgraph, used: &[usize]) -> Vec<usize> {
        if used.len() == 0 {
            (0..graph.n_nodes()).collect()
        } else {
            let mut cand = Vec::new();
            let mut v_conn = Vec::new();
            let mut v_min = usize::MAX;
            for v in used {
                for n in graph.undir_neighbors(*v).ones() {
                    if used.contains(&n) {
                        continue;
                    }
                    v_conn.push(n);
                    let nn = graph.n_undir_neighbors(n);
                    if nn < v_min {
                        v_min = nn;
                    }
                }
            }
            for i in v_conn {
                cand.push(i);
            }
            cand
        }
    }
}
