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

fn build_gtrie(input: String, output: Option<String>, size: usize, visualize: bool) -> Result<()> {
    let mut gtrie = Gtrie::new(size);
    io::iter_graphs_from_file(&input).for_each(|graph| {

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
        },
    }
}

fn visualize_gtrie(gtrie: String) -> Result<()> {
    let gtrie = Gtrie::read_from_file(&gtrie)?;
    gtrie.pprint(false);
    Ok(())
}

fn enumerate_subgraphs(gtrie: String, input: String) -> Result<()> {
    let graph = io::load_numeric_graph(&input, true)?;
    let query = Bitgraph::from_graph(&graph);
    let mut gtrie = Gtrie::read_from_file(&gtrie)?;

    let now = std::time::Instant::now();
    gtrie.census(&query);
    eprintln!("Elapsed: {} ms", now.elapsed().as_millis());

    gtrie.pprint_results();

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.mode {
        Mode::Enumerate { gtrie, input } => {
            enumerate_subgraphs(gtrie, input)?;
        }

        Mode::Build {
            input,
            output,
            size,
            visualize,
        } => {
            build_gtrie(input, output, size, visualize)?;
        }

        Mode::Visualize { input } => {
            visualize_gtrie(input)?;
        }
    }

    Ok(())
}
