use crate::graph::edges::edge::Edge;
use crate::graph::edges::edge_trait::{BidirectionalEdgeTrait, EdgeTrait};
use std::ops::{Index, IndexMut};

pub mod edges;
pub mod scc;
pub mod topological_sort;

#[derive(Clone)]
pub struct Graph<E: EdgeTrait> {
    edges: Vec<Vec<E>>,
    edge_count: usize,
}

// base Graph implementation
impl<E: EdgeTrait> Graph<E> {
    pub fn new(node_count: usize) -> Self {
        Self {
            edges: vec![Vec::new(); node_count],
            edge_count: 0,
        }
    }

    pub fn add_edge(&mut self, (from, mut edge): (usize, E)) -> usize {
        let to = edge.to();
        assert!(to < self.vertex_count());
        let direct_id = self.edges[from].len();
        edge.set_id(self.edge_count);
        self.edges[from].push(edge);
        if E::REVERSABLE {
            let rev_id = self.edges[to].len();
            self.edges[from][direct_id].set_reverse_id(rev_id);
            let mut rev_edge = self.edges[from][direct_id].reverse_edge(from);
            rev_edge.set_id(self.edge_count);
            rev_edge.set_reverse_id(direct_id);
            self.edges[to].push(rev_edge);
        }
        self.edge_count += 1;
        direct_id
    }

    pub fn add_vertices(&mut self, cnt: usize) {
        self.edges.resize(self.edges.len() + cnt, Vec::new());
    }

    pub fn clear(&mut self) {
        self.edge_count = 0;
        for ve in self.edges.iter_mut() {
            ve.clear();
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.edges.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    pub fn degrees(&self) -> Vec<usize> {
        self.edges.iter().map(|v| v.len()).collect()
    }
}

impl<E: BidirectionalEdgeTrait> Graph<E> {
    pub fn is_tree(&self) -> bool {
        if self.edge_count + 1 != self.vertex_count() {
            return false;
        } else {
            self.is_connected()
        }
    }

    pub fn is_connected(&self) -> bool {
        let mut dsu = DSU::new(self.vertex_count());
        for i in 0..self.vertex_count() {
            for e in self[i].iter() {
                dsu.union(i, e.to());
            }
        }

        dsu.set_count() == 1
    }
}

impl<E: EdgeTrait> Index<usize> for Graph<E> {
    type Output = [E];

    fn index(&self, index: usize) -> &Self::Output {
        &self.edges[index]
    }
}

impl<E: EdgeTrait> IndexMut<usize> for Graph<E> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.edges[index]
    }
}

impl Graph<Edge<()>> {
    pub fn with_edges(n: usize, edges: &[(usize, usize)]) -> Self {
        let mut graph = Self::new(n);
        for &(from, to) in edges {
            graph.add_edge(Edge::new(from, to));
        }
        graph
    }
}

impl<P: Clone> Graph<Edge<P>> {
    pub fn with_edges_with_payload(n: usize, edges: &[(usize, usize, P)]) -> Self {
        let mut graph = Self::new(n);
        for (from, to, p) in edges.iter() {
            graph.add_edge(Edge::with_payload(*from, *to, p.clone()));
        }
        graph
    }
}
