use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::mem;
use std::ops::Range;

use crate::{IntBitOps, Rangelike};

use super::MonoidOps;

#[derive(Default, Clone)]
pub struct SegTree<T, U, O: MonoidOps<T, U>>
where
    T: Default,
    U: Default
{
    len: usize,
    tree: Vec<T>,
    pending: Vec<U>,
    phantom: PhantomData<O>,
}

impl<T, U, O: MonoidOps<T, U>> From<Vec<T>> for SegTree<T, U, O>
where
    T: Default,
    U: Default
{
    fn from(mut vec: Vec<T>) -> Self
    {
        let len = vec.len();
        let p2 = len.next_power_of_two();

        let mut tree = Vec::with_capacity(p2 * 2);
        tree.resize_with(p2, T::default);
        tree.append(&mut vec);
        tree.resize_with(p2 * 2, O::operator_identity);
        for u in (1..p2).rev() {
            tree[u] = O::operator(&tree[u<<1], &tree[u<<1|1])
        }

        let ptl = if O::LAZY {p2} else {0};
        let mut pending = Vec::with_capacity(ptl);
        pending.resize_with(ptl, O::update_identity);

        Self { len, tree, pending, phantom: PhantomData }
    }
}

impl<T, U, O: MonoidOps<T, U>> FromIterator<T> for SegTree<T, U, O>
where
    T: Default,
    U: Default
{
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        Self::from(iter.into_iter().collect::<Vec<T>>())
    }
}

impl<T, U, O: MonoidOps<T, U>> SegTree<T, U, O>
where
    T: Default,
    U: Default
{
    pub fn new(len: usize) -> Self
    {
        let mut vec = Vec::with_capacity(len);
        vec.resize_with(len, O::operator_identity);
        Self::from(vec)
    }

    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    fn width(&self) -> usize {
        self.tree.len() / 2
    }

    #[inline]
    fn height(&self) -> u32 {
        self.width().bit_length()
    }

    /// The range of indexes of the leaf nodes descendant of tree node `u`
    #[inline]
    fn span(&self, u: usize) -> (usize, usize)
    {
        let s = self.height() - u.bit_length();
        ((u<<s) - self.width(), ((u+1)<<s) - self.width())
    }

    /// Apply any pending update at `u` to its two immediate children, then compose the update into those children in the pending tree
    #[inline]
    fn push1(&mut self, u: usize)
    {
        let a = mem::replace(&mut self.pending[u], O::update_identity());
        self.subtree_update(u<<1  , &a);
        self.subtree_update(u<<1|1, &a);
    }

    /// Recalculate tree node `u` by applying `operator` to its two immediate children. In a lazy tree, any pending updates at `u` or its ancestors must be pushed down before this call
    #[inline]
    fn build1(&mut self, u: usize) {
        self.tree[u] = O::operator(&self.tree[u<<1], &self.tree[u<<1|1]);
    }

    /// Push pending updates from root to tree node `u`
    #[inline]
    fn push_path(&mut self, u: usize)
    {
        for s in (1..u.bit_length()).rev() {
            self.push1(u >> s);
        }
    }

    /// Recalculate tree nodes from the parent of `u` to root. In a lazy tree, any pending updates at ancestors of `u` must be pushed down before this call
    #[inline]
    fn build_path(&mut self, mut u: usize)
    {
        while u > 1 {
            u >>= 1;
            self.build1(u);
        }
    }

    /// In a non-lazy tree, update leaf node `u`
    #[inline]
    fn leaf_update(&mut self, u: usize, a: &U) {
        self.tree[u] = O::update(&self.tree[u], a);
    }

    /// In a lazy tree, attempt to update tree node `u` as if all its descendant leaves had been updated with `a`, without actually traversing down to them. Then, insert the update into the pending tree at index `u`. Any pending updates at ancestors of `u` must be pushed down before this call
    #[inline]
    fn subtree_update(&mut self, u: usize, a: &U)
    {
        if let Some(x) = O::update_distributive(self.span(u), &self.tree[u], a) {
            self.tree[u] = x;
            if u < self.width() {
                self.pending[u] = O::update_composition(&self.pending[u], a);
            }
        } else {
            // update both children instead
            // maintain invariants for `u` and its descendants
            debug_assert!(u < self.width(), "update_distributive() must not fail on a leaf node");
            let a0 = mem::replace(&mut self.pending[u], O::update_identity());
            let a = O::update_composition(&a0, a);
            self.subtree_update(u<<1  , &a);
            self.subtree_update(u<<1|1, &a);
            self.build1(u);
        }
    }

    pub fn set(&mut self, index: usize, value: T)
    {
        let i = index;
        let u = self.width() + i;
        debug_assert!(i < self.len, "Index out of range");

        if O::LAZY { self.push_path(u); }
        self.tree[u] = value;
        self.build_path(u);
    }

    pub fn update(&mut self, range: impl Rangelike<usize>, value: &U)
    {
        let Range{start: i, end: j} = range.clamp(0..self.len).expect("Invalid update range");

        match O::LAZY {
            false => {
                debug_assert_eq!(j-i, 1, "Non-lazy trees do not support range updates");

                let u = self.width() + i;
                self.leaf_update(u, value);
                self.build_path(u);
            }
            true => {
                let mut u = self.width() + i;
                let mut v = self.width() + j-1;
                u >>= u.trailing_zeros();
                v >>= v.trailing_ones();

                self.push_path(u);
                self.push_path(v);

                let (mut l, mut r) = (self.width() + i, self.width() + j-1);
                while l <= r {
                    if l&1 == 1 {
                        self.subtree_update(l, value);
                        l += 1;
                    }
                    if r&1 == 0 {
                        self.subtree_update(r, value);
                        r -= 1;
                    }
                    l >>= 1;
                    r >>= 1;
                }

                self.build_path(u);
                self.build_path(v);
            }
        }
    }

    pub fn query(&mut self, range: impl Rangelike<usize>) -> T
    {
        let Range{start: i, end: j} = range.clamp(0..self.len).expect("Invalid query range");

        if O::LAZY {
            let mut u = self.width() + i;
            let mut v = self.width() + j-1;
            u >>= u.trailing_zeros();
            v >>= v.trailing_ones();

            self.push_path(u);
            self.push_path(v);
        }

        let (mut l, mut r) = (self.width() + i, self.width() + j-1);
        let (mut x, mut y) = (O::operator_identity(), O::operator_identity());
        while l <= r {
            if l&1 == 1 {
                x = O::operator(&x, &self.tree[l]);
                l += 1;
            }
            if r&1 == 0 {
                y = O::operator(&self.tree[r], &y);
                r -= 1;
            }
            l >>= 1;
            r >>= 1;
        }
        O::operator(&x, &y)
    }
}

impl<T, U, O: MonoidOps<T, U>> Debug for SegTree<T, U, O>
where
    T: Default + Debug,
    U: Default + Debug
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        for i in 0..self.height() {
            let (u, v) = (1<<i, 2<<i);

            write!(f, "\n{:?}", &self.tree[u..v])?;
            if O::LAZY && i < self.height() - 1 {
                write!(f, "\n{:?}", &self.pending[u..v])?;
            }
        }
        Ok(())
    }
}
