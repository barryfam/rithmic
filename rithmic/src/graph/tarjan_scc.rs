use derive_more::Deref;

use crate::{imin, BVec};

use super::*;
use super::dfs::DfsStepKind::*;

impl<E, const FLAGS: usize> Graph<E, FLAGS>
where E: Copy
{
    pub fn tarjan_scc(&self) -> SCCs
    {
        let n = self.size();

        let mut sccs = vec![];
        let mut map = vec![0; n];

        let mut t: usize = 0;
        let mut preorder = vec![0; n];
        let mut lowlink = vec![0; n];
        let mut stack = vec![];
        let mut in_stack = BVec::new(n);

        for st in self.dfs(0..n) {
            let (p, u) = (st.p, st.u);
            match st.kind {
                TreeNodeStart => {
                    preorder[u] = t;
                    lowlink[u] = t;
                    t += 1;
                    stack.push(u);
                    in_stack.set(u, true);
                }
                BackEdge => {
                    imin!(lowlink[p], preorder[u]);
                }
                CrossEdge => {
                    if in_stack[u] {
                        imin!(lowlink[p], lowlink[u]);
                    }
                }
                ForwardEdge => {}
                TreeNodeFinish => {
                    if preorder[u] != lowlink[u] {
                        imin!(lowlink[p], lowlink[u]);
                    }
                    else {
                        let mut cmpnt = vec![];
                        let ci = sccs.len();
                        loop {
                            let v = stack.pop().unwrap();
                            in_stack.set(v, false);
                            cmpnt.push(v);
                            map[v] = ci;
                            if v == u {
                                break
                            }
                        }
                        sccs.push(cmpnt);
                    }
                }
            }
        }

        sccs.reverse();
        for ci in &mut map { *ci = sccs.len()-1 - *ci }

        SCCs { sccs, map }
    }
}

#[derive(Default, Clone, Deref, Debug)]
pub struct SCCs {
    #[deref] sccs: Vec<Vec<usize>>,
    map: Vec<usize>,
}

impl SCCs {
    #[inline]
    pub fn map(&self, u: usize) -> usize {
        self.map[u]
    }
}
