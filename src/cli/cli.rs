use clap::Parser;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::rc::Rc;

use crate::algorithms::cycles;
use crate::graph::directed_graph::DirectedGraph;
use crate::graph::graph::Graph;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Algorithm {
    IsAcyclic,
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
            let res = cycles::is_acyclic::run(g.deref());

            println!("Is acylic result: {:}", res);
        }
    }
}

fn read_graph(path: PathBuf) -> Box<dyn Graph> {
    let lines = fs::read_to_string(path).unwrap();
    let mut lines = lines.lines();
    let mut header = lines
        .next()
        .unwrap_or_else(|| panic!("Invalid format"))
        .split_whitespace();
    let n = header
        .next()
        .map(|v| v.parse::<usize>().unwrap())
        .unwrap_or_else(|| panic!("Invalid format"));
    let mut g = Box::new(DirectedGraph::new(n));

    lines.for_each(|v| {
        let mut line = v.split_whitespace();
        let u = line
            .next()
            .map(|v| v.parse::<usize>().unwrap() - 1)
            .unwrap_or_else(|| panic!("Invalid format"));
        let v = line
            .next()
            .map(|v| v.parse::<usize>().unwrap() - 1)
            .unwrap_or_else(|| panic!("Invalid format"));
        let w = line
            .next()
            .map(|v| v.parse::<i128>().unwrap() - 1)
            .unwrap_or_else(|| panic!("Invalid format"));

        g.add_edge(u, v, w);
        g.add_edge(v, u, w);
    });

    return g;
}
