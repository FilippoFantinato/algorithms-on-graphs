use clap::Parser;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::rc::Rc;

use crate::algorithms::{cycles, minimum_spanning_tree};
use crate::graph::graph::{Graph, Weight};
use crate::graph::undirected_graph::UndirectedGraph;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Algorithm {
    IsAcyclic,
    KruskalNaive,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum)]
    algorithm: Algorithm,

    #[arg(short, long)]
    file: PathBuf,
}

pub fn run_cli() {
    let args: Args = Args::parse();

    match args.algorithm {
        Algorithm::IsAcyclic => {
            let g = read_graph(args.file);
            let res = cycles::is_acyclic::run(&g);

            println!("Is acylic result: {:}", res);
        }
        Algorithm::KruskalNaive => {
            let g = read_graph(args.file);
            let path = minimum_spanning_tree::kruskal_naive::run(&g);
            let weight: i128 = path.iter().map(|e| (*e).2).sum();

            println!("Kruskal naive path: {:?}", path);
            println!("Kruskal naive weight: {:?}", weight);
        }
    }
}

fn read_graph(path: PathBuf) -> impl Graph {
    let lines = fs::read_to_string(path).unwrap();
    let mut lines = lines.lines();
    let mut header = lines
        .next()
        .unwrap_or_else(|| panic!("Invalid format"))
        .split_whitespace();
    let _n = header
        .next()
        .map(|v| v.parse::<usize>().unwrap())
        .unwrap_or_else(|| panic!("Invalid format"));
    let mut g = UndirectedGraph::new();

    lines.for_each(|v| {
        let mut line = v.split_whitespace();
        let u = line
            .next()
            .map(|v| v.parse::<usize>().unwrap())
            .unwrap_or_else(|| panic!("Invalid format"));
        let v = line
            .next()
            .map(|v| v.parse::<usize>().unwrap())
            .unwrap_or_else(|| panic!("Invalid format"));
        let w = line
            .next()
            .map(|v| v.parse::<i128>().unwrap())
            .unwrap_or_else(|| panic!("Invalid format"));

        g.add_edge(u, v, w);
    });

    return g;
}
