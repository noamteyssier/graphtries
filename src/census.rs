use fixedbitset::FixedBitSet;

use crate::{bitgraph::Bitgraph, node::GtrieNode};

type Candidates = FixedBitSet;

pub fn match_child(node: &mut GtrieNode, used: &mut Vec<usize>, candidates: &mut Candidates, graph: &Bitgraph) {
    let vertices = matching_vertices(node, &used, graph, candidates);
    for v in vertices {
        used.push(v);
        if node.is_graph() {
            node.increment_frequency();
        } else {
            for c in node.iter_children_mut() {
                match_child(c, used, candidates, graph);
            }
        }
        used.pop();
    }
}

fn matching_vertices(node: &GtrieNode, used: &[usize], graph: &Bitgraph, candidates: &mut Candidates) -> Vec<usize> {
    build_candidates(graph, used, candidates);
    let mut vertices = Vec::new();
    for v in candidates.ones() {
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
    clear_candidates(candidates);
    vertices
}

fn build_candidates(graph: &Bitgraph, used: &[usize], candidates: &mut Candidates) {
    if used.len() == 0 {
        candidates.insert_range(..);
    } else {
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
            candidates.insert(i);
        }
    }
}

fn clear_candidates(candidates: &mut Candidates) {
    candidates.clear();
}
