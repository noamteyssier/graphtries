mod bitgraph;
mod census;
mod cli;
mod gtrie;
mod io;
mod isomorphism;
mod node;
mod symmetry;

use anyhow::Result;
use clap::Parser;
use bitgraph::Bitgraph;
use cli::{Cli, Mode};
use graph_canon::CanonLabeling;
use gtrie::Gtrie;
use isomorphism::canonical_based_nauty;
use petgraph::{Directed, Graph};

fn build_gtrie(input: String, output: Option<String>, size: usize) -> Result<()> {
    let mut gtrie = Gtrie::new(4);
    io::iter_graphs_from_file(&input).for_each(|graph| {
        let canon = CanonLabeling::new(&graph);
        let canon_graph: Graph<(), (), Directed> = Graph::from(&canon);
        let mut bgraph = Bitgraph::from_graph(&canon_graph);
        let canon_based_nauty = canonical_based_nauty(&bgraph.adjacency(), size, canon.orbits());
        bgraph.overwrite_adjacency(canon_based_nauty.adjacency());
        gtrie.insert(&bgraph, canon_based_nauty.conditions());
    });
    match output {
        Some(path) => gtrie.write_to_file(&path),
        None => gtrie.write_to_stdout(),
    }
}

fn enumerate_subgraphs(gtrie: String, input: String) -> Result<()> {
    let graph = io::load_numeric_graph(&input, true)?;
    let query = Bitgraph::from_graph(&graph);
    let mut gtrie = Gtrie::read_from_file(&gtrie)?;
    
    let now = std::time::Instant::now();
    gtrie.census(&query);
    println!("Elapsed: {} ms", now.elapsed().as_millis());

    gtrie.pprint();

    Ok(())
}

fn main() -> Result<()> {

    let cli = Cli::parse();
    match cli.mode {

        Mode::Enumerate { gtrie, input } => {
            enumerate_subgraphs(gtrie, input)?;
        },

        Mode::Build { input, output, size } => {
            build_gtrie(input, output, size)?;
        },
    }

    Ok(())
}
