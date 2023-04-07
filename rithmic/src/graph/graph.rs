pub const NONE: usize = !0;

#[derive(Clone, Debug)]
pub struct Graph<E, const FLAGS: usize>
where E: Copy
{
    pub adj: Vec<Vec<(usize, E)>>
}

pub(super) mod flags {
    pub const SIMPLE:     usize = 0b0001;
    pub const UNDIRECTED: usize = 0b0010;
    pub const ACYCLIC:    usize = 0b0100;
    pub const CONNECTED:  usize = 0b1000;

    pub const FOREST: usize = SIMPLE | UNDIRECTED | ACYCLIC;
    pub const TREE:   usize = FOREST | CONNECTED;

    use super::Graph;
    pub type DirGraph<E> = Graph<E, 0>;
    pub type UndirGraph<E> = Graph<E, UNDIRECTED>;
    pub type SimpDirGraph<E> = Graph<E, SIMPLE>;
    const SU: usize = SIMPLE | UNDIRECTED;
    pub type SimpUndirGraph<E> = Graph<E, SU>;
    pub type Dag<E> = Graph<E, ACYCLIC>;
    pub type Forest<E> = Graph<E, FOREST>;
    pub type Tree<E> = Graph<E, TREE>;
}
pub use flags::*;

use crate::OrdPair;

pub(super) const fn has_flags(u: usize, flags: usize) -> bool {
    u & flags == flags
}

impl<E, const FLAGS: usize> Graph<E, FLAGS>
where E: Copy
{
    pub fn new(size: usize) -> Self {
        Self {
            adj: vec![vec![]; size]
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize, edge: E)
    {
        if has_flags(FLAGS, SIMPLE) {
            debug_assert_ne!(u, v, "A simple graph may not contain loops");
            // Only check last out-edge for multi-edgeness. A weak check, but avoids O(n²) debug build
            debug_assert_ne!(self.adj[u].last().map(|&(v, _)| v), Some(v),
                "A simple graph may not contain more than one edge per vertex pair");
        }

        self.adj[u].push((v, edge));
        if has_flags(FLAGS, UNDIRECTED) {
            self.adj[v].push((u, edge));
        }
    }

    pub fn from_iter(size: usize, iter: impl IntoIterator<Item=(usize, usize, E)>) -> Self {
        let mut g = Self::new(size);
        for (u, v, e) in iter.into_iter() {
            g.add_edge(u, v, e);
        }
        g
    }

    pub fn size(&self) -> usize {
        self.adj.len()
    }

    pub fn rev(&self) -> Self {
        let mut rev = Self::new(self.size());
        for (u, adj) in self.adj.iter().enumerate() {
            for &(v, e) in adj {
                rev.add_edge(v, u, e);
            }
        }
        rev
    }
}

impl<const FLAGS: usize> Graph<(), FLAGS>
{
    pub fn from_iter_unweighted(size: usize, iter: impl IntoIterator<Item=(usize, usize)>) -> Self {
        let mut g = Self::new(size);
        for (u, v) in iter.into_iter() {
            g.add_edge(u, v, ());
        }
        g
    }

    pub fn map<const DEST_FLAGS: usize>(&self, dest_size: usize, mut f: impl FnMut(usize) -> usize)
        -> Graph<(), DEST_FLAGS>
    {
        let mut edges = vec![];
        for (u, adj) in self.adj.iter().enumerate() {
            for &(v, _) in adj {
                let (mut fu, mut fv) = (f(u), f(v));
                if fu == fv {
                    continue
                }
                if has_flags(FLAGS, UNDIRECTED) {
                    (fu, fv) = (fu, fv).ordered();
                }
                if fu != NONE && fv != NONE {
                    edges.push((fu, fv));
                }
            }
        }
        edges.sort_unstable();
        edges.dedup();

        Graph::<(), DEST_FLAGS>::from_iter_unweighted(dest_size, edges)
    }
}


#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::super::prelude::*;

    #[test]
    fn test_from_iter() {
        let v = vec![(0, 1), (1, 2), (2, 3)];
        let _g = DirGraph::<()>::from_iter_unweighted(4, v);
    }

    #[test]
    fn test_dfs_basic() {
        let mut g = DirGraph::<()>::new(7);
        g.add_edge(0, 1, ());
        g.add_edge(1, 2, ());
        g.add_edge(2, 3, ());
        g.add_edge(3, 4, ());

        g.add_edge(1, 3, ());
        g.add_edge(4, 0, ());

        g.add_edge(5, 6, ());
        g.add_edge(6, 0, ());

        let dfs0 = g.dfs([0]);

        let counts = dfs0.iter().counts_by(|s| s.kind);
        assert_eq!(counts[&TreeNodeStart], 5);
        assert_eq!(counts[&TreeNodeFinish], 5);
        assert_eq!(counts[&BackEdge], 1);
        assert_eq!(counts.get(&ForwardEdge).unwrap_or(&0) +
                   counts.get(&CrossEdge).unwrap_or(&0), 1);

        let dfs2 = g.dfs([2, 3, 4, 5, 6, 0, 1]);

        let counts = dfs2.iter().counts_by(|s| s.kind);
        assert_eq!(counts[&TreeNodeStart], 7);
        assert_eq!(counts[&TreeNodeFinish], 7);
        assert_eq!(counts[&BackEdge], 2);
        assert!(!counts.contains_key(&ForwardEdge));
        assert_eq!(counts[&CrossEdge], 1);
    }

    #[test]
    fn test_dfs_undir_weighted() {
        let mut g = SimpUndirGraph::<f64>::new(3);
        g.add_edge(0, 1, 3.0);
        g.add_edge(1, 2, 5.0);
        g.add_edge(2, 0, 7.0);

        let dfs = g.dfs([0]);
        assert_eq!(dfs.len(), 3*2 + 1);
    }
}
