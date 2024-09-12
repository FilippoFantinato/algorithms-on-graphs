use std::collections::HashSet;

use crate::graph::graph::{Graph, Vertex};

pub fn run(g: &dyn Graph) -> bool {
    return is_acyclic(g);
}

pub fn is_acyclic(g: &dyn Graph) -> bool {
    for u in g.get_vertices() {
        let mut visited: HashSet<Vertex> = HashSet::new();
        let cycle = dfs_cycle(g, u, None, &mut visited);
        if cycle {
            return false;
        }
    }

    return true;
}

fn dfs_cycle(
    g: &dyn Graph,
    u: &Vertex,
    parent: Option<&Vertex>,
    visited: &mut HashSet<Vertex>,
) -> bool {
    visited.insert(*u);

    for v in g.get_vertices() {
        if (parent.is_none() || *v != *parent.unwrap()) && g.get_weight(u, v).is_some() {
            if visited.get(v).is_some() {
                return true;
            } else {
                let cycle = dfs_cycle(g, v, Some(u), visited);
                if cycle {
                    return true;
                }
            }
        }
    }

    return false;
}

mod tests {
    use std::ops::Deref;

    use super::*;
    use crate::graph::directed_graph::DirectedGraph;

    #[test]
    fn graph_without_cycle_success() {
        let mut g = DirectedGraph::new(5);

        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(0, 3, 1);
        g.add_edge(2, 4, 1);

        let expected = true;
        let current = is_acyclic(&g);
        assert_eq!(expected, current);
    }

    #[test]
    fn graph_with_cycle_success() {
        let mut g = DirectedGraph::new(5);

        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(0, 3, 1);
        g.add_edge(1, 2, 1);
        g.add_edge(2, 3, 1);
        g.add_edge(2, 4, 1);

        let expected = false;
        let current = is_acyclic(&g);
        assert_eq!(expected, current);
    }

    #[test]
    fn non_connected_graph_without_cycle_success() {
        let mut g = DirectedGraph::new(8);

        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(0, 3, 1);
        g.add_edge(2, 4, 1);

        g.add_edge(5, 6, 1);
        g.add_edge(6, 7, 1);

        let expected = true;
        let current = is_acyclic(&g);
        assert_eq!(expected, current);
    }

    #[test]
    fn non_connected_graph_with_cycle_success() {
        let mut g = DirectedGraph::new(8);

        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(0, 3, 1);
        g.add_edge(2, 4, 1);

        g.add_edge(5, 6, 1);
        g.add_edge(6, 7, 1);
        g.add_edge(7, 5, 1);

        let expected = false;
        let current = is_acyclic(&g);
        assert_eq!(expected, current);
    }
}
