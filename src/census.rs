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
        *u != v
            && node.out_contains(i) == graph.is_connected(*u, v)
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
    // println!("--------------------------");
    // println!("Entering match: {}", node);
    // println!("Used    : {:?}", used);
    let vertices = matching_vertices_conditionally(node, &used, graph, candidates, connections);
    // println!("Vertices: {:?}", vertices);
    for v in vertices {
        used.push(v);
        if node.is_graph() {
            // println!("Found graph!");
            node.increment_frequency();
        } else {
            for c in node.iter_children_mut() {
                match_child_conditionally(c, used, candidates, connections, graph);
            }
        }
        used.pop();
    }
    // println!("Leaving match: {}", node);
}

pub fn matching_vertices_conditionally(
    node: &GtrieNode,
    used: &[usize],
    graph: &Bitgraph,
    candidates: &mut Candidates,
    connections: &mut Connections,
) -> Vec<usize> {
    build_candidates_conditionally(graph, used, candidates, connections, node.conditions());
    // println!("Candidates: {:?}", candidates.ones().collect::<Vec<usize>>());
    let mut vertices = Vec::new();
    candidates
        .ones()
        .filter(|v| matches_structure(node, graph, used, *v))
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
    if !used_respects_conditions(used, conditions) {
        return;
    }
    let label_min = minimal_possible_index(used, conditions);
    if used.len() == 0 {
        candidates.insert_range(label_min..);
    } else {
        used.iter().for_each(|v| {
            graph.undir_neighbors(*v).ones()
                .filter(|n| *n >= label_min)
                .for_each(|n| {
                    connections.insert(n);
                })
        });
        candidates.union_with(connections);
    }
}

fn used_respects_conditions(used: &[usize], conditions: Option<&Conditions>) -> bool {
    if let Some(conditions) = conditions {
        for i in 0..used.len() {
            for j in i + 1..used.len() {
                if !conditions.respects_any(i, j, used[i], used[j]) {
                    return false
                }
            }
        }
    }
    true
}

fn minimal_possible_index(used: &[usize], conditions: Option<&Conditions>) -> usize {
    if let Some(conditions) = conditions {
        let k = used.len();
        conditions
            .iter()
            .filter(|c| c.max() == k)
            .fold(0, |acc, c| {
                acc.max(used[c.min()] + 1)
            })
    } else {
        0
    }
}

fn clear_bits(candidates: &mut Candidates, connections: &mut Connections) {
    candidates.clear();
    connections.clear();
}

#[cfg(test)]
mod testing {

    use super::*;
    use crate::symmetry::Condition;


    #[test]
    fn conditions_used_positive_a() {
        let used = vec![10, 20, 30];
        let c1 = Condition::new(0, 1);
        let conditions = Conditions::from_vec(vec![c1]);
        assert!(used_respects_conditions(&used, Some(&conditions)));
    }

    #[test]
    fn conditions_used_positive_b() {
        let used = vec![10, 20, 30];
        let c1 = Condition::new(1, 2);
        let conditions = Conditions::from_vec(vec![c1]);
        assert!(used_respects_conditions(&used, Some(&conditions)));
    }

    #[test]
    fn conditions_used_positive_c() {
        let used = vec![10, 20, 30];
        let c1 = Condition::new(0, 2);
        let conditions = Conditions::from_vec(vec![c1]);
        assert!(used_respects_conditions(&used, Some(&conditions)));
    }

    #[test]
    fn conditions_used_positive_d() {
        let used = vec![10, 20, 30];
        let c1 = Condition::new(0, 1);
        let c2 = Condition::new(1, 2);
        let conditions = Conditions::from_vec(vec![c1, c2]);
        assert!(used_respects_conditions(&used, Some(&conditions)));
    }

    #[test]
    fn conditions_used_negative_a() {
        let used = vec![20, 10, 30];
        let c1 = Condition::new(0, 1);
        let conditions = Conditions::from_vec(vec![c1]);
        assert!(!used_respects_conditions(&used, Some(&conditions)));
    }

    #[test]
    fn conditions_used_negative_b() {
        let used = vec![10, 20, 15];
        let c1 = Condition::new(1, 2);
        let conditions = Conditions::from_vec(vec![c1]);
        assert!(!used_respects_conditions(&used, Some(&conditions)));
    }

    #[test]
    fn conditions_used_negative_c() {
        let used = vec![10, 20, 5];
        let c1 = Condition::new(0, 2);
        let conditions = Conditions::from_vec(vec![c1]);
        assert!(!used_respects_conditions(&used, Some(&conditions)));
    }

    #[test]
    fn minimal_index_a() {
        let used = vec![10];
        let c1 = Condition::new(0, 1);
        let conditions = Conditions::from_vec(vec![c1]);
        assert_eq!(minimal_possible_index(&used, Some(&conditions)), 11);
    }

    #[test]
    fn minimal_index_b() {
        let used = vec![10, 20];
        let c1 = Condition::new(0, 1);
        let conditions = Conditions::from_vec(vec![c1]);
        assert_eq!(minimal_possible_index(&used, Some(&conditions)), 0);
    }

    #[test]
    fn minimal_index_c() {
        let used = vec![10, 20];
        let c1 = Condition::new(0, 1);
        let c2 = Condition::new(0, 2);
        let conditions = Conditions::from_vec(vec![c1, c2]);
        assert_eq!(minimal_possible_index(&used, Some(&conditions)), 11);
    }
}
