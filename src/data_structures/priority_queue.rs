use std::{cmp::Ordering, collections::HashMap, fmt::Display, hash::Hash};

#[derive(Clone, Debug)]
pub struct PriorityQueue<T: Eq + Hash + Clone + PartialOrd + Display> {
    h: Vec<PriorityQueueItem<T>>,
    indexes: HashMap<T, usize>,
    size: usize,
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct PriorityQueueItem<T: Display>(pub T, pub i32);

impl<T: Eq + Hash + Clone + PartialOrd + Display> PriorityQueue<T> {
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            h: vec![],
            indexes: HashMap::new(),
            size: 0,
        }
    }

    pub fn from(l: &Vec<PriorityQueueItem<T>>) -> PriorityQueue<T> {
        let mut pq = PriorityQueue::new();

        for el in l {
            pq.insert(el.clone());
        }

        pq
    }

    pub fn insert(&mut self, el: PriorityQueueItem<T>) {
        let priority = el.1;
        let el = PriorityQueueItem(el.0.clone(), std::i32::MAX);
        self.h.push(el.clone());
        self.indexes.insert(el.0.clone(), self.h.len() - 1);

        self.decrease_key(self.h.len() - 1, priority);
    }

    pub fn extract_min(&mut self) -> Option<PriorityQueueItem<T>> {
        self.h.get(0).cloned().map(|min| {
            let max = self.h[self.h.len() - 1].clone();
            self.h[0] = max.clone();
            self.h.remove(self.h.len() - 1);
            self.indexes.remove(&min.0);
            self.indexes.insert(max.0.clone(), 0);
            self.min_heapify(0);

            min
        })
    }

    pub fn decrease_key(&mut self, i: usize, new_priority: i32) {
        self.h[i].1 = new_priority;
        let mut p = parent(i);
        let mut index = i;
        while index > 0 && self.h[p].1 > self.h[index].1 {
            self.swap(index, p);
            index = p;
            p = parent(index);
        }
    }

    pub fn get_index(&self, el: &T) -> Option<usize> {
        self.indexes.get(el).cloned()
    }
    pub fn get_element(&self, i: usize) -> Option<&PriorityQueueItem<T>> {
        self.h.get(i)
    }

    pub fn is_empty(&self) -> bool {
        self.h.is_empty()
    }

    fn min_heapify(&mut self, i: usize) {
        let l = left(i);
        let r = right(i);

        let mut min = i;

        if l < self.h.len() && self.h[l].1 < self.h[min].1 {
            min = l;
        }

        if r < self.h.len() && self.h[r].1 < self.h[min].1 {
            min = r;
        }

        if min != i {
            self.swap(min, i);
            self.min_heapify(min);
        }
    }

    fn swap(&mut self, i: usize, j: usize) {
        let x = self.h[i].clone();
        let y = self.h[j].clone();
        self.indexes.insert(x.0.clone(), j);
        self.indexes.insert(y.0.clone(), i);
        self.h[i] = y.clone();
        self.h[j] = x.clone();
    }
}

fn parent(i: usize) -> usize {
    if i == 0 {
        0
    } else {
        (i - 1) / 2
    }
}

fn left(i: usize) -> usize {
    2 * i + 1
}

fn right(i: usize) -> usize {
    2 * i + 2
}
