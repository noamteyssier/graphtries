use crate::symmetry::{Condition, Conditions};
use fixedbitset::FixedBitSet;
use itertools::Itertools;

/// A struct that holds the adjacency matrix and orbits of a graph
pub struct CanonicalBasedNauty {
    pub adj: FixedBitSet,
    pub orbits: Vec<usize>,
    conditions: Option<Conditions>,
    size: usize,
}
impl CanonicalBasedNauty {
    pub fn new(adj: FixedBitSet, orbits: Vec<usize>, conditions: Option<Conditions>) -> Self {
        let size = orbits.len();
        CanonicalBasedNauty {
            adj,
            orbits,
            conditions,
            size,
        }
    }

    pub fn adjacency(&self) -> &FixedBitSet {
        &self.adj
    }

    pub fn conditions(&self) -> Option<&Conditions> {
        self.conditions.as_ref()
    }

    #[allow(dead_code)]
    /// Pretty print the adjacency matrix, orbits, and symmetry breaking conditions
    /// for debugging purposes
    pub fn pprint(&self) {
        println!("--------------------------------");
        print!("Adjacency: ");
        for u in 0..self.size {
            for v in 0..self.size {
                if self.adj.contains(u * self.size + v) {
                    print!("1 ");
                } else {
                    print!("0 ");
                }
            }
        }
        println!();

        print!("Orbits:");
        for i in 0..self.orbits.len() {
            print!("{} ", self.orbits[i]);
        }
        println!();

        if let Some(conditions) = &self.conditions {
            println!("Conditions:");
            for c in conditions.iter() {
                println!(":: {}", c);
            }
        }

        println!("--------------------------------");
    }
}

/// Algorithm: Converting a graph to a canonical form
///
/// Require: Graph G
/// Ensure: Canonical form of G
///
/// 1. function GTCanon(G)
/// 2.    G := NautyLabeling(G)
/// 3.    for all i in V(G) do
/// 4.        current_degree[i] := n_incoming[i] + n_outgoing[i]
/// 5.        global_degree[i] := last_degree[i] := current_degree[i]
/// 6.    for pos: |V(G)| down to 1 do
/// 7.        Choose u_min subject to:
/// 8.            u_min still not labeled
/// 9.            u_min is not an articulation point
/// 10.           u_min has the smallest current_degree
/// 11.           if case of equal min degrees, choose u_min with min last_degree
/// 12.           if case of equal min degrees, choose u_min with min global_degree
/// 13.       label_canon[u_min] := pos
/// 14.       last_degree[] := current_degree[]
/// 15.       update current_degree[] removing u_min connections
/// 16.   return label_canon
pub fn canonical_based_nauty(
    adj: &FixedBitSet,
    size: usize,
    orbits: &[i32],
) -> CanonicalBasedNauty {
    let mut new_adj = FixedBitSet::with_capacity(size * size);
    let mut new_orbits = Vec::with_capacity(orbits.len());

    let mut degree = vec![0; size];
    let mut global_degree = vec![0; size];
    let mut used = vec![false; size];
    let mut last_degree = vec![0; size];
    let mut labels = vec![0; size];

    // initializes the degrees
    init_degrees(adj, size, &mut degree, &mut global_degree, &mut last_degree);

    // calculate relabeling of vertices
    calculate_relabels(
        adj,
        size,
        &mut degree,
        &mut global_degree,
        &mut last_degree,
        &mut used,
        &mut labels,
    );

    // write the new adjacency matrix given the labels
    relabel_adj(adj, &mut new_adj, size, &labels);

    // write the new orbits
    relabel_orbits(orbits, &mut new_orbits, &labels);

    // identify symmetry breaking conditions
    let conditions = symmetry_breaking_conditions(&new_orbits);

    // return the new adjacency matrix and orbits
    CanonicalBasedNauty::new(new_adj, new_orbits, conditions)
}

fn calculate_relabels(
    adj: &FixedBitSet,
    size: usize,
    degree: &mut [usize],
    global_degree: &mut [usize],
    last_degree: &mut [usize],
    used: &mut [bool],
    labels: &mut [usize],
) {
    for pos in (0..size).rev() {
        // Find articulation points
        let ap = if pos > 2 {
            find_articulation_points(adj, size, used)
        } else {
            vec![0; size]
        };

        // Select the minimally connected vertex that is not an articulation point
        let min_u = select_minimum_vertex(degree, last_degree, global_degree, used, &ap, size);
        used[min_u] = true;
        labels[pos] = min_u;

        // update current degree removing min_u connections
        update_degree(adj, size, degree, last_degree, min_u);
    }
}

