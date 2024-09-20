use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::graph::graph::{Edge, Graph};

pub type Vertex = usize;
pub type Weight = i32;

#[derive(PartialEq, Eq)]
pub struct UndirectedGraph<V: Eq + Clone + Hash + Ord + Copy, W: Eq + Clone + Hash> {
    adj_matrix: HashMap<V, HashMap<V, W>>,
    vertices: HashSet<V>,
    edges: HashSet<(V, V, W)>,
}

impl<V: Eq + Clone + Hash + Ord + Copy, W: Eq + Clone + Hash> UndirectedGraph<V, W> {
    pub fn new() -> UndirectedGraph<V, W> {
        UndirectedGraph {
            adj_matrix: HashMap::new(),
            vertices: HashSet::new(),
            edges: HashSet::new(),
        }
    }
}

impl<V: Eq + Clone + Hash + Ord + Copy, W: Eq + Clone + Hash> Graph<V, W>
    for UndirectedGraph<V, W>
{
    fn add_edge(&mut self, u: V, v: V, w: W) {
        if self.adj_matrix.get(&u).is_none() {
            self.adj_matrix.insert(u, HashMap::new());
        }
        if self.adj_matrix.get(&v).is_none() {
            self.adj_matrix.insert(v, HashMap::new());
        }
        self.adj_matrix.get_mut(&u).unwrap().insert(v, w.clone());
        self.adj_matrix.get_mut(&v).unwrap().insert(u, w.clone());

        self.vertices.insert(u);
        self.vertices.insert(v);

        self.edges.insert(if u <= v {
            (u, v, w.clone())
        } else {
            (v, u, w.clone())
        });
    }

    fn _get_size(&self) -> usize {
        self.vertices.len()
    }

    fn _get_adj_list(&self, v: &V) -> Option<&HashMap<V, W>> {
        self.adj_matrix.get(&v)
    }

    fn get_weight(&self, u: &V, v: &V) -> Option<&W> {
        self.adj_matrix.get(u).and_then(|el| el.get(v))
    }

    fn get_vertices(&self) -> &HashSet<V> {
        &self.vertices
    }

    fn get_edges(&self) -> &HashSet<Edge<V, W>> {
        &self.edges
    }

    fn delete_edge(&mut self, u: &V, v: &V) {
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

fn clean_vertex<V: Eq + Clone + Hash + Ord + Copy, W: Eq + Clone + Hash>(
    g: &mut UndirectedGraph<V, W>,
    t: &V,
) {
    if g.adj_matrix.get(t).unwrap().is_empty() {
        g.adj_matrix.remove(t);
        g.vertices.remove(t);
    }
}

mod tests {
    use std::collections::{HashMap, HashSet};
    use std::vec;

    use crate::graph::graph::{Edge, Graph};
    use crate::graph::undirected_graph::{Vertex, Weight};

    use super::UndirectedGraph;

    #[test]
    fn add_edge() {
        let mut g = UndirectedGraph::<Vertex, Weight>::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let expected_edges: HashSet<Edge<Vertex, Weight>> = HashSet::from([(0, 1, 2), (1, 4, 3)]);
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
    fn get_size() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let current = 3;
        let expected = g._get_size();
        assert_eq!(expected, current);
    }

    #[test]
    fn get_adj_list_existing_vertex() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let adj_list = HashMap::from([(0, 2), (4, 3)]);
        let expected = Some(&adj_list);
        let current = g._get_adj_list(&1);

        assert_eq!(expected, current);
    }

    #[test]
    fn get_adj_list_non_existent_vertex() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let expected = None;
        let current = g._get_adj_list(&5);

        assert_eq!(expected, current);
    }

    #[test]
    fn get_vertices() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let vertices = HashSet::from([0, 1, 4]);
        let expected: &HashSet<Vertex> = &vertices;
        let current = g.get_vertices();
        assert_eq!(expected, current);
    }

    #[test]
    fn get_edges() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let edges: HashSet<Edge<Vertex, Weight>> = HashSet::from([(0, 1, 2), (1, 4, 3)]);
        let expected: &HashSet<Edge<Vertex, Weight>> = &edges;
        let current = g.get_edges();
        assert_eq!(expected, current);
    }

    #[test]
    fn get_weight_existing_edge() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let expected = Some(&3);
        let current = g.get_weight(&1, &4);
        assert_eq!(expected, current);
    }

    #[test]
    fn get_weight_non_existent_edge() {
        let mut g = UndirectedGraph::new();

        g.add_edge(0, 1, 2);
        g.add_edge(1, 4, 3);

        let expected = None;
        let current = g.get_weight(&1, &5);
        assert_eq!(expected, current);
    }

    #[test]
    fn delete_edge_existing_edge() {
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
