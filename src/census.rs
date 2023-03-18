use fixedbitset::FixedBitSet;
use crate::{bitgraph::Bitgraph, node::GtrieNode, symmetry::Conditions};

type Candidates = FixedBitSet;
type Connections = FixedBitSet;

pub fn match_child(
    node: &mut GtrieNode,
    used: &mut Vec<usize>,
    candidates: &mut Candidates,
    connections: &mut Connections,
    graph: &Bitgraph,
) {
    let vertices = matching_vertices(node, &used, graph, candidates, connections);
    for v in vertices {
        used.push(v);
        if node.is_graph() {
            node.increment_frequency();
        } else {
            for c in node.iter_children_mut() {
                match_child(c, used, candidates, connections, graph);
            }
        }
        used.pop();
    }
}


fn matching_vertices(
    node: &GtrieNode,
    used: &[usize],
    graph: &Bitgraph,
    candidates: &mut Candidates,
    connections: &mut Connections,
) -> Vec<usize> {
    build_candidates(graph, used, candidates, connections);
    let mut vertices = Vec::new();
    candidates
        .ones()
        .filter(|v| matches_structure(node, graph, used, *v))
        .for_each(|v| vertices.push(v));
    clear_bits(candidates, connections);
    vertices
}

fn build_candidates(
    graph: &Bitgraph,
    used: &[usize],
    candidates: &mut Candidates,
    connections: &mut Connections,
) {
    if used.len() == 0 {
        candidates.insert_range(..);
    } else {
        let mut v_min = usize::MAX;
        for v in used {
            for n in graph.undir_neighbors(*v).ones() {
                if used.contains(&n) {
                    continue;
                }
                connections.insert(n);
                let nn = graph.n_undir_neighbors(n);
                if nn < v_min {
                    v_min = nn;
                }
            }
        }
        candidates.union_with(connections);
    }
}

fn matches_structure(node: &GtrieNode, graph: &Bitgraph, used: &[usize], v: usize) -> bool {
    used.iter().enumerate().all(|(i, u)| {
        node.out_contains(i) == graph.is_connected(*u, v)
            && node.in_contains(i) == graph.is_connected(v, *u)
    })
}

/*
 * Conditionally match a child node.
 * This is used for the census of the graph space.
 * The condition is that the node must be a graph.
*/

pub fn match_child_conditionally(
    node: &mut GtrieNode,
    used: &mut Vec<usize>,
    candidates: &mut Candidates,
    connections: &mut Connections,
    graph: &Bitgraph,
) {
    let vertices = matching_vertices_conditionally(node, &used, graph, candidates, connections);
    for v in vertices {
        used.push(v);
        if node.is_graph() {
            node.increment_frequency();
        } else {
            for c in node.iter_children_mut() {
                match_child_conditionally(c, used, candidates, connections, graph);
            }
        }
        used.pop();
    }
}

pub fn matching_vertices_conditionally(
    node: &GtrieNode,
    used: &[usize],
    graph: &Bitgraph,
    candidates: &mut Candidates,
    connections: &mut Connections,
) -> Vec<usize> {
    build_candidates_conditionally(graph, used, candidates, connections, node.conditions());
    let mut vertices = Vec::new();
    candidates
        .ones()
        .filter(|v| matches_structure_conditionally(node, graph, used, *v))
        .for_each(|v| vertices.push(v));
    clear_bits(candidates, connections);
    vertices
}

fn build_candidates_conditionally(
    graph: &Bitgraph,
    used: &[usize],
    candidates: &mut Candidates,
    connections: &mut Connections,
    conditions: Option<&Conditions>,
) {
    if used.len() == 0 {
        candidates.insert_range(..);
    } else {
        let mut v_min = usize::MAX;
        for v in used {
            for n in graph.undir_neighbors(*v).ones() {
                if used.contains(&n) {
                    continue;
                }
                connections.insert(n);
                let nn = graph.n_undir_neighbors(n);
                if nn < v_min {
                    v_min = nn;
                }
            }
        }
        candidates.union_with(connections);
    }
}

fn matches_structure_conditionally(node: &GtrieNode, graph: &Bitgraph, used: &[usize], v: usize) -> bool {
    used.iter().enumerate().all(|(i, u)| {
        node.out_contains(i) == graph.is_connected(*u, v)
            && node.in_contains(i) == graph.is_connected(v, *u)
    })
}

fn clear_bits(candidates: &mut Candidates, connections: &mut Connections) {
    candidates.clear();
    connections.clear();
}
