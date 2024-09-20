use std::{path::PathBuf, str::FromStr};

use algorithms_on_graphs::cli::cli::{run_cli, Algorithm, Args};
use algorithms_on_graphs::graph::graph::Path;

#[test]
#[should_panic]
fn wrong_header() {
    let args = Args {
        algorithm: Algorithm::IsAcyclic,
        file: PathBuf::from_str("./tests/test_dataset/wrong_header.txt").unwrap(),
        start: None,
    };
    run_cli(&args);
}

#[test]
#[should_panic]
fn wrong_first_vertex() {
    let args = Args {
        algorithm: Algorithm::IsAcyclic,
        file: PathBuf::from_str("./tests/test_dataset/wrong_first_vertex.txt").unwrap(),
        start: None,
    };
    run_cli(&args);
}

#[test]
#[should_panic]
fn wrong_second_vertex() {
    let args = Args {
        algorithm: Algorithm::IsAcyclic,
        file: PathBuf::from_str("./tests/test_dataset/wrong_second_vertex.txt").unwrap(),
        start: None,
    };
    run_cli(&args);
}

#[test]
#[should_panic]
fn wrong_weight() {
    let args = Args {
        algorithm: Algorithm::IsAcyclic,
        file: PathBuf::from_str("./tests/test_dataset/wrong_weight.txt").unwrap(),
        start: None,
    };
    run_cli(&args);
}
