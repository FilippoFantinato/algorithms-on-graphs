mod algorithms;
mod cli;
mod graph;

use clap::Parser;
use cli::cli::Algorithm;
use graph::graph::Path;

use crate::cli::cli::{run_cli, Args};

fn main() {
    let args: Args = Args::parse();

    let res = run_cli(&args);

    match args.algorithm {
        Algorithm::IsAcyclic => {
            println!("Is acylic result: {:}", res.downcast_ref::<bool>().unwrap());
        }
        Algorithm::KruskalNaive => {
            let path = res.downcast_ref::<Path>().unwrap();
            let weight: i128 = path.iter().map(|e| e.2).sum();

            println!("Kruskal naive path: {:?}", path);
            println!("Kruskal naive weight: {:?}", weight);
        }
    };
}
