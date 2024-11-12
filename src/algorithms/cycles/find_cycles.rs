use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet, VecDeque},
    iter::empty,
    path::Path,
};

use crate::graph::{
    graph::Graph,
    undirected_graph::{UndirectedGraph, Vertex, Weight},
};

fn run(g: &dyn Graph<Vertex, Weight>) -> Vec<Path> {
    get_cycles(g)
}

fn get_cycles(g: &dyn Graph<Vertex, Weight>) -> Vec<Path> {
    let r = g.get_vertices().iter().next().unwrap();
    let fundamental_cycles = get_fundamental_cycles(g, r);
    let mut cycles = [];

    return powerset(fundamental_cycles.iter().collect())
        .iter()
        .filter(|subset| subset.is_empty())
        .map(|subset| {
            subset
                .into_iter()
                .fold(UndirectedGraph::new(), |new_cycle, cycle| new_cycle ^ cycle)
        })
        .collect();
}

fn get_fundamental_cycles(
    g: &dyn Graph<Vertex, Weight>,
    r: &Vertex,
) -> Vec<UndirectedGraph<Vertex, Weight>> {
    let mut in_t = HashSet::new();
    let mut t = UndirectedGraph::new();
    let mut q = VecDeque::new();
    let mut parents = vec![];
    let mut cycles = vec![];

    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        g._get_adj_list(v)
            .unwrap()
            .iter()
            .filter(|(u, _)| parents.get(*v).is_some_and(|p| p != *u))
            .for_each(|(u, _)| {
                if in_t.contains(u) {
                    let r_v_path = get_path_from_to(&t, r, v, &parents);
                    let r_u_path = get_path_from_to(&t, r, u, &parents);

                    let mut cycle = r_u_path ^ r_v_path;
                    cycle.add_edge(*u, *v, *g.get_weight(u, v).unwrap());
                    cycles.push(cycle);
                } else {
                    q.push_back(u);
                    in_t.insert(u);
                    t.add_edge(*v, *u, *g.get_weight(v, u).unwrap());
                    parents.insert(*u, *v);
                }
            });
    }

    cycles
}

fn get_path_from_to(
    g: &dyn Graph<Vertex, Weight>,
    v: &Vertex,
    u: &Vertex,
    parents: &Vec<Vertex>,
) -> UndirectedGraph<Vertex, Weight> {
    let mut path = UndirectedGraph::<Vertex, Weight>::new();
    let mut cur = u;
    while !cur.eq(v) {
        let p = &parents[*cur];
        path.add_edge(*cur, *p, *g.get_weight(cur, p).unwrap());
        cur = p
    }

    path
}

fn powerset<T>(s: &[T]) -> Vec<Vec<&T>> {
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element)
                .collect()
        })
        .collect()
}

/* def get_path_from_to(G: Graph, v: Vertex, u: Vertex, parents) -> Graph:
    """returns a graph representing the path from v to u"""
    path = Graph()
    cur = u
    while cur != None and cur != v:
        p = parents[cur]
        path.add_edge(cur, p, G.get_weight(cur, p))
        cur = p

    return path


def get_fundamental_cycles(G: Graph, r: Vertex) -> Set[Graph]:
    """returns a set of graphs represting the fundamental cycles"""
    in_t = defaultdict(bool)
    T = Graph()
    Q = deque([r])
    parents = defaultdict(lambda: None)
    cycles = set()

    while Q:
        v = Q.popleft()
        for u in G.get_adj_list_vertex(v):
            if u != parents[v]:
                if in_t[u]:
                    r_v_path = get_path_from_to(T, r, v, parents)
                    r_u_path = get_path_from_to(T, r, u, parents)
                    cycle = r_u_path ^ r_v_path
                    cycle.add_edge(u, v, G.get_weight(u, v))
                    cycles.add(cycle)
                else:
                    Q.append(u)
                    in_t[u] = True
                    T.add_edge(v, u, G.get_weight(v, u))
                    parents[u] = v

    return cycles
 */
