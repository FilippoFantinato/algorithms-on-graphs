use std::collections::HashSet;

use crate::graph::{
    graph::Graph,
    undirected_graph::{Vertex, Weight},
};

pub fn run(g: &dyn Graph<Vertex, Weight>) -> usize {
    count_connected_components(g)
}

pub fn count_connected_components(g: &dyn Graph<Vertex, Weight>) -> usize {
    let mut connected_components: usize = 0;
    let mut visited: HashSet<Vertex> = HashSet::new();
    for u in g.get_vertices() {
        if !visited.contains(u) {
            dfs_connected_components(g, u, &mut visited);
            connected_components = connected_components + 1;
        }
    }

    return connected_components;
}

pub fn dfs_connected_components(
    g: &dyn Graph<Vertex, Weight>,
    u: &Vertex,
    visited: &mut HashSet<Vertex>,
) {
    visited.insert(*u);

    for (v, _) in g._get_adj_list(u).unwrap() {
        if !visited.contains(v) {
            dfs_connected_components(g, v, visited);
        }
    }
}

mod tests {
    use super::*;
    use crate::{
        algorithms::connected_components::count_connected_components::count_connected_components,
        graph::undirected_graph::UndirectedGraph,
    };

    #[test]
    pub fn count_connected_components_one_component() {
        let mut g = UndirectedGraph::<Vertex, Weight>::new();

        g.add_edge(1, 2, 4993);
        g.add_edge(2, 3, 1392);
        g.add_edge(3, 4, 8856);
        g.add_edge(4, 5, -433);
        g.add_edge(5, 6, 6590);
        g.add_edge(6, 7, -7462);
        g.add_edge(7, 8, 6658);
        g.add_edge(8, 9, -976);
        g.add_edge(9, 10, 9698);

        let expected = 1;
        let current = count_connected_components(&g);

        assert_eq!(expected, current);
    }

    #[test]
    pub fn count_connected_components_two_components() {
        let mut g = UndirectedGraph::<Vertex, Weight>::new();

        g.add_edge(1, 2, 4993);
        g.add_edge(5, 6, 6590);
        g.add_edge(6, 7, -7462);
        g.add_edge(7, 8, 6658);
        g.add_edge(8, 9, -976);
        g.add_edge(9, 10, 9698);

        let expected = 2;
        let current = count_connected_components(&g);

        assert_eq!(expected, current);
    }

    #[test]
    pub fn count_connected_components_several_components() {
        let mut g = UndirectedGraph::<Vertex, Weight>::new();

        g.add_edge(1, 2, 4993);
        g.add_edge(3, 4, 8856);
        g.add_edge(5, 6, 6590);
        g.add_edge(7, 8, 6658);
        g.add_edge(9, 10, 9698);

        let expected = 5;
        let current = count_connected_components(&g);

        assert_eq!(expected, current);
    }
}
