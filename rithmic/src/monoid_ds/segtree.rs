use std::fmt::{self, Debug, Formatter};
use std::marker::PhantomData;
use std::{mem, iter};

use crate::{IntBitOps, Rangelike};

use super::monoid_ops::{MonoidOps, USumQSum};

#[derive(Default, Clone)]
pub struct SegTree<T, U = T, O: MonoidOps<T, U> = USumQSum>
where
    T: Default,
    U: Default
{
    len: usize,
    tree: Vec<(T, U)>,
    phantom: PhantomData<O>,
}

impl<T, U, O: MonoidOps<T, U>> From<Vec<T>> for SegTree<T, U, O>
where
    T: Default,
    U: Default
{
    fn from(vec: Vec<T>) -> Self
    {
        let len = vec.len();
        let p2 = len.next_power_of_two();

        let mut tree = Vec::with_capacity(p2 * 2);
        tree.resize_with(p2, Default::default);
        tree.extend(vec.into_iter().zip(iter::repeat_with(O::update_identity)));
        tree.resize_with(p2 * 2, || (O::operator_identity(), O::update_identity()));
        for u in (1..p2).rev() {
            tree[u] = (
                O::operator(&tree[u<<1].0, &tree[u<<1|1].0),
                O::update_identity()
            );
        }

        Self { len, tree, phantom: PhantomData }
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
        let a = mem::replace(&mut self.tree[u].1, O::update_identity());
        self.subtree_update(u<<1  , &a);
        self.subtree_update(u<<1|1, &a);
    }

    /// Recalculate tree node `u` by applying `operator` to its two immediate children. In a lazy tree, any pending updates at `u` or its ancestors must be pushed down before this call
    #[inline]
    fn build1(&mut self, u: usize) {
        self.tree[u].0 = O::operator(&self.tree[u<<1].0, &self.tree[u<<1|1].0);
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
        self.tree[u].0 = O::update(&self.tree[u].0, a);
    }

    /// In a lazy tree, attempt to update tree node `u` as if all its descendant leaves had been updated with `a`, without actually traversing down to them. Then, insert the update into the pending tree at index `u`. Any pending updates at ancestors of `u` must be pushed down before this call
    #[inline]
    fn subtree_update(&mut self, u: usize, a: &U)
    {
        if let Some(x) = O::update_distributive(self.span(u), &self.tree[u].0, a) {
            self.tree[u].0 = x;
            if u < self.width() {
                self.tree[u].1 = O::update_composition(&self.tree[u].1, a);
            }
        } else {
            // update both children instead
            // maintain invariants for `u` and its descendants
            debug_assert!(u < self.width(), "update_distributive() must not fail on a leaf node");
            let a0 = mem::replace(&mut self.tree[u].1, O::update_identity());
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
        debug_assert!(i < self.len, "index out of range");

        if O::LAZY { self.push_path(u); }
        self.tree[u].0 = value;
        self.build_path(u);
    }

    pub fn update(&mut self, range: impl Rangelike<usize>, value: &U)
    {
        let (i, j) = range.clamp(0..self.len).expect("update range outside bounds");

        match O::LAZY {
            false => {
                debug_assert_eq!(j-i, 1, "non-lazy trees do not support range updates");

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
        let (i, j) = range.clamp(0..self.len).expect("query range outside bounds");
        if i == j {
            return O::operator_identity()
        }

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
                x = O::operator(&x, &self.tree[l].0);
                l += 1;
            }
            if r&1 == 0 {
                y = O::operator(&self.tree[r].0, &y);
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
            let (row, pending): (Vec<_>, Vec<_>) = self.tree[u..v].iter().map(|(x, a)| (x, a)).unzip();

            write!(f, "\n{:?}", row)?;
            if O::LAZY && i < self.height() - 1 {
                write!(f, "\n{:?}", pending)?;
            }
        }
        Ok(())
    }
}
