use std::collections::BTreeMap;

use itertools::Itertools;

use super::prelude::*;

/// See [`rooted_subtree_fn`](Graph::rooted_subtree_fn)
pub enum RsfStep<'a, E, R> {
    Leaf {
        p: usize,
        edge: Option<E>,
        u: usize,
    },
    Sibling {
        p: usize,
        r0: &'a R,
        r1: &'a R,
    },
    Parent {
        pp: usize,
        edge: Option<E>,
        p: usize,
        r: &'a R,
    },
}

impl<E> Tree<E>
where E: Copy
{
    #[cfg_attr(doc, aquamarine::aquamarine)]
    /// *O*(*n*)-step calculation of function `f()` for:
    ///
    /// - All of the 2(*n* - 1) rooted subtrees induced by choosing an edge (*p*, *u*), rooting at *u*, and removing all vertices on the *p* side
    /// ```mermaid
    /// graph TD
    ///     0(("0 (p)")) --- 1((1))
    ///     0 -.- 2(("2 (u)"))
    ///     1 --- 3((3))
    ///     2 === 4((4))
    ///     2 === 5((5))
    ///     style 2 fill:#267,stroke:#bbb,stroke-width:4px
    ///     style 4 fill:#267,stroke:#bbb,stroke-width:4px
    ///     style 5 fill:#267,stroke:#bbb,stroke-width:4px
    /// ```
    /// ```mermaid
    /// graph TD
    ///     0(("0 (u)")) === 1((1))
    ///     0 -.- 2(("2 (p)"))
    ///     1 === 3((3))
    ///     2 --- 4((4))
    ///     2 --- 5((5))
    ///     style 0 fill:#267,stroke:#bbb,stroke-width:4px
    ///     style 1 fill:#267,stroke:#bbb,stroke-width:4px
    ///     style 3 fill:#267,stroke:#bbb,stroke-width:4px
    /// ```
    ///
    /// - All of the *n* trees rooted at some node *u* of the tree
    /// ```mermaid
    /// graph TD
    ///     0((0)) === 1((1))
    ///     0 === 2(("2 (u)"))
    ///     1 === 3((3))
    ///     2 === 4((4))
    ///     2 === 5((5))
    ///     style 0 fill:#267,stroke:#bbb,stroke-width:4px
    ///     style 1 fill:#267,stroke:#bbb,stroke-width:4px
    ///     style 2 fill:#267,stroke:#bbb,stroke-width:4px
    ///     style 3 fill:#267,stroke:#bbb,stroke-width:4px
    ///     style 4 fill:#267,stroke:#bbb,stroke-width:4px
    ///     style 5 fill:#267,stroke:#bbb,stroke-width:4px
    /// ```
    ///
    /// # Requirements
    /// Using very informal notation:
    ///
    /// - `f()` must be calculable for a leaf node
    /// - `f()` must have some way to efficiently "combine two sibling subtrees". Let us call this operation ⨂
    /// - `f()` must have some way to efficiently "combine a subtree, with no siblings, with its parent". Let us call this operation ⨁
    ///
    /// Then for a parent *p* of siblings *u*, *v*, *w*, the idea is
    ///
    /// *f*(*p*) = *p* ⨁ [ *f*(*u*) ⨂ *f*(*v*) ⨂ *f*(*w*) ]
    ///
    /// # API
    /// `f()` will be passed [`RsfStep`]s:
    /// - `f(RsfStep::Leaf { p, edge, u })` must return *f*(*u*) for a leaf node `u`
    /// - `f(RsfStep::Sibling { p, r0, r1 })` must return `r0` ⨂ `r1`
    /// - `f(RsfStep::Parent { pp, edge, p, r })` must return `p` ⨁ `r`
    ///
    /// When all steps are complete, the returned [`BTreeMap`] contains 3*n* - 2 entries:
    /// - `[&(p, u)]` stores *f*() for an induced subtree rooted at `u` and pruned at `p` as described above
    /// - `[&(`[`NONE`]`, u)]` stores *f*() for the full tree rooted at `u` as described above
    ///
    /// # Complexity
    /// *O*(*n* log *n*), due to `BTreeMap`, assuming `f()` is *O*(1)
    ///
    /// # Examples
    /// Calculate all subtree sizes in a 6-pointed (*n* = 7) star graph
    /// ```
    /// use rithmic::graph::prelude::*;
    ///
    /// let g = Tree::star(7);
    /// let rsf = g.rooted_subtree_fn( |step| {
    ///     match step {
    ///         RsfStep::Leaf { .. } => 1,
    ///         RsfStep::Sibling { r0, r1, .. } => *r0 + *r1,
    ///         RsfStep::Parent { r, .. } => *r + 1,
    ///     }
    /// });
    ///
    /// assert_eq!(rsf.len(), 2*6 + 7);
    /// for u in 1..7 {
    ///     assert_eq!(rsf[&(0, u)], 1);
    ///     assert_eq!(rsf[&(u, 0)], 6);
    /// }
    /// for u in 0..7 {
    ///     assert_eq!(rsf[&(NONE, u)], 7);
    /// }
    /// ```
    pub fn rooted_subtree_fn<R>(&self, f: impl FnMut(RsfStep<E, R>) -> R) -> BTreeMap<(usize, usize), R>
    where R: Clone {
        self._rooted_subtree_fn::<R, true>(f)
    }

    /// Run [`rooted_subtree_fn`](Graph::rooted_subtree_fn) but stop after the first upward DFS (normally there are two; one up then down). This may reduce runtime by more than half. The returned [`BTreeMap`] contains:
    /// - `[&(p, u)]` for the *n* - 1 induced subtrees where `u` is farther from `0` than `p`, but not the reverse
    /// - `[&(`[`NONE`]`, 0)]` for the full tree rooted at `0`, but no other roots
    pub fn rooted_subtree_fn_half<R>(&self, f: impl FnMut(RsfStep<E, R>) -> R) -> BTreeMap<(usize, usize), R>
    where R: Clone {
        self._rooted_subtree_fn::<R, false>(f)
    }

    fn _rooted_subtree_fn<R, const FULL: bool>(&self, mut f: impl FnMut(RsfStep<E, R>) -> R)
        -> BTreeMap<(usize, usize), R>
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
        if !FULL {
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

            let adj_sorted;
            let adj = if self.adj[u].is_sorted_by_key(|&(v, _)| v) {
                &self.adj[u]
            }
            else {
                adj_sorted = self.adj[u].iter().copied().sorted_unstable_by_key(|&(v, _)| v).collect();
                &adj_sorted
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
