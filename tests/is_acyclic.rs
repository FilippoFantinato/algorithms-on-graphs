use std::{path::PathBuf, str::FromStr};

use algorithms_on_graphs::cli::cli::{run_cli, Algorithm, Args};
use algorithms_on_graphs::graph::graph::Path;

#[test]
fn is_acylic() {
    let args = Args {
        algorithm: Algorithm::IsAcyclic,
        file: PathBuf::from_str("./dataset/input_random_01_10.txt").unwrap(),
        start: None,
    };
    let res = run_cli(&args);

    let expected = &true;
    let current = res.downcast_ref::<bool>().unwrap();
    assert_eq!(expected, current);
}
