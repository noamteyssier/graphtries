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
        #[arg(short, long)]
        gtrie: String,

        #[arg(short, long)]
        input: String,
    },

    /// Build a gtrie from a list of graphs.
    Build {
        #[arg(short, long)]
        input: String,

        #[arg(short, long)]
        output: Option<String>,

        #[arg(short, long)]
        size: usize,
    },

    /// Visualize a gtrie.
    Visualize {
        #[arg(short, long)]
        input: String,
    },
}
