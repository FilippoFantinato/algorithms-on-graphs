use crate::graph::graph::Graph;

pub fn is_acyclic(g: &dyn Graph, start: usize) -> bool {
    let mut visited = vec![false; g.get_size()];

    for u in 0..g.get_size() {
        if !visited[u] {
            let cycle = dfs_cycle(g, start, None, &mut visited);

            if cycle {
                return false;
            }
        }
    }

    return true;
}

fn dfs_cycle(g: &dyn Graph, u: usize, parent: Option<usize>, visited: &mut Vec<bool>) -> bool {
    visited[u] = true;

    let adj_list = g.get_adj_list(u);
    for v in 0..adj_list.len() {
        if (parent.is_none() || v != parent.unwrap()) && g.get_weight(u, v) != 0 {
            if visited[v] {
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
    use super::*;
    use crate::graph::undirected_weighted_graph::UndirectedWeightedGraph;

    #[test]
    fn is_acyclic_success() {
        let mut g = UndirectedWeightedGraph::new(5);

        g.add_edge(0, 1, 1);
        g.add_edge(0, 2, 1);
        g.add_edge(0, 3, 1);
        // g.add_edge(1, 2, 1);
        // g.add_edge(2, 3, 1);
        g.add_edge(2, 4, 1);

        let expected = true;
        let current = is_acyclic(&g, 0);
        assert_eq!(expected, current);
    }
}
