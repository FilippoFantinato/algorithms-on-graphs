use crate::data_structures::union_find::UnionFind;
use crate::graph::graph::{Edge, Graph, Path};
use crate::graph::undirected_graph::{Vertex, Weight};

pub fn run(g: &dyn Graph<Vertex, Weight>) -> Path<Vertex, Weight> {
    kruskal_union_find(g)
}

pub fn kruskal_union_find(g: &dyn Graph<Vertex, Weight>) -> Path<Vertex, Weight> {
    let mut uf = UnionFind::from(g.get_vertices());
    let mut edges: Vec<&Edge<Vertex, Weight>> = g.get_edges().iter().collect();
    edges.sort_by_key(|e| e.2);

    let mut mst = vec![];
    for e in edges {
        let (u, v) = (&e.0, &e.1);
        if uf.find(u) != uf.find(v) {
            mst.push(e.clone());
            uf.union(u, v);
        }
    }

    return mst;
}

mod tests {
    use crate::{
        algorithms::minimum_spanning_tree::kruskal_union_find::kruskal_union_find,
        graph::{graph::Graph, undirected_graph::UndirectedGraph},
    };

    #[test]
    fn kruskal_union_find_simple_graph() {
        let mut g = UndirectedGraph::new();

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
        let current = kruskal_union_find(&g);
        assert_eq!(expected, current);
    }

    #[test]
    fn kruskal_union_find_graph_with_cycle() {
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
            (6, 9, -7462),
            (8, 10, -976),
            (3, 8, -433),
            (5, 9, -432),
            (2, 6, -34),
            (6, 7, 442),
            (3, 7, 844),
            (1, 5, 2432),
            (2, 4, 4687),
        ];
        let current = kruskal_union_find(&g);
        assert_eq!(expected, current);
    }

    #[test]
    fn kruskal_union_find_full_connected_graph() {
        let mut g = UndirectedGraph::new();

        g.add_edge(1, 2, -544);
        g.add_edge(1, 3, 455);
        g.add_edge(1, 4, -12);
        g.add_edge(2, 3, 84);
        g.add_edge(2, 4, 27);
        g.add_edge(3, 4, -7);

        let expected = vec![(1, 2, -544), (1, 4, -12), (3, 4, -7)];
        let current = kruskal_union_find(&g);
        assert_eq!(expected, current);
    }
}
