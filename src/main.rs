mod bitgraph;
mod gtrie;
mod node;
mod io;
mod isomorphism;

use bitgraph::Bitgraph;
use gtrie::Gtrie;
use isomorphism::canonical_based_nauty;


fn main() {
    let s = 4;
    let path = "example/dir4.g6";
    let mut gtrie = Gtrie::new(s);

    let mut num_graphs = 0;
    for graph in io::iter_graphs_from_file(path) {
        let mut bgraph = Bitgraph::from_graph(&graph);
        // let canon = canonical_based_nauty(bgraph.adjacency(), bgraph.n_nodes());
        // bgraph.overwrite_adjacency(canon);
        gtrie.insert(&bgraph);
        // num_graphs += 1;
        // if num_graphs == 2 {
        //     break;
        // }
        // break;
    }
    gtrie.pprint();
}
