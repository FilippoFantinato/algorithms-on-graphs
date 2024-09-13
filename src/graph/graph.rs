use std::collections::{HashMap, HashSet};

pub type Vertex = usize;
pub type Weight = i128;
pub type Edge = (Vertex, Vertex, Weight);
pub type Path = Vec<Edge>;

pub trait Graph {
    fn add_edge(&mut self, u: Vertex, v: Vertex, w: Weight);
    fn _get_size(&self) -> usize;
    fn _get_adj_list(&self, v: &Vertex) -> Option<&HashMap<Vertex, Weight>>;
    fn get_weight(&self, u: &Vertex, v: &Vertex) -> Option<&Weight>;
    fn get_vertices(&self) -> &HashSet<Vertex>;
    fn get_edges(&self) -> &HashSet<Edge>;
    fn delete_edge(&mut self, u: &Vertex, v: &Vertex);
}
