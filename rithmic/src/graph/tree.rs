use std::collections::BTreeMap;

use crate::UnionFind;

use super::prelude::*;

impl<E> Tree<E>
where E: Copy
{
    pub fn dfs_up_tree(&self, s: usize) -> impl Iterator<Item=(usize, usize, E)> {
        self.dfs([s]).into_iter()
            .filter_map( |step|
                (step.kind == TreeNodeFinish && step.p != NONE)
                    .then(|| (step.u, step.p, step.edge.unwrap()))
            )
    }

    pub fn subtree_sizes_rel_to(&self, root: usize) -> Vec<usize>
    {
        let n = self.size();
        let mut st_sz = vec![0; n];
        let mut uf = UnionFind::new(n);

        for step in self.dfs([root]) {
            if let DfsStep { kind: TreeNodeFinish, p, u, .. } = step {
                st_sz[u] = uf.size(u);
                if p != NONE {
                    uf.union(p, u);
                }
            }
        }
        st_sz
    }

    pub fn subtree_sizes(&self) -> BTreeMap<(usize, usize), usize> {
        self.rooted_subtree_fn( |step| {
            match step {
                RsfStep::Leaf { .. } => 1,
                RsfStep::Sibling { r0, r1, .. } => *r0 + *r1,
                RsfStep::Parent { r, .. } => *r + 1,
            }
        })
    }
}
