use std::collections::{HashMap, HashSet};

use crate::graph::graph::Graph;

use super::graph::{Edge, Vertex, Weight};

#[derive(PartialEq, Eq)]
pub struct UndirectedGraph {
    adj_matrix: HashMap<Vertex, HashMap<Vertex, Weight>>,
    vertices: HashSet<Vertex>,
    edges: HashSet<Edge>,
}

impl UndirectedGraph {
    pub fn new() -> UndirectedGraph {
        UndirectedGraph {
            adj_matrix: HashMap::new(),
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }
}

impl Graph for UndirectedGraph {
    fn add_edge(&mut self, u: Vertex, v: Vertex, w: Weight) {
        if self.adj_matrix.get(&u).is_none() {
            self.adj_matrix.insert(u, HashMap::new());
        }
        if self.adj_matrix.get(&v).is_none() {
            self.adj_matrix.insert(v, HashMap::new());
        }
        self.adj_matrix.get_mut(&u).unwrap().insert(v, w);
        self.adj_matrix.get_mut(&v).unwrap().insert(u, w);

        self.vertices.insert(u);
        self.vertices.insert(v);

        self.edges
            .insert(if u <= v { (u, v, w) } else { (v, u, w) });
    }

    fn _get_size(&self) -> usize {
        self.vertices.len()
    }

    fn _get_adj_list(&self, v: &Vertex) -> Option<&HashMap<Vertex, Weight>> {
        self.adj_matrix.get(&v)
    }

    fn get_weight(&self, u: &Vertex, v: &Vertex) -> Option<&Weight> {
        self.adj_matrix.get(u).and_then(|el| el.get(v))
    }

    fn get_vertices(&self) -> &HashSet<Vertex> {
        &self.vertices
    }

    fn get_edges(&self) -> &HashSet<Edge> {
        &self.edges
    }

    fn delete_edge(&mut self, u: &Vertex, v: &Vertex) {
        self.get_weight(u, v).cloned().map(|w| {
            let e = if u <= v { (*u, *v, w) } else { (*v, *u, w) };
            self.adj_matrix.get_mut(u).unwrap().remove(v);
            self.adj_matrix.get_mut(v).unwrap().remove(u);

            clean_vertex(self, u);
            clean_vertex(self, v);

            self.edges.remove(&e);
        });
    }
}

fn clean_vertex(g: &mut UndirectedGraph, t: &Vertex) {
    if g.adj_matrix.get(t).unwrap().is_empty() {
        g.adj_matrix.remove(t);
        g.vertices.remove(t);
    }
}

mod tests {
    use std::collections::{HashMap, HashSet};
    use std::vec;

    use crate::graph::graph::{Edge, Graph, Vertex, Weight};

    use super::UndirectedGraph;

    #[test]
    fn add_edge_success() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let expected_edges: HashSet<(Vertex, Vertex, Weight)> =
            HashSet::from([(0, 1, 2), (1, 4, 3)]);
        assert_eq!(g.edges, expected_edges);

        let expected_vertices: HashSet<Vertex> = HashSet::from([0, 1, 4]);
        assert_eq!(g.vertices, expected_vertices);

        let expected_adj_matrix: HashMap<Vertex, HashMap<Vertex, Weight>> = HashMap::from([
            (0, HashMap::from([(1, 2)])),
            (1, HashMap::from([(0, 2), (4, 3)])),
            (4, HashMap::from([(1, 3)])),
        ]);
        assert_eq!(g.adj_matrix, expected_adj_matrix);
    }

    #[test]
    fn get_size_success() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let current = 3;
        let expected = g._get_size();
        assert_eq!(expected, current);
    }

    #[test]
    fn get_adj_list_existing_vertex_success() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let adj_list = HashMap::from([(0, 2), (4, 3)]);
        let expected = Some(&adj_list);
        let current = g._get_adj_list(&1);

        assert_eq!(expected, current);
    }

    #[test]
    fn get_adj_list_non_existent_vertex_success() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let expected = None;
        let current = g._get_adj_list(&5);

        assert_eq!(expected, current);
    }

    #[test]
    fn get_vertices_success() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let vertices = HashSet::from([0, 1, 4]);
        let expected: &HashSet<Vertex> = &vertices;
        let current = g.get_vertices();
        assert_eq!(expected, current);
    }

    #[test]
    fn get_edges_success() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let edges: HashSet<Edge> = HashSet::from([(0, 1, 2), (1, 4, 3)]);
        let expected: &HashSet<Edge> = &edges;
        let current = g.get_edges();
        assert_eq!(expected, current);
    }

    #[test]
    fn get_weight_existing_edge_success() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let expected = Some(&3);
        let current = g.get_weight(&1, &4);
        assert_eq!(expected, current);
    }

    #[test]
    fn get_weight_non_existent_edge_success() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let expected = None;
        let current = g.get_weight(&1, &5);
        assert_eq!(expected, current);
    }

    #[test]
    fn delete_edge_existing_edge_success() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        g.delete_edge(&0, &1);

        let expected_edges: HashSet<(Vertex, Vertex, Weight)> = HashSet::from([(1, 4, 3)]);
        assert_eq!(g.edges, expected_edges);

        let expected_vertices: HashSet<Vertex> = HashSet::from([1, 4]);
        assert_eq!(g.vertices, expected_vertices);

        let expected_adj_matrix: HashMap<Vertex, HashMap<Vertex, Weight>> =
            HashMap::from([(1, HashMap::from([(4, 3)])), (4, HashMap::from([(1, 3)]))]);
        assert_eq!(g.adj_matrix, expected_adj_matrix);
    }
}
