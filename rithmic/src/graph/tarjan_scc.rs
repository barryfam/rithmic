use crate::imin;

use super::*;
use super::dfs::DfsStepKind::*;

impl<E, const FLAGS: usize> Graph<E, FLAGS>
where E: Copy
{
    pub fn tarjan_scc(&self) -> Vec<Vec<usize>>
    {
        let mut scc = vec![];

        let n = self.size();

        let mut t: usize = 0;
        let mut preorder = vec![0; n];
        let mut lowlink = vec![0; n];
        let mut stack = vec![];

        for st in self.dfs(0..n) {
            let (p, u) = (st.p, st.u);
            match st.kind {
                TreeNodeStart => {
                    preorder[u] = t;
                    lowlink[u] = t;
                    t += 1;
                    stack.push(u);
                }
                BackEdge => {
                    imin!(lowlink[p], preorder[u]);
                }
                TreeNodeFinish => {
                    if p != NONE && preorder[u] != lowlink[u] {
                        imin!(lowlink[p], lowlink[u]);
                    }
                    else {
                        let mut cmpnt = vec![];
                        loop {
                            let v = stack.pop().unwrap();
                            cmpnt.push(v);
                            if v == u {
                                break
                            }
                        }
                        scc.push(cmpnt);
                    }
                }
                _ => {}
            }
        }

        scc.reverse();
        scc
    }
}
