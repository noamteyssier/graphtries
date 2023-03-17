use fixedbitset::FixedBitSet;


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
pub fn canonical_based_nauty(adj: &FixedBitSet, size: usize) -> FixedBitSet {
    let mut new_adj = FixedBitSet::with_capacity(size * size);

    let mut global_adj = vec![vec![0; size]; size];
    let mut current_degree = vec![0; size];
    let mut global_degree = vec![0; size];
    let mut used = vec![false; size];
    let mut last_degree = vec![0; size];
    let mut labels = vec![0; size];

    // initializes the degrees
    for u in 0..size {
        for v in 0..size {
            if adj.contains(u * size + v) || adj.contains(v * size + u) {
                current_degree[u] += 1;
                global_degree[u] += 1;
            }
            if adj.contains(u * size + v) {
                global_adj[u][v] = 1;
            }
            if adj.contains(v * size + u) {
                global_adj[v][u] = 1;
            }
        }
    }

    // println!("----");
    // for v in global_adj.iter() {
    //     println!("{:?}", v);
    // }
    // println!("----");

    for pos in (0..size).rev() {
        // println!("pos: {}", pos);
        let ap = if pos > 1 {
            find_articulation_points(adj, size, &used)
        } else {
            vec![0; size]
        };
        // println!("{:?}", ap);

        let mut min_u = -1;
        for u in 0..size {

            // Skip if used or is an articulation point
            if used[u] || ap[u] != 0 { continue }

            // Iteratively replace min_u to the smallest degree vertex
            if min_u < 0 || current_degree[u] < current_degree[min_u as usize] {
                min_u = u as i32;

            // In the case of ties
            } else if current_degree[u] == current_degree[min_u as usize] {

                // Tie breaker 1: last_degree
                if last_degree[u] < last_degree[min_u as usize] {
                    min_u = u as i32;

                // Tie breaker 2: global_degree
                } else if last_degree[u] == last_degree[min_u as usize] {
                    if global_degree[u] < global_degree[min_u as usize] {
                        min_u = u as i32;
                    }
                }
            }
        }

        // println!("min_u: {}", min_u);

        used[min_u as usize] = true;
        labels[pos] = min_u as usize;
        last_degree.iter_mut().for_each(|x| *x = current_degree[*x as usize]);

        // update current degree removing min_u connections
        for v in 0..size {
            if adj.contains(min_u as usize * size + v) {
                current_degree[v] -= 1;
            }
            if adj.contains(v * size + min_u as usize) {
                current_degree[v] -= 1;
            }
        }
    }

    // write the new adjacency matrix given the labels
    for u in 0..size {
        for v in 0..size {
            if adj.contains(u * size + v) {
                new_adj.insert(labels[u] * size + labels[v]);
            }
        }
    }

    new_adj
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
            println!("Starting DFS from {}", i);
            dfs_articulation(i, -1, &mut timer, &mut visited, &mut tin, &mut low, adj_matrix, n, &mut ap, used);
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
                dfs_articulation(to, v as i32, timer, visited, tin, low, adj_matrix, n, ap, used);
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