fn select_minimum_vertex(
    degree: &[usize],
    last_degree: &[usize],
    global_degree: &[usize],
    used: &[bool],
    ap: &[usize],
    size: usize,
) -> usize {
    let mut min_u = -1;
    for u in 0..size {
        // Skip if used or is an articulation point
        if used[u] || ap[u] != 0 {
            continue;
        }

        // Iteratively replace min_u to the smallest degree vertex
        if min_u < 0 || degree[u] < degree[min_u as usize] {
            // println!("COND1");
            min_u = u as i32;

        // In the case of ties
        } else if degree[u] == degree[min_u as usize] {
            if smaller_last_degree(last_degree, u, min_u as usize) || smaller_global_degree(global_degree, last_degree, u, min_u as usize) {
                min_u = u as i32;
            }
        }
    }

    assert!(min_u >= 0);
    min_u as usize
}

/// First tie breaker; smaller last degree
fn smaller_last_degree(last_degree: &[usize], u: usize, min_u: usize) -> bool {
    last_degree[u] < last_degree[min_u]
}

/// Second tie breaker; smaller global degree
fn smaller_global_degree(last_degree: &[usize], global_degree: &[usize], u: usize, min_u: usize) -> bool {
    last_degree[u] == last_degree[min_u] && global_degree[u] < global_degree[min_u]
}

/// Deincrements the degree of all vertices adjacent to u
fn update_degree(
    adj: &FixedBitSet,
    size: usize,
    degree: &mut [usize],
    last_degree: &mut [usize],
    u: usize,
) {
    for v in 0..size {
        last_degree[v] = degree[v];
        if adj.contains(u * size + v) {
            degree[v] -= 1;
        }
        if adj.contains(v * size + u) {
            degree[v] -= 1;
        }
    }
}

fn init_degrees(
    adj: &FixedBitSet,
    n: usize,
    degree: &mut [usize],
    global_degree: &mut [usize],
    last_degree: &mut [usize],
) {
    for u in 0..n {
        for v in 0..n {
            if adj.contains(u * n + v) {
                degree[u] += 1;
                global_degree[u] += 1;
                last_degree[u] += 1;
            }
            if adj.contains(v * n + u) {
                degree[u] += 1;
                global_degree[u] += 1;
                last_degree[u] += 1;
            }
        }
    }
}

fn relabel_adj(adj: &FixedBitSet, new_adj: &mut FixedBitSet, size: usize, labels: &[usize]) {
    for u in 0..size {
        for v in 0..size {
            if adj.contains(labels[u] * size + labels[v]) {
                new_adj.insert(u * size + v);
            }
        }
    }
}

fn relabel_orbits(orbits: &[i32], new_orbits: &mut Vec<usize>, labels: &[usize]) {
    labels.iter().for_each(|v| {
        new_orbits.push(orbits[*v] as usize);
    });
}

fn symmetry_breaking_conditions(orbits: &[usize]) -> Option<Conditions> {
    let unique_orbits = orbits.iter().unique().collect::<Vec<_>>();
    if unique_orbits.is_empty() {
        None
    } else {
        let mut conditions = Vec::new();
        for o in unique_orbits {
            let mut last = None;
            orbits
                .iter()
                .enumerate()
                .filter(|(_, v)| *v == o)
                .for_each(|(i, _)| {
                    if let Some(last) = last {
                        conditions.push(Condition::new(last, i));
                    }
                    last = Some(i);
                });
        }
        Some(Conditions::from_vec(conditions))
    }
}

/// Algorithm: Finding articulation points
fn find_articulation_points(adj_matrix: &FixedBitSet, n: usize, used: &[bool]) -> Vec<usize> {
    let mut timer = 0;
    let mut visited = vec![false; n];
    let mut tin = vec![-1; n];
    let mut low = vec![-1; n];
    let mut ap = vec![0; n];
    for i in 0..n {
        if !visited[i] && !used[i] {
            dfs_articulation(
                i,
                -1,
                &mut timer,
                &mut visited,
                &mut tin,
                &mut low,
                adj_matrix,
                n,
                &mut ap,
                used,
            );
        }
    }
    ap
}

