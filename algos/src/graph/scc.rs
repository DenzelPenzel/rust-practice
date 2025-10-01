use crate::collections::bit_set::BitSet;
use crate::graph::Graph;
use crate::graph::edges::edge::Edge;
use crate::graph::edges::edge_trait::EdgeTrait;
use crate::misc::recursive_function::{Callable, RecursiveFunction};

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
        let mut vis = BitSet::new(n);
        
        for i in 0..n {
            if !vis[i] {
                let mut first_dfs = RecursiveFunction::new(|f, v| {
                    if vis[v] {
                        return;
                    }
                    vis.set(v);
                    for e in self[v].iter() {
                        f.call(e.to());
                    }
                    order.push(v);
                });
                first_dfs.call(i);
            }
        }

        vis.fill(false);
        let mut res = Graph::new(0);
        let mut index = 0usize;
        let mut next = vec![n; n];
        let mut queue = Vec::with_capacity(n);
        let mut gt = Graph::new(n);
        for i in 0..n {
            for e in self[i].iter() {
                gt.add_edge(Edge::new(e.to(), i));
            }
        }

        for i in (0..n).rev() {
            if !vis[order[i]] {
                let key = i;
                let mut second_dfs = RecursiveFunction::new(|f, v| {
                    if vis[v] {
                        if color[v] != index && next[color[v]] != key {
                            next[color[v]] = key;
                            queue.push(color[v]);
                        }
                        return;
                    }
                    color[v] = index;
                    vis.set(v);
                    for e in gt[v].iter() {
                        f.call(e.to());
                    }
                });
                second_dfs.call(order[i]);
                res.add_vertices(1);
                for j in queue.drain(..) {
                    res.add_edge(Edge::new(j, index));
                }
                index += 1;
            }
        }

        SCC {
            color,
            graph: res,
        }
    }
}
