use clap::Parser;
use mockall::automock;
use std::any::Any;
use std::fs;
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::rc::Rc;

use crate::algorithms::{cycles, minimum_spanning_tree};
use crate::graph::graph::{Graph, Weight};
use crate::graph::undirected_graph::UndirectedGraph;

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Algorithm {
    IsAcyclic,
    KruskalNaive,
    KruskalUnionFind,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, value_enum)]
    pub algorithm: Algorithm,

    #[arg(short, long)]
    pub file: PathBuf,
}

pub fn run_cli(args: &Args) -> Box<dyn Any> {
    match args.algorithm {
        Algorithm::IsAcyclic => {
            let g = read_graph(&args.file);
            let res = cycles::is_acyclic::run(g.deref());

            Box::new(res)
        }
        Algorithm::KruskalNaive => {
            let g = read_graph(&args.file);
            let path = minimum_spanning_tree::kruskal_naive::run(g.deref());

            Box::new(path)
        }
        Algorithm::KruskalUnionFind => {
            let g = read_graph(&args.file);
            let path = minimum_spanning_tree::kruskal_union_find::run(g.deref());

            Box::new(path)
        }
    }
}

fn read_graph(path: &PathBuf) -> Box<dyn Graph> {
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
    let mut g = Box::new(UndirectedGraph::new());

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
