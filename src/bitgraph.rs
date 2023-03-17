
use fixedbitset::FixedBitSet;
use petgraph::{Graph, EdgeType};


pub struct Bitgraph {
    adj: FixedBitSet,
    n: usize,
    is_dir: bool,
}
impl Bitgraph {
    pub fn from_graph<Ty: EdgeType>(graph: &Graph<(), (), Ty>) -> Self {
        let n = graph.node_count();
        let is_dir = Ty::is_directed();
        let mut adj = FixedBitSet::with_capacity(n * n);
        for edge in graph.edge_indices() {
            let (src, dst) = graph.edge_endpoints(edge).unwrap();
            adj.insert(src.index() * n + dst.index());
        }
        Bitgraph { adj, n, is_dir }
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

    pub fn overwrite_adjacency(&mut self, adj: FixedBitSet) {
        self.adj = adj;
    }

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
        let edges = vec![
            (1, 0),
            (2, 0)
        ];
        let graph = Graph::from_edges(&edges);
        graph
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

