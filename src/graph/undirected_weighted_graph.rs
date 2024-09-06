use crate::graph::graph::Graph;

pub struct UndirectedWeightedGraph {
    adj_matrix: Vec<Vec<i128>>
}

impl Graph for UndirectedWeightedGraph {
    fn add_edge(&mut self, u: usize, v: usize, w: i128) {
        self.adj_matrix[u][v] = w;
        self.adj_matrix[v][u] = w;
    }

    fn get_size(&self) -> usize {
        self.adj_matrix.len()
    }

    fn get_adj_list(&self, v: usize) -> &Vec<i128> {
        self.adj_matrix.get(v).unwrap()
    }

    fn get_weight(&self, u: usize, v: usize) -> i128 {
        self.adj_matrix[u][v]
    }
}

impl UndirectedWeightedGraph {
    pub fn new(size: usize) -> UndirectedWeightedGraph {
        UndirectedWeightedGraph {
            adj_matrix: vec![vec![0; size]; size],
        }
    }
}
