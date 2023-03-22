use fixedbitset::FixedBitSet;
use petgraph::{EdgeType, Graph};

pub struct Bitgraph {
    adj: FixedBitSet,
    n: usize,
    is_dir: bool,
    neighbors: Vec<Vec<usize>>,
}
impl Bitgraph {
    pub fn from_graph<Ty: EdgeType>(graph: &Graph<(), (), Ty>) -> Self {
        let n = graph.node_count();
        let is_dir = Ty::is_directed();
        let mut adj = FixedBitSet::with_capacity(n * n);
        let mut neighbors = vec![Vec::with_capacity(n); n];
        for edge in graph.edge_indices() {
            let (src, dst) = graph.edge_endpoints(edge).unwrap();
            adj.insert(src.index() * n + dst.index());
        }

        for u in 0..n {
            for v in u + 1..n {
                if adj.contains(u * n + v) || adj.contains(v * n + u) {
                    neighbors[u].push(v);
                    neighbors[v].push(u);
                }
            }
        }

        Bitgraph {
            adj,
            n,
            is_dir,
            neighbors,
        }
    }

    pub fn is_connected(&self, u: usize, v: usize) -> bool {
        self.adj.contains(u * self.n + v)
    }

    pub fn n_nodes(&self) -> usize {
        self.n
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn adjacency(&self) -> &FixedBitSet {
        &self.adj
    }

    pub fn overwrite_adjacency(&mut self, adj: &FixedBitSet) {
        self.adj = adj.clone();
    }

    pub fn neighbors(&self, u: usize) -> &Vec<usize> {
        &self.neighbors[u]
    }

    pub fn as_bitvec(&self) -> Vec<usize> {
        let mut bitvec = Vec::with_capacity(self.n * self.n);
        for u in 0..self.n {
            for v in 0..self.n {
                if self.is_connected(u, v) {
                    bitvec.push(1);
                } else {
                    bitvec.push(0);
                }
            }
        }
        bitvec
    }

    #[allow(dead_code)]
    pub fn pprint(&self) {
        for u in 0..self.n {
            for v in 0..self.n {
                if self.is_connected(u, v) {
                    print!("1");
                } else {
                    print!("0");
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod testing {

    use petgraph::Directed;

    use super::*;

    fn build_graph() -> Graph<(), (), Directed> {
        let edges = vec![(1, 0), (2, 0)];
        
        Graph::from_edges(&edges)
    }

    #[test]
    fn test_from_graph() {
        let graph = build_graph();
        let bitgraph = Bitgraph::from_graph(&graph);
        assert_eq!(bitgraph.n_nodes(), 3);
        for i in 0..bitgraph.n_nodes() {
            for j in 0..bitgraph.n_nodes() {
                if i == 1 && j == 0 {
                    assert!(bitgraph.is_connected(i, j));
                } else if i == 2 && j == 0 {
                    assert!(bitgraph.is_connected(i, j));
                } else {
                    assert!(!bitgraph.is_connected(i, j));
                }
            }
        }
    }
}
