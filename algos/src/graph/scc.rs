use crate::collections::BitSet;
use crate::graph::Graph;
use crate::graph::edges::edge::Edge;
use crate::graph::edges::edge_trait::EdgeTrait;

pub struct SCC {
    pub color: Vec<usize>,
    pub graph: Graph<Edge<()>>,
}

pub trait SCCTrait {
    fn scc(&self) -> SCC;
}

impl<E: EdgeTrait> SCCTrait for Graph<E> {
    fn scc(&self) -> SCC {
        assert!(!E::REVERSABLE);
        let n = self.vertex_count();
        let mut order = Vec::with_capacity(n);
        let mut color = vec![0; n];
        // let mut visited = BitSet
    }
}
