use itertools::Itertools;

use crate::UnionFind;

use super::*;

impl<const FLAGS: usize, E> Graph<E, FLAGS>
where E: Copy + Ord
{
    pub fn mst(&self) -> Graph<E, {FLAGS | SIMPLE | ACYCLIC}>
    {
        assert!(has_flags(FLAGS, UNDIRECTED), "MST requires an undirected graph");

        let n = self.size();
        let mut mst = Graph::new(n);
        let mut uf = UnionFind::new(n);

        for (u, v, e) in self.all_edges().sorted_unstable_by_key(|&(_u, _v, e)| e)
        {
            if !uf.union(u, v) { continue }
            mst.add_edge(u, v, e);
        }

        mst
    }
}
