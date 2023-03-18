use crate::{bitgraph::Bitgraph, node::GtrieNode};

pub fn match_child(node: &mut GtrieNode, used: &[usize], candidates: &mut [usize], graph: &Bitgraph) {
    let vertices = matching_vertices(node, &used, graph, candidates);
    for v in vertices {
        let used_2 = insert_to_used(&used, v);
        if node.is_graph() {
            node.increment_frequency();
        } else {
            for c in node.iter_children_mut() {
                match_child(c, &used_2, candidates, graph);
            }
        }
    }
}

fn matching_vertices(node: &GtrieNode, used: &[usize], graph: &Bitgraph, candidates: &mut [usize]) -> Vec<usize> {
    let n_cand = build_candidates(graph, used, candidates);
    let mut vertices = Vec::new();
    for idx in 0..n_cand {
        let v = candidates[idx];
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

fn build_candidates(graph: &Bitgraph, used: &[usize], candidates: &mut [usize]) -> usize {
    if used.len() == 0 {
        for i in 0..graph.n_nodes() {
            candidates[i] = i;
        }
        return graph.n_nodes();
    } else {
        let mut v_conn = Vec::new();
        let mut v_min = usize::MAX;
        let mut n_cand = 0;
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
            candidates[n_cand] = i;
            n_cand += 1;
        }
        n_cand
    }
}

fn insert_to_used(used: &[usize], v: usize) -> Vec<usize> {
    let mut used_2 = used.to_vec();
    used_2.push(v);
    used_2
}

fn clear_candidates(candidates: &mut [usize]) {
    candidates.iter_mut().for_each(|x| *x = 0);
}
