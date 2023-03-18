mod bitgraph;
mod gtrie;
mod io;
mod isomorphism;
mod node;

use bitgraph::Bitgraph;
use gtrie::Gtrie;
use isomorphism::canonical_based_nauty;

fn main() {
    let s = 3;
    // let graph_path = "example/example.txt";
    let graph_path = "example/yeast.txt";
    let path = "example/dir3.g6";
    let mut gtrie = Gtrie::new(s);

    for graph in io::iter_graphs_from_file(path) {
        let mut bgraph = Bitgraph::from_graph(&graph);
        let canon = canonical_based_nauty(&bgraph.adjacency(), s);
        bgraph.overwrite_adjacency(canon);
        gtrie.insert(&bgraph);
    }

    let graph = io::load_numeric_graph(graph_path, true).unwrap();
    let query = Bitgraph::from_graph(&graph);
    gtrie.census(&query);

    gtrie.pprint();
}
