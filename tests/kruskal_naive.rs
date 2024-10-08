use std::{path::PathBuf, str::FromStr};

use algorithms_on_graphs::cli::cli::{run_cli, Algorithm, Args};
use algorithms_on_graphs::graph::graph::Path;
use algorithms_on_graphs::graph::undirected_graph::{Vertex, Weight};

#[test]
fn kruskal_naive() {
    let args = Args {
        algorithm: Algorithm::KruskalNaive,
        file: PathBuf::from_str("./dataset/input_random_01_10.txt").unwrap(),
        start: None,
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
    let current_path = res.downcast_ref::<Path<Vertex, Weight>>().unwrap();
    assert_eq!(expected_path, current_path);
}
