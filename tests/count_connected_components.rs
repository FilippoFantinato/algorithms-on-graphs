use std::{path::PathBuf, str::FromStr};

use algorithms_on_graphs::cli::cli::{run_cli, Algorithm, Args};
use algorithms_on_graphs::graph::graph::Path;
use algorithms_on_graphs::graph::undirected_graph::{Vertex, Weight};

#[test]
fn count_connected_components() {
    let args = Args {
        algorithm: Algorithm::CountConnectedComponents,
        file: PathBuf::from_str("./dataset/input_random_01_10.txt").unwrap(),
        start: None,
    };
    let res = run_cli(&args);

    let expected_path = &1;
    let current_path = res.downcast_ref::<usize>().unwrap();
    assert_eq!(expected_path, current_path);
}
