use std::collections::{HashMap, HashSet};

use crate::graph::graph::Graph;

use super::graph::{Edge, Vertex, Weight};

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
            self.edges.remove(&e);
        });
    }
}
