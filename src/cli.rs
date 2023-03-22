use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub mode: Mode,
}

#[derive(Subcommand, Debug)]
pub enum Mode {
    /// Enumerate all subgraphs of a given size given a graph and a gtrie.
    Enumerate {
        /// Path to the gtrie-formatted file (created with `build`).
        #[arg(short, long)]
        gtrie: String,

        /// Path to the input graph.
        #[arg(short, long)]
        input: String,
    },

    /// Build a gtrie from a list of graphs.
    Build {

        /// Path to the input file containing the graph6 formatted graphs
        #[arg(short, long)]
        input: String,

        /// Path to the output file where gtrie will be written (default = stdout).
        #[arg(short, long)]
        output: Option<String>,

        /// Size of subgraphs in the input file.
        #[arg(short, long)]
        size: usize,

        /// Visualize the gtrie (will not write gtrie if no output is provided).
        #[arg(short, long)]
        visualize: bool,
    },

    /// Visualize a gtrie.
    Visualize {
        #[arg(short, long)]
        input: String,
    },
}
