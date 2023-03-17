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

    pub fn census(&self, graph: &Bitgraph) {
        let used = Vec::new();
        for c in self.root.iter_children() {
            self.match_child(c, &used, &graph);
        }
    }

    fn match_node(&self, node: &GtrieNode) -> bool {
        for i in 0..3 {
            if i == 1 {
                if node.in_contains(i) && !node.out_contains(i) {
                    return true
                }
            }
            else {
                if node.in_contains(i) || node.out_contains(i) {
                    return false
                }
            }
        }
        false
    }

    fn match_child(&self, node: &GtrieNode, used: &[usize], graph: &Bitgraph) {
        println!("---------------------------------------");
        println!("Entering match_child at depth: {}", node.depth());
        println!("                    with used: {:?}", used);
        println!("                    with node: {}", node);
        println!("                node is graph: {}", node.is_graph());
        let vertices = self.matching_vertices(node, &used, graph);
        // if node.depth() == 3 && self.match_node(node) {
        //     unimplemented!();
        // }
        for v in vertices {
            let mut used_2 = used.to_vec();
            if node.is_graph() {
                used_2.push(v);
                println!("Found a graph!: {}", node);
                println!("{:?}", used_2);
            }
            used_2.push(v);
            // println!("Picking vertex: {v}");
            for c in node.iter_children() {
                self.match_child(c, &used_2, graph);
            }
            // break;
        }
    }

    fn matching_vertices(&self, node: &GtrieNode, used: &[usize], graph: &Bitgraph) -> Vec<usize> {
        println!("Used: {:?}", used);
        let cand = self.build_candidates(graph, used);
        println!("Cand: {:?}", cand);
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
        println!("Vertices: {:?}", vertices);
        vertices
    }

    fn build_candidates(&self, graph: &Bitgraph, used: &[usize]) -> Vec<usize> {
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
            // println!("Conn: {:?}", v_conn);
            for i in v_conn {
                cand.push(i);
                // if graph.n_undir_neighbors(i) <= v_min && !used.contains(&i) {
                //     cand.push(i);
                // }
            }
            cand
        }
    }
}
