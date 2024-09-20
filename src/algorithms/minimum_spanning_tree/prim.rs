use std::collections::HashMap;

use crate::{
    data_structures::priority_queue::{PriorityQueue, PriorityQueueItem},
    graph::{
        graph::{Edge, Graph, Path},
        undirected_graph::{Vertex, Weight},
    },
};

pub fn run(g: &dyn Graph<Vertex, Weight>, start: &Vertex) -> Path<Vertex, Weight> {
    prim(g, start)
}

pub fn prim(g: &dyn Graph<Vertex, Weight>, s: &Vertex) -> Path<Vertex, Weight> {
    let mut parents: HashMap<&Vertex, &Vertex> = HashMap::new();
    let mut pq = PriorityQueue::new();
    let mut mst = vec![];

    for v in g.get_vertices() {
        pq.insert(PriorityQueueItem(v, if v != s { std::i32::MAX } else { 0 }));
    }

    while !pq.is_empty() {
        let u = pq.extract_min().unwrap();

        if let Some(p) = parents.get(u.0) {
            mst.push((**p, *u.0, u.1));
        }

        for (v, w) in g._get_adj_list(u.0).unwrap() {
            if let Some(i) = pq.get_index(&v) {
                let t = pq.get_element(i).unwrap();
                if *w < t.1 {
                    parents.insert(t.0, u.0);
                    pq.decrease_key(i, *w);
                }
            }
        }
    }

    mst
}

mod tests {
    use super::*;
    use crate::graph::graph::Graph;
    use crate::graph::undirected_graph::{UndirectedGraph, Vertex, Weight};

    #[test]
    fn prim_simple_graph() {
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

        let expected = vec![
            (1, 2, 4993),
            (2, 3, 1392),
            (3, 4, 8856),
            (4, 5, -433),
            (5, 6, 6590),
            (6, 7, -7462),
            (7, 8, 6658),
            (8, 9, -976),
            (9, 10, 9698),
        ];
        let current = prim(&g, &1);

        assert_eq!(expected, current);
    }

    #[test]
    fn prim_graph_with_cycle() {
        let mut g = UndirectedGraph::new();

        g.add_edge(1, 2, 4993);
        g.add_edge(1, 5, 2432);
        g.add_edge(2, 3, 1392);
        g.add_edge(2, 4, 4687);
        g.add_edge(2, 6, -34);
        g.add_edge(3, 4, 8856);
        g.add_edge(3, 7, 844);
        g.add_edge(3, 8, -433);
        g.add_edge(5, 9, -432);
        g.add_edge(6, 9, -7462);
        g.add_edge(6, 7, 442);
        g.add_edge(8, 10, -976);

        let expected = vec![
            (1, 5, 2432),
            (5, 9, -432),
            (9, 6, -7462),
            (6, 2, -34),
            (6, 7, 442),
            (7, 3, 844),
            (3, 8, -433),
            (8, 10, -976),
            (2, 4, 4687),
        ];
        let current = prim(&g, &1);
        assert_eq!(expected, current);
    }

    #[test]
    fn prim_full_connected_graph() {
        let mut g = UndirectedGraph::new();

        g.add_edge(1, 2, -544);
        g.add_edge(1, 3, 455);
        g.add_edge(1, 4, -12);
        g.add_edge(2, 3, 84);
        g.add_edge(2, 4, 27);
        g.add_edge(3, 4, -7);

        let expected = vec![(1, 2, -544), (1, 4, -12), (4, 3, -7)];
        let current = prim(&g, &1);
        assert_eq!(expected, current);
    }
}