/// Algorithm: Finding articulation points
///
/// Similar to CPP implementation here: https://cp-algorithms.com/graph/cutpoints.html#algorithm
fn dfs_articulation(
    v: usize,
    p: i32,
    timer: &mut i32,
    visited: &mut Vec<bool>,
    tin: &mut Vec<i32>,
    low: &mut Vec<i32>,
    adj_matrix: &FixedBitSet,
    n: usize,
    ap: &mut Vec<usize>,
    used: &[bool],
) {
    visited[v] = true;
    tin[v] = *timer;
    low[v] = *timer;
    *timer += 1;

    let mut children = 0;
    for to in 0..n {
        if adj_matrix.contains(v * n + to) || adj_matrix.contains(to * n + v) {
            if to == p as usize {
                continue;
            }
            if visited[to] {
                low[v] = std::cmp::min(low[v], tin[to]);
            } else {
                dfs_articulation(
                    to, v as i32, timer, visited, tin, low, adj_matrix, n, ap, used,
                );
                low[v] = std::cmp::min(low[v], low[to]);
                if low[to] >= tin[v] && p != -1 {
                    ap[v] = 1;
                }
                children += 1;
            }
        }
    }
    if p == -1 && children > 1 {
        ap[v] = 1;
    }
}

#[cfg(test)]
mod testing {
    use fixedbitset::FixedBitSet;

    fn insert_graph(adj: &mut FixedBitSet, n: usize, u: usize, v: usize) {
        adj.insert(u * n + v);
    }

    #[test]
    fn articulation_points_no_used_a() {
        let n = 4;
        let mut adj = fixedbitset::FixedBitSet::with_capacity(n * n);
        let used = vec![false; n];
        insert_graph(&mut adj, n, 0, 2);
        insert_graph(&mut adj, n, 1, 0);
        insert_graph(&mut adj, n, 1, 2);
        insert_graph(&mut adj, n, 2, 3);

        let ap = super::find_articulation_points(&adj, n, &used);
        assert_eq!(ap, vec![0, 0, 1, 0]);
    }

    #[test]
    fn articulation_points_no_used_b() {
        let n = 4;
        let mut adj = fixedbitset::FixedBitSet::with_capacity(n * n);
        let used = vec![false; n];
        insert_graph(&mut adj, n, 0, 1);
        insert_graph(&mut adj, n, 1, 2);
        insert_graph(&mut adj, n, 2, 3);

        let ap = super::find_articulation_points(&adj, n, &used);
        assert_eq!(ap, vec![0, 1, 1, 0]);
    }

    #[test]
    fn symmetry_breaking_conditions_a() {
        let orbits = vec![0, 0, 1, 1, 2, 2, 3, 3];
        let conditions = super::symmetry_breaking_conditions(&orbits).unwrap();
        assert_eq!(conditions.len(), 4);
        assert!(conditions.contains(&super::Condition::new(0, 1)));
        assert!(conditions.contains(&super::Condition::new(2, 3)));
        assert!(conditions.contains(&super::Condition::new(4, 5)));
        assert!(conditions.contains(&super::Condition::new(6, 7)));
    }

    #[test]
    fn symmetry_breaking_conditions_b() {
        let orbits = vec![0, 0, 0];
        let conditions = super::symmetry_breaking_conditions(&orbits).unwrap();
        assert_eq!(conditions.len(), 2);
        assert!(conditions.contains(&super::Condition::new(0, 1)));
        assert!(conditions.contains(&super::Condition::new(1, 2)));
    }

    #[test]
    fn symmetry_breaking_conditions_c() {
        let orbits = vec![0, 0, 0, 0];
        let conditions = super::symmetry_breaking_conditions(&orbits).unwrap();
        assert_eq!(conditions.len(), 3);
        assert!(conditions.contains(&super::Condition::new(0, 1)));
        assert!(conditions.contains(&super::Condition::new(1, 2)));
        assert!(conditions.contains(&super::Condition::new(2, 3)));
    }

    // #[test]
    // fn articulation_points_with_used() {
    //     let n = 4;
    //     let mut adj = fixedbitset::FixedBitSet::with_capacity(n * n);
    //     let used = vec![false, false, true, false];
    //     insert_graph(&mut adj, n, 0, 2);
    //     insert_graph(&mut adj, n, 1, 0);
    //     insert_graph(&mut adj, n, 1, 2);
    //     insert_graph(&mut adj, n, 2, 3);
    //
    //     let ap = super::find_articulation_points(&adj, n, &used);
    //     assert_eq!(ap, vec![0, 0, 0, 0]);
    // }

    // #[test]
    // fn articulation_points_with_used_b() {
    //     let n = 4;
    //     let mut adj = fixedbitset::FixedBitSet::with_capacity(n * n);
    //     let used = vec![false, false, true, false];
    //     insert_graph(&mut adj, n, 0, 1);
    //     insert_graph(&mut adj, n, 1, 2);
    //     insert_graph(&mut adj, n, 2, 3);
    //
    //     let ap = super::find_articulation_points(&adj, n, &used);
    //     assert_eq!(ap, vec![0, 1, 0, 0]);
    // }
}
