use std::collections::BTreeMap;

use itertools::Itertools;

use super::prelude::*;

pub enum RsfStep<'a, E, R> {
    Leaf {
        p: usize,
        u: usize,
        edge: Option<E>,
    },
    Sibling {
        p: usize,
        r0: &'a R,
        r1: &'a R,
    },
    Parent {
        pp: usize,
        p: usize,
        edge: Option<E>,
        r: &'a R,
    },
}

impl<E> Tree<E>
where E: Copy
{
    pub fn rooted_subtree_fn<R>(&self, f: impl FnMut(RsfStep<E, R>) -> R) -> BTreeMap<(usize, usize), R>
    where R: Clone {
        self._rooted_subtree_fn(f, true)
    }

    pub fn rooted_subtree_fn_half<R>(&self, f: impl FnMut(RsfStep<E, R>) -> R) -> BTreeMap<(usize, usize), R>
    where R: Clone {
        self._rooted_subtree_fn(f, false)
    }

    fn _rooted_subtree_fn<R>(&self, mut f: impl FnMut(RsfStep<E, R>) -> R, full: bool) -> BTreeMap<(usize, usize), R>
    where R: Clone
    {
        let mut rsf = BTreeMap::<(usize, usize), R>::new();

        if self.size() == 0 {
            return rsf
        }
        else if self.size() == 1 {
            rsf.insert((NONE, 0), f(RsfStep::Leaf{p: NONE, u: 0, edge: None}));
            return rsf
        }

        let dfs = self.dfs([0]);

        for &step in dfs.iter().filter(|&s| (s.kind == TreeNodeFinish)) {
            let DfsStep{p, u, edge, ..} = step;

            let r = if u != 0 && self.adj[u].len() == 1 {
                f(RsfStep::Leaf{p, u, edge})
            }
            else {
                let mut children = rsf.range((u, 0)..(u, usize::MAX));
                let mut r = children.next().unwrap().1.clone();

                for (_, r1) in children {
                    r = f(RsfStep::Sibling{p: u, r0: &r, r1});
                }
                f(RsfStep::Parent{pp: p, p: u, edge, r: &r})
            };
            rsf.insert((p, u), r);
        }
        if !full {
            return rsf
        }

        for &step in dfs.iter().filter(|&s| (s.kind == TreeNodeStart)) {
            let DfsStep{u, ..} = step;

            let n = self.adj[u].len();
            if n == 1 {
                if u == 0 {
                    let (v, e) = self.adj[0][0];
                    let r = f(RsfStep::Leaf{p: v, u, edge: Some(e)});
                    rsf.insert((v, 0), r);
                }
                else {
                    let r = rsf.get(&(u, self.adj[u][0].0)).unwrap();
                    let r = f(RsfStep::Parent{pp: NONE, p: u, edge: None, r});
                    rsf.insert((NONE, u), r);
                }
                continue
            }

            let adj_owned;
            let adj = if self.adj[u].is_sorted_by_key(|&(v, _)| v) {
                &self.adj[u]
            }
            else {
                adj_owned = self.adj[u].iter().copied().sorted_unstable_by_key(|&(v, _)| v).collect();
                &adj_owned
            };
            let r_adj: Vec<&R> = rsf.range((u, 0)..(u, usize::MAX)).map(|(_, r)| r).collect();
            debug_assert_eq!(r_adj.len(), n);

            let mut prefix = Vec::<R>::with_capacity(n);
            let mut suffix = Vec::<R>::with_capacity(n-1);

            prefix.push(r_adj[0].clone());
            for &r1 in &r_adj[1..] {
                prefix.push(f(RsfStep::Sibling{p: u, r0: prefix.last().unwrap(), r1}));
            }
            suffix.push(r_adj[n-1].clone());
            for &r1 in r_adj[1..n-1].iter().rev() {
                suffix.push(f(RsfStep::Sibling{p: u, r0: suffix.last().unwrap(), r1}));
            }

            let (v0, e0) = adj[0];
            let (vn1, en1) = adj[n-1];
            rsf.insert((NONE, u), f(RsfStep::Parent{pp: NONE, p: u, edge: None     , r: &prefix.pop().unwrap()}));
            rsf.insert((vn1 , u), f(RsfStep::Parent{pp: vn1 , p: u, edge: Some(en1), r: &prefix.pop().unwrap()}));
            rsf.insert((v0  , u), f(RsfStep::Parent{pp: v0  , p: u, edge: Some(e0) , r: &suffix.pop().unwrap()}));
            for i in 1..n-1 {
                let (v, e) = adj[i];
                let r = f(RsfStep::Sibling{p: u, r0: &prefix[i-1], r1: &suffix[n-2-i]});
                let r = f(RsfStep::Parent{pp: v, p: u, edge: Some(e), r: &r});
                rsf.insert((v, u), r);
            }
        }

        rsf
    }
}
