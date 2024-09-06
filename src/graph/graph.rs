pub trait Graph {
    fn add_edge(&mut self, u: usize, v: usize, w: i128);
    fn get_size(&self) -> usize;
    fn get_adj_list(&self, v: usize) -> &Vec<i128>;
    fn get_weight(&self, v: usize, u: usize) -> i128;
}
