use std::collections::{HashMap, HashSet};
use std::default;
use std::hash::Hash;

pub struct UnionFind<T: Eq + Hash + Clone + Copy> {
    data: HashMap<T, T>,
    sizes: HashMap<T, i32>,
}

impl<T: Eq + Hash + Clone + Copy> UnionFind<T> {
    pub fn new() -> UnionFind<T> {
        UnionFind {
            data: HashMap::new(),
            sizes: HashMap::new(),
        }
    }

    pub fn from(l: &HashSet<T>) -> UnionFind<T> {
        let mut data = HashMap::new();
        let mut sizes = HashMap::new();

        for el in l {
            data.insert(el.clone(), el.clone());
            sizes.insert(el.clone(), 0);
        }

        UnionFind { data, sizes }
    }

    pub fn find(&self, el: &T) -> Option<&T> {
        self.parent(el).and_then(|parent| {
            if el == parent {
                return Some(parent);
            }

            return self.find(parent);
        })
    }

    pub fn union(&mut self, x: &T, y: &T) -> bool {
        let p1 = self.find(x);
        let p2 = self.find(y);

        match (p1, p2) {
            (Some(p1), Some(p2)) => {
                if p1 != p2 {
                    if self.sizes[p1] >= self.sizes[p2] {
                        self.change_parent(p2.clone(), p1.clone());
                    } else {
                        self.change_parent(p1.clone(), p2.clone());
                    }
                };

                return true;
            }
            (_, _) => false,
        }
    }

    pub fn parent(&self, el: &T) -> Option<&T> {
        self.data.get(el)
    }

    pub fn change_parent(&mut self, el: T, new_parent: T) {
        assert!(self.data.contains_key(&el));

        let current_parent = self.data[&el];
        self.sizes
            .insert(current_parent, self.sizes[&current_parent]);
        self.sizes.insert(new_parent, self.sizes[&new_parent] + 1);

        self.data.insert(el, new_parent);
    }
}

mod test {}
