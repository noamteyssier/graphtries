use crate::symmetry::{Condition, Conditions};
use fixedbitset::FixedBitSet;
use graph_canon::autom::AutoGroups;
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
    aut: &AutoGroups,
) -> CanonicalBasedNauty {
    let mut new_adj = FixedBitSet::with_capacity(size * size);
    let mut new_orbits = Vec::with_capacity(aut.n_nodes());

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
    relabel_orbits(aut.orbits(), &mut new_orbits, &labels);

    // identify symmetry breaking conditions
    // let conditions = symmetry_breaking_conditions(&new_orbits);
    let conditions = symmetry_breaking_conditions(aut, &new_orbits);

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
            vec![false; size]
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
    ap: &[bool],
    size: usize,
) -> usize {
    let mut min_u = -1;
    for u in 0..size {
        // Skip if used or is an articulation point
        if used[u] || ap[u] {
            continue;
        }

        // Iteratively replace min_u to the smallest degree vertex
        if min_u < 0 || degree[u] < degree[min_u as usize] {
            // println!("COND1");
            min_u = u as i32;

        // In the case of ties
        } else if degree[u] == degree[min_u as usize] {
            if smaller_last_degree(last_degree, u, min_u as usize)
                || smaller_global_degree(global_degree, last_degree, u, min_u as usize)
            {
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
fn smaller_global_degree(
    last_degree: &[usize],
    global_degree: &[usize],
    u: usize,
    min_u: usize,
) -> bool {
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

fn symmetry_breaking_conditions(aut: &AutoGroups, orbits: &[usize]) -> Option<Conditions> {
    if aut.size() == 0 {
        None
    } else {
        let mut conditions = Vec::new();
        let unique_orbits = orbits.iter().unique().collect::<Vec<_>>();
        let mut group = aut.automorphisms().clone();

        for o in unique_orbits {
            if group.len() == 1 {
                break;
            }
            for (idx, _u) in orbits.iter().enumerate().filter(|(_, v)| *v == o) {
                if group.len() == 1 {
                    break;
                }
                for (jdx, _v) in orbits.iter().enumerate().filter(|(_, v)| *v == o) {
                    if group.len() == 1 {
                        break;
                    }
                    if idx < jdx {
                        conditions.push(Condition::new(idx, jdx));
                        group.retain(|g| g[idx] < g[jdx]);
                    }
                }
            }
        }

        Some(Conditions::from_vec(conditions))
    }
}

/// Algorithm: Finding articulation points
fn find_articulation_points(
    adj_matrix: &FixedBitSet, 
    n: usize, 
    blacklist: &[bool]
) -> Vec<bool> {
    let mut timer = 0;
    let mut visited = vec![false; n];
    let mut timing = vec![-1; n];
    let mut low = vec![-1; n];
    let mut ap = vec![false; n];
    dfs_articulation(
        adj_matrix,
        n,
        0,
        None,
        &mut timer,
        &mut visited,
        &mut timing,
        &mut low,
        &mut ap,
        blacklist,
    );
    ap
}

/// Algorithm: Finding articulation points
///
/// Adapted from C++ implementation here: 
/// https://cp-algorithms.com/graph/cutpoints.html#algorithm
fn dfs_articulation(
    adj: &FixedBitSet,
    n: usize,
    u: usize,
    parent: Option<usize>,
    timer: &mut usize,
    visited: &mut [bool],
    timing: &mut [i32],
    low: &mut [i32],
    ap: &mut [bool],
    blacklist: &[bool],
) {
    *timer += 1;
    visited[u] = true;
    timing[u] = *timer as i32;
    low[u] = *timer as i32;

    let mut children = 0;

    // iterate over all nodes
    for v in 0..n {

        // skip if not a neighbor of the current head
        if !adj.contains(u*n+v) && !adj.contains(v*n+u) {
            continue
        }

        // skip if blacklisted
        if blacklist[v] {
            continue
        }

        // skip if the parent of the current head
        if let Some(p) = parent {
            if v == p {
                continue;
            }
        }

        if visited[v] {
            low[u] = low[u].min(timing[v]);
        } else {
            dfs_articulation(
                adj,
                n,
                v,
                Some(u),
                timer,
                visited,
                timing,
                low,
                ap,
                blacklist,
            );
            low[u] = low[u].min(low[v]);
            if low[v] >= timing[u] && parent.is_some() {
                ap[u] = true;
            }
            children += 1;
        }
    }
    if parent.is_none() && children > 1 {
        ap[u] = true;
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
        assert_eq!(ap, vec![false, false, true, false]);
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
        assert_eq!(ap, vec![false, true, true, false]);
    }

    #[test]
    fn articulation_points_no_used_c() {
        let n = 7;
        let mut adj = fixedbitset::FixedBitSet::with_capacity(n * n);
        let used = vec![false; n];
        insert_graph(&mut adj, n, 0, 1);
        insert_graph(&mut adj, n, 0, 2);
        insert_graph(&mut adj, n, 1, 3);
        insert_graph(&mut adj, n, 2, 3);
        insert_graph(&mut adj, n, 2, 4);
        insert_graph(&mut adj, n, 3, 4);
        insert_graph(&mut adj, n, 4, 5);
        insert_graph(&mut adj, n, 4, 6);
        insert_graph(&mut adj, n, 5, 6);

        let ap = super::find_articulation_points(&adj, n, &used);
        assert_eq!(ap, vec![false, false, false, false, true, false, false]);
    }

    #[test]
    fn articulation_points_no_used_d() {
        let n = 7;
        let mut adj = fixedbitset::FixedBitSet::with_capacity(n * n);
        let used = vec![false; n];
        insert_graph(&mut adj, n, 0, 1);
        insert_graph(&mut adj, n, 0, 2);
        insert_graph(&mut adj, n, 1, 3);
        insert_graph(&mut adj, n, 2, 3);
        insert_graph(&mut adj, n, 3, 4);
        insert_graph(&mut adj, n, 3, 5);
        insert_graph(&mut adj, n, 4, 5);
        insert_graph(&mut adj, n, 4, 6);
        insert_graph(&mut adj, n, 5, 6);

        let ap = super::find_articulation_points(&adj, n, &used);
        assert_eq!(ap, vec![false, false, false, true, false, false, false]);
    }

    #[test]
    fn articulation_points_with_used() {
        let n = 4;
        let mut adj = fixedbitset::FixedBitSet::with_capacity(n * n);
        let used = vec![false, false, true, false];
        insert_graph(&mut adj, n, 0, 2);
        insert_graph(&mut adj, n, 1, 0);
        insert_graph(&mut adj, n, 1, 2);
        insert_graph(&mut adj, n, 2, 3);

        let ap = super::find_articulation_points(&adj, n, &used);
        assert_eq!(ap, vec![false, false, false, false]);
    }

    #[test]
    fn articulation_points_with_used_b() {
        let n = 4;
        let mut adj = fixedbitset::FixedBitSet::with_capacity(n * n);
        let used = vec![false, false, true, false];
        insert_graph(&mut adj, n, 0, 1);
        insert_graph(&mut adj, n, 1, 2);
        insert_graph(&mut adj, n, 2, 3);

        let ap = super::find_articulation_points(&adj, n, &used);
        assert_eq!(ap, vec![false, false, false, false]);
    }

    #[test]
    fn articulation_points_with_used_c() {
        let n = 7;
        let mut adj = fixedbitset::FixedBitSet::with_capacity(n * n);
        let mut used = vec![false; n];
        used[2] = true;
        
        insert_graph(&mut adj, n, 0, 1);
        insert_graph(&mut adj, n, 0, 2);
        insert_graph(&mut adj, n, 1, 3);
        insert_graph(&mut adj, n, 2, 3);
        insert_graph(&mut adj, n, 2, 4);
        insert_graph(&mut adj, n, 3, 4);
        insert_graph(&mut adj, n, 4, 5);
        insert_graph(&mut adj, n, 4, 6);
        insert_graph(&mut adj, n, 5, 6);

        let ap = super::find_articulation_points(&adj, n, &used);
        assert_eq!(ap, vec![false, true, false, true, true, false, false]);
    }

    #[test]
    fn articulation_points_with_used_d() {
        let n = 7;
        let mut adj = fixedbitset::FixedBitSet::with_capacity(n * n);
        let mut used = vec![false; n];
        used[2] = true;
        used[4] = true;

        insert_graph(&mut adj, n, 0, 1);
        insert_graph(&mut adj, n, 0, 2);
        insert_graph(&mut adj, n, 1, 3);
        insert_graph(&mut adj, n, 2, 3);
        insert_graph(&mut adj, n, 3, 4);
        insert_graph(&mut adj, n, 3, 5);
        insert_graph(&mut adj, n, 4, 5);
        insert_graph(&mut adj, n, 4, 6);
        insert_graph(&mut adj, n, 5, 6);

        let ap = super::find_articulation_points(&adj, n, &used);
        assert_eq!(ap, vec![false, true, false, true, false, true, false]);
    }
}
