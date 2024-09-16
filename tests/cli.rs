use std::{path::PathBuf, str::FromStr};

use algorithms_on_graphs::cli::cli::{run_cli, Algorithm, Args};
use algorithms_on_graphs::graph::graph::Path;

#[test]
#[should_panic]
fn wrong_header_panic() {
    let args = Args {
        algorithm: Algorithm::IsAcyclic,
        file: PathBuf::from_str("./tests/test_dataset/wrong_header.txt").unwrap(),
    };
    run_cli(&args);
}

#[test]
#[should_panic]
fn wrong_first_vertex_panic() {
    let args = Args {
        algorithm: Algorithm::IsAcyclic,
        file: PathBuf::from_str("./tests/test_dataset/wrong_first_vertex.txt").unwrap(),
    };
    run_cli(&args);
}

#[test]
#[should_panic]
fn wrong_second_vertex_panic() {
    let args = Args {
        algorithm: Algorithm::IsAcyclic,
        file: PathBuf::from_str("./tests/test_dataset/wrong_second_vertex.txt").unwrap(),
    };
    run_cli(&args);
}

#[test]
#[should_panic]
fn wrong_weight_panic() {
    let args = Args {
        algorithm: Algorithm::IsAcyclic,
        file: PathBuf::from_str("./tests/test_dataset/wrong_weight.txt").unwrap(),
    };
    run_cli(&args);
}

#[test]
fn is_acylic_success() {
    let args = Args {
        algorithm: Algorithm::IsAcyclic,
        file: PathBuf::from_str("./dataset/input_random_01_10.txt").unwrap(),
    };
    let res = run_cli(&args);

    let expected = &true;
    let current = res.downcast_ref::<bool>().unwrap();
    assert_eq!(expected, current);
}

#[test]
fn kruskal_naive_success() {
    let args = Args {
        algorithm: Algorithm::KruskalNaive,
        file: PathBuf::from_str("./dataset/input_random_01_10.txt").unwrap(),
    };
    let res = run_cli(&args);

    let expected_path = &vec![
        (6, 7, -7462),
        (8, 9, -976),
        (4, 5, -433),
        (2, 3, 1392),
        (1, 2, 4993),
        (5, 6, 6590),
        (7, 8, 6658),
        (3, 4, 8856),
        (9, 10, 9698),
    ];
    let current_path = res.downcast_ref::<Path>().unwrap();
    assert_eq!(expected_path, current_path);
}
