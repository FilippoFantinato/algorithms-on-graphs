use std::collections::{HashMap, HashSet};

pub type Edge<V, W> = (V, V, W);
pub type Path<V, W> = Vec<Edge<V, W>>;

pub trait Graph<V, W> {
    fn add_edge(&mut self, u: V, v: V, w: W);
    fn _get_size(&self) -> usize;
    fn _get_adj_list(&self, v: &V) -> Option<&HashMap<V, W>>;
    fn get_weight(&self, u: &V, v: &V) -> Option<&W>;
    fn get_vertices(&self) -> &HashSet<V>;
    fn get_edges(&self) -> &HashSet<Edge<V, W>>;
    fn delete_edge(&mut self, u: &V, v: &V);
}
