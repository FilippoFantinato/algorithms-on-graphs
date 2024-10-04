use std::collections::HashSet;

use crate::graph::{
    graph::Graph,
    undirected_graph::{Vertex, Weight},
};

pub fn run(g: &dyn Graph<Vertex, Weight>, s: &Vertex, t: &Vertex) -> bool {
    checking_connectivity(g, s, t)
}

pub fn checking_connectivity(g: &dyn Graph<Vertex, Weight>, s: &Vertex, t: &Vertex) -> bool {
    dfs_checking_connectivity(g, &mut HashSet::new(), s, t)
}

pub fn dfs_checking_connectivity(
    g: &dyn Graph<Vertex, Weight>,
    visited: &mut HashSet<Vertex>,
    u: &Vertex,
    t: &Vertex,
) -> bool {
    visited.insert(*u);

    for (v, _) in g._get_adj_list(u).unwrap() {
        if v.eq(t) || (!visited.contains(v) && dfs_checking_connectivity(g, visited, v, t)) {
            return true;
        }
    }

    false
}
