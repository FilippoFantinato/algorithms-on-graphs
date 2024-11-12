use std::{path::PathBuf, str::FromStr};

use algorithms_on_graphs::cli::cli::{run_cli, Algorithm, Args};
use algorithms_on_graphs::graph::graph::Path;
use algorithms_on_graphs::graph::undirected_graph::{Vertex, Weight};

#[test]
fn checking_connectivity() {
    let args = Args {
        algorithm: Algorithm::CheckingConnectivity,
        file: PathBuf::from_str("./dataset/input_random_01_10.txt").unwrap(),
        start: Some(1),
        end: Some(10),
    };
    let res = run_cli(&args);

    let expected_path = &true;
    let current_path = res.downcast_ref::<bool>().unwrap();
    assert_eq!(expected_path, current_path);
}
