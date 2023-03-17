use anyhow::{Result, bail};
use petgraph::{Graph, Directed};
use graph6_rs::DiGraph;
use std::{io::{BufRead, BufReader}, fs::File};

pub fn iter_graphs_from_file(path: &str) -> impl Iterator<Item = Graph<(), (), Directed>> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|line| load_repr(&line.unwrap()))
}

pub fn load_repr(repr: &str) -> Graph<(), (), Directed> {
    let graph = DiGraph::from_d6(repr).unwrap();
    let mut g = Graph::with_capacity(graph.n, graph.bit_vec.iter().sum());
    for _ in 0..graph.n {
        g.add_node(());
    }
    for u in 0..graph.n {
        for v in 0..graph.n {
            if graph.bit_vec[u * graph.n + v] == 1 {
                g.add_edge((u as u32).into(), (v as u32).into(), ());
            }
        }
    }
    g
}


/// Load a graph from a file
///
/// Expects a 1-Indexed numeric white-space delimited edgelist.
pub fn load_numeric_graph(filepath: &str, include_loops: bool) -> Result<Graph<(), (), Directed>> {
    let mut reader = File::open(filepath).map(BufReader::new)?;
    load_numeric_graph_from_buffer(&mut reader, include_loops)
}

/// Load a graph from a buffer
///
/// Expects a 1-Indexed numeric white-space delimited edgelist.
pub fn load_numeric_graph_from_buffer<B: BufRead>(
    buffer: &mut B,
    include_loops: bool,
) -> Result<Graph<(), (), Directed>> {
    let mut edges = Vec::new();
    for line in buffer.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        let u = split.next().unwrap().parse::<u32>()?;
        let v = split.next().unwrap().parse::<u32>()?;
        if u == 0 || v == 0 {
            bail!("ERROR: Found a node index: 0; Please use 1-indexed node indices.");
        }
        if !include_loops && u == v {
            continue;
        } else {
            edges.push((u - 1, v - 1));
        }
    }
    Ok(Graph::from_edges(&edges))
}
