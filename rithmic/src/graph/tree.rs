use std::collections::BTreeMap;
use std::iter;

use crate::UnionFind;

use super::prelude::*;

impl<E> Tree<E>
where E: Copy
{
    /// (Tree) Filter a DFS of the tree to only the *n* - 1 "upward" edges `u` → `p` where `p` is the parent of `u` in the DFS
    ///
    /// # Examples
    /// ```
    /// # use rithmic::graph::Tree;
    /// let g = Tree::line(4);
    ///
    /// assert_eq!(g.dfs_up_tree(0).collect::<Vec<_>>(), vec![
    ///     (3, (), 2),
    ///     (2, (), 1),
    ///     (1, (), 0)
    /// ]);
    /// ```
    pub fn dfs_up_tree(&self, s: usize) -> impl Iterator<Item=(usize, E, usize)> {
        self.dfs([s]).into_iter()
            .filter_map( |step|
                (step.kind == TreeNodeFinish && step.p != NONE)
                    .then(|| (step.u, step.edge.unwrap(), step.p))
            )
    }

    /// (Tree) List node visits in a DFS starting at `s`
    ///
    /// # Examples
    /// ```
    /// # use rithmic::graph::Tree;
    /// let g = Tree::line(4);
    ///
    /// assert_eq!(g.euler_tour(0).collect::<Vec<_>>(), vec![0, 1, 2, 3, 2, 1, 0]);
    /// ```
    pub fn euler_tour(&self, s: usize) -> impl Iterator<Item=usize> {
        self.dfs([s]).into_iter()
            .map( |step|
                match step.kind {
                    TreeNodeStart => step.p,
                    TreeNodeFinish => step.u,
                    _ => panic!("malformed tree")
                }
            )
            .skip(1)  // skip { TreeNodeStart, NONE, s } => NONE
    }

    /// (Tree) List edges traversed in a DFS starting at `s`
    ///
    /// # Examples
    /// ```
    /// # use rithmic::graph::Tree;
    /// let mut g = Tree::<&str>::new(3);
    /// g.add_edge(0, 1, "Main Street");
    /// g.add_edge(1, 2, "Privet Drive");
    ///
    /// assert_eq!(g.euler_tour_edges(0).collect::<Vec<_>>(), vec![
    ///     (0, "Main Street", 1),
    ///     (1, "Privet Drive", 2),
    ///     (2, "Privet Drive", 1),
    ///     (1, "Main Street", 0)
    /// ]);
    /// ```
    pub fn euler_tour_edges(&self, s: usize) -> impl Iterator<Item=(usize, E, usize)> {
        self.dfs([s]).into_iter()
            .skip(1)
            .map( |step|
                match step.kind {
                    TreeNodeStart => (step.p, step.edge.unwrap(), step.u),
                    TreeNodeFinish => (step.u, step.edge.unwrap(), step.p),
                    _ => panic!("malformed tree")
                }
            )
            .take(2*(self.size()-1))
    }

    /// (Tree) Relative to `root`, determine the number of nodes descendant of node *u* (including *u*) for all *u*, in *O*(*n*)
    ///
    /// # Examples
    /// A 3-pointed (*n* = 4) star graph, where node `0` is the center
    /// ```
    /// # use rithmic::graph::Tree;
    /// let g = Tree::star(4);
    ///
    /// assert_eq!(g.subtree_sizes_rel_to(0), vec![4, 1, 1, 1]);
    /// assert_eq!(g.subtree_sizes_rel_to(1), vec![3, 4, 1, 1]);
    /// ```
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

    /// (Tree) Determine the size of all 2(*n* - 1) possible subtrees that can be created by deleting a single edge. See [`rooted_subtree_fn`](Graph::rooted_subtree_fn) for illustration of the naming scheme
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

impl Tree<()>
{
    pub fn star(n: usize) -> Self {
        Self::from_iter_unweighted(n, (1..n).zip(iter::repeat(0)))
    }

    pub fn line(n: usize) -> Self {
        Self::from_iter_unweighted(n, (1..n).map(|u| (u-1, u)))
    }
}
