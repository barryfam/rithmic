use crate::{SparseTable, OrdPair};
use super::*;
use dfs::DfsStep;
use dfs::DfsStepKind::TreeNodeFinish;

#[derive(Clone)]
pub struct Lca {
    rmq: SparseTable<(usize, usize)>,       // (depth, node id)
    u_map_rmq: Vec<usize>,                  // u_map_rmq[node id] = rmq index
}

impl Lca {
    pub fn query(&self, u: usize, v: usize) -> usize
    {
        let (i, j) = (self.u_map_rmq[u], self.u_map_rmq[v]).ordered();
        self.rmq.query(i..=j).1
    }
}

impl<E> Tree<E>
where E: Copy
{
    pub fn lca(&self, s: usize) -> Lca
    {
        let mut rmq = Vec::<(usize, usize)>::new();
        let mut u_map_rmq = vec![NONE; self.size()];

        let mut rmq_push = |u, d| {
            if rmq.last().map(|&x| x.1) == Some(u) { return }

            u_map_rmq[u] = rmq.len();
            rmq.push((d, u));
        };

        for step in self.dfs([s]) {
            if let DfsStep{kind: TreeNodeFinish, p, u, depth, ..} = step {
                rmq_push(u, depth);
                if p != NONE {
                    rmq_push(p, depth-1);
                }
            }
        }

        let rmq = SparseTable::from(rmq, |&a, &b| a.min(b));
        Lca{rmq, u_map_rmq}
    }
}
