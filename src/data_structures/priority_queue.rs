use std::{collections::HashMap, hash::Hash};

struct PriorityQueue<T: Eq + Hash + Clone> {
    h: Vec<T>,
    indexes: HashMap<T, i32>,
    size: i32,
}

impl<T: Eq + Hash + Clone> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {}

    pub fn from(l: Vec<T>) -> PriorityQueue<T> {}

    pub fn extract_min(&mut self) -> Option<&T> {}
    pub fn insert(&mut self, el: T) {}
    pub fn decrease_key(&mut self, i: i32, new_priority: i32) {}
    pub fn get_index(&self, el: &T) -> i32 {}
    pub fn get_element(&self, i: i32) -> Option<&T> {}

    fn parent(&self, el: &T) -> Option<&T> {}
    fn left(&self, el: &T) -> Option<&T> {}
    fn right(&self, el: &T) -> Option<&T> {}
    fn min_heapify(&mut self, i: i32) {}
    fn swap(&mut self, i: i32, j: i32) {}
}
