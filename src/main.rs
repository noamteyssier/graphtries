mod bitgraph;
mod census;
mod cli;
mod gtrie;
mod io;
mod isomorphism;
mod node;
mod symmetry;

use anyhow::Result;
use bitgraph::Bitgraph;
use clap::Parser;
use cli::{Cli, Mode};
use graph_canon::CanonLabeling;
use gtrie::Gtrie;
use isomorphism::canonical_based_nauty;
use petgraph::{Directed, Graph};

fn build_gtrie(input: &str, output: Option<String>, size: usize, visualize: bool) -> Result<()> {
    let mut gtrie = Gtrie::new(size);
    io::iter_graphs_from_file(input).for_each(|graph| {
        // Create the canonical label of the graph
        let canon_label = CanonLabeling::new(&graph);

        // Convert the canonical label to a new graph
        let canon_graph: Graph<(), (), Directed> = canon_label.into();

        // Convert to a bitgraph
        let mut bgraph = Bitgraph::from_graph(&canon_graph);

        // Compute the nauty-based canonical labeling
        let canon_based_nauty = canonical_based_nauty(bgraph.adjacency(), size);

        // Overwrite the adjacency matrix with the new nauty-based one
        bgraph.overwrite_adjacency(canon_based_nauty.adjacency());

        // Generate the nauty-representation of the new graph
        let repr = graph6_rs::write_graph6(bgraph.as_bitvec(), bgraph.n_nodes(), bgraph.is_dir());

        // Insert the graph into the gtrie
        gtrie.insert(&bgraph, canon_based_nauty.conditions(), Some(repr));
    });

    if visualize {
        gtrie.pprint(false);
    }

    match output {
        Some(path) => gtrie.write_to_file(&path),
        None => {
            if visualize {
                Ok(())
            } else {
                gtrie.write_to_stdout()
            }
        }
    }
}

fn visualize_gtrie(gtrie: &str) -> Result<()> {
    let gtrie = Gtrie::read_from_file(&gtrie)?;
    gtrie.pprint(false);
    Ok(())
}

fn enumerate_subgraphs(gtrie: &str, input: &str) -> Result<Gtrie> {
    let graph = io::load_numeric_graph(&input, true)?;
    let query = Bitgraph::from_graph(&graph);
    let mut gtrie = Gtrie::read_from_file(&gtrie)?;

    let now = std::time::Instant::now();
    gtrie.census(&query);
    eprintln!("Elapsed: {} ms", now.elapsed().as_millis());
    eprintln!("Total subgraphs: {}", gtrie.total_subgraphs());

    gtrie.pprint_results();

    Ok(gtrie)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.mode {
        Mode::Enumerate { gtrie, input } => {
            enumerate_subgraphs(&gtrie, &input)?;
        }

        Mode::Build {
            input,
            output,
            size,
            visualize,
        } => {
            build_gtrie(&input, output, size, visualize)?;
        }

        Mode::Visualize { input } => {
            visualize_gtrie(&input)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod testing {
    use crate::enumerate_subgraphs;


    #[test]
    fn test_example_dir3() {
        let graph_path = "example/graphs/example.txt";
        let gtrie_path = "example/gtries/dir3.gt";
        let gtrie = enumerate_subgraphs(gtrie_path, graph_path).unwrap();
        let nonzero = gtrie.get_nonzero();
        assert_eq!(gtrie.total_subgraphs(), 16);
        assert_eq!(nonzero.len(), 4);
        nonzero.values().for_each(|&v| {
            let cond = v == 1 || v == 3 || v == 9;
            assert!(cond);
        });
    }

    #[test]
    fn test_example_dir4() {
        let graph_path = "example/graphs/example.txt";
        let gtrie_path = "example/gtries/dir4.gt";
        let gtrie = enumerate_subgraphs(gtrie_path, graph_path).unwrap();
        let nonzero = gtrie.get_nonzero();
        assert_eq!(gtrie.total_subgraphs(), 24);
        assert_eq!(nonzero.len(), 8);
        nonzero.values().for_each(|&v| {
            let cond = v == 3;
            assert!(cond);
        });
    }

    #[test]
    fn test_yeast_dir3() {
        let graph_path = "example/graphs/yeast.txt";
        let gtrie_path = "example/gtries/dir3.gt";
        let gtrie = enumerate_subgraphs(gtrie_path, graph_path).unwrap();
        let nonzero = gtrie.get_nonzero();
        assert_eq!(gtrie.total_subgraphs(), 13150);
        assert_eq!(nonzero.len(), 7);
        nonzero.values().for_each(|&v| {
            let cond = v == 1
                || v == 18
                || v == 70
                || v == 293
                || v == 889
                || v == 11878;
            assert!(cond);
        });
    }

    #[test]
    fn test_yeast_dir4() {
        let graph_path = "example/graphs/yeast.txt";
        let gtrie_path = "example/gtries/dir4.gt";
        let gtrie = enumerate_subgraphs(gtrie_path, graph_path).unwrap();
        let nonzero = gtrie.get_nonzero();
        assert_eq!(gtrie.total_subgraphs(), 183174);
        assert_eq!(nonzero.len(), 34);
        nonzero.values().for_each(|&v| {
            let cond = v == 1
                || v == 3
                || v == 4
                || v == 6
                || v == 9
                || v == 10
                || v == 11
                || v == 16
                || v == 17
                || v == 32
                || v == 55
                || v == 92
                || v == 102
                || v == 121
                || v == 125
                || v == 157
                || v == 286
                || v == 400
                || v == 989
                || v == 1125
                || v == 1460
                || v == 1843
                || v == 4498
                || v == 22995
                || v == 148761;
            assert!(cond);
        });
    }

}
