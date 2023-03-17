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
