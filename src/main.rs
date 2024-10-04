pub mod algorithms;
pub mod cli;
pub mod data_structures;
pub mod graph;

use algorithms::connected_components;
use clap::Parser;
use cli::cli::Algorithm;
use graph::{
    graph::Path,
    undirected_graph::{Vertex, Weight},
};

use crate::cli::cli::{run_cli, Args};

fn main() {
    let args: Args = Args::parse();

    let res = run_cli(&args);

    match args.algorithm {
        Algorithm::IsAcyclic => {
            println!("Is acylic result: {:}", res.downcast_ref::<bool>().unwrap());
        }
        Algorithm::KruskalNaive => {
            let path = res.downcast_ref::<Path<Vertex, Weight>>().unwrap();
            let weight: Weight = path.iter().map(|e| e.2).sum();

            println!("Kruskal naive path: {:?}", path);
            println!("Kruskal naive weight: {:?}", weight);
        }
        Algorithm::KruskalUnionFind => {
            let path = res.downcast_ref::<Path<Vertex, Weight>>().unwrap();
            let weight: Weight = path.iter().map(|e| e.2).sum();

            println!("Kruskal union find path: {:?}", path);
            println!("Kruskal union find weight: {:?}", weight);
        }
        Algorithm::Prim => {
            let path = res.downcast_ref::<Path<Vertex, Weight>>().unwrap();
            let weight: Weight = path.iter().map(|e| e.2).sum();

            println!("Prim path: {:?}", path);
            println!("Prim weight: {:?}", weight);
        }
        Algorithm::CountConnectedComponents => {
            let connected_components = res.downcast_ref::<usize>().unwrap();

            println!("Connected components: {:}", connected_components);
        }
        Algorithm::CheckingConnectivity => {
            let connected = res.downcast_ref::<usize>().unwrap();

            println!("Checking for connectivity: {:}", connected);
        }
    };
}
