use std::collections::{HashMap, HashSet};

use crate::graph::graph::Graph;

use super::graph::{Vertex, Weight};

pub struct DirectedGraph {
    adj_matrix: HashMap<Vertex, HashMap<Vertex, Weight>>,
    vertices: HashSet<Vertex>,
    _size: usize,
}

impl DirectedGraph {
    pub fn new(size: usize) -> DirectedGraph {
        DirectedGraph {
            adj_matrix: HashMap::new(),
            vertices: HashSet::new(),
            _size: size,
        }
    }
}

impl Graph for DirectedGraph {
    fn add_edge(&mut self, u: Vertex, v: Vertex, w: Weight) {
        if self.adj_matrix.get(&u).is_none() {
            self.adj_matrix.insert(u, HashMap::new());
        }
        self.adj_matrix.get_mut(&u).unwrap().insert(v, w);

        self.vertices.insert(u);
        self.vertices.insert(v);
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
}
