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
use graph_canon::autom::AutoGroups;
use gtrie::Gtrie;
use isomorphism::canonical_based_nauty;
use petgraph::{Directed, Graph};

fn build_gtrie(input: String, output: Option<String>, size: usize) -> Result<()> {
    let mut gtrie = Gtrie::new(size);
    io::iter_graphs_from_file(&input).for_each(|graph| {
        let aut = AutoGroups::from_petgraph(&graph);
        let canon_graph: Graph<(), (), Directed> = Graph::from(&aut);
        let mut bgraph = Bitgraph::from_graph(&canon_graph);
        let canon_based_nauty = canonical_based_nauty(bgraph.adjacency(), size, &aut);
        bgraph.overwrite_adjacency(canon_based_nauty.adjacency());
        let repr = graph6_rs::write_graph6(bgraph.as_bitvec(), bgraph.n_nodes(), bgraph.is_dir());
        gtrie.insert(&bgraph, canon_based_nauty.conditions(), Some(repr));
    });
    match output {
        Some(path) => gtrie.write_to_file(&path),
        None => gtrie.write_to_stdout(),
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
        } => {
            build_gtrie(input, output, size)?;
        }

        Mode::Visualize { input } => {
            visualize_gtrie(input)?;
        }
    }

    Ok(())
}
