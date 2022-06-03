mod list;
mod sorted_list;

#[cfg(test)] mod tests;

pub use list::List;
pub use sorted_list::SortedList;

use std::borrow::Borrow;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::{RangeBounds, self};

use delegate::delegate;

use crate::Rangelike;

use super::{AugFn, AugTreap, Node, OptNode};

#[derive(Clone)]
pub struct OrderAugFn<K, V> {
    phantom: PhantomData<(K, V)>
}
impl<K, V> AugFn<K, V, usize> for OrderAugFn<K, V> {
    #[inline]
    fn f(u: &Node<K, V, usize>) -> usize
    {
        1 + u.left_aug().unwrap_or(&0) + u.right_aug().unwrap_or(&0)
    }
}

#[derive(Clone)]
pub struct OrderTreap<K, V>(pub AugTreap<K, V, usize, OrderAugFn<K, V>>);

impl<K: Ord, V> OrderTreap<K, V>
{
    pub fn new() -> Self {
        Self(AugTreap::new())
    }

    delegate! {
    to self.0 {
        pub fn is_empty(&self) -> bool;
        pub fn aug_fn(&self, u: &Node<K, V, usize>) -> usize;

        pub fn contains<Q>(&mut self, key: &Q) -> bool
        where K: Borrow<Q>, Q: Ord;

        pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
        where K: Borrow<Q>, Q: Ord;
    }}

    #[inline]
    pub fn len(&self) -> usize {
        self.0.root.as_ref().map_or(0, |r| r.aug)
    }

    pub fn insert(&mut self, key: K, value: V) -> usize
    {
        let r = self.0.root.take();
        let (u, w) = self.0.split_by(r, |k| *k <= key);
        let mut v = Some(box Node::new(key, value));

        let i = u.as_ref().map_or(0, |u| u.aug);

        v.as_mut().map(|mut v| v.aug = OrderAugFn::f(v));
        self.0.root = self.0.join3(u, v, w);
        i
    }

    pub fn insert_left(&mut self, key: K, value: V) -> usize
    {
        let r = self.0.root.take();
        let (u, w) = self.0.split_by(r, |k| *k < key);
        let mut v = Some(box Node::new(key, value));

        let i = u.as_ref().map_or(0, |u| u.aug);

        v.as_mut().map(|mut v| v.aug = OrderAugFn::f(v));
        self.0.root = self.0.join3(u, v, w);
        i
    }

    pub fn remove_range<Q>(&mut self, range: impl RangeBounds<Q>) -> usize
    where
        K: Borrow<Q>,
        Q: Ord
    {
        let r = self.0.root.take();
        let (u, v, w) = self.0.split3_by_range(r, range);

        self.0.root = self.0.join(u, w);
        v.as_ref().map_or(0, |v| v.aug)
    }

    pub fn remove_all<Q>(&mut self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Ord
    {
        self.remove_range(key..=key)
    }

    pub fn get_at(&self, index: usize) -> (&K, &V)
    {
        let mut i = index;
        let mut v = &self.0.root;
        while let Some(u) = v {
            let left = u.left_aug().unwrap_or(&0);

            use std::cmp::Ordering::*;
            match left.cmp(&i) {
                Equal => {
                    return (&u.key, &u.value)
                }
                Greater => {
                    v = &u.left;
                }
                Less => {
                    i -= left + 1;
                    v = &u.right;
                }
            }
        }
        panic!("insertion index (is {index}) should be <= len (is {})", self.len())
    }

    pub fn position<Q>(&mut self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Ord
    {
        let (i, j) = self.trisect(key);
        (i < j).then_some(i)
    }

    pub fn count<Q>(&mut self, key: &Q) -> usize
    where
        K: Borrow<Q>,
        Q: Ord
    {
        let (i, j) = self.trisect(key);
        j - i
    }

    pub fn trisect_range<Q>(&mut self, range: impl RangeBounds<Q>) -> (usize, usize)
    where
        K: Borrow<Q>,
        Q: Ord
    {
        let r = self.0.root.take();

        let (u, v, w) = self.0.split3_by_range(r, range);

        let i = u.as_ref().map_or(0, |u| u.aug);
        let j = v.as_ref().map_or(0, |v| v.aug) + i;

        self.0.root = self.0.join3(u, v, w);
        (i, j)
    }

    pub fn trisect<Q>(&mut self, key: &Q) -> (usize, usize)
    where
        K: Borrow<Q>,
        Q: Ord
    {
        self.trisect_range(key..=key)
    }

    pub fn range_len<Q>(&mut self, range: impl RangeBounds<Q>) -> usize
    where
        K: Borrow<Q>,
        Q: Ord
    {
        let (i, j) = self.trisect_range(range);
        j - i
    }

    pub fn range<Q>(&mut self, range: impl RangeBounds<Q> + Clone) -> self::Range<K, V>
    where
        K: Borrow<Q>,
        Q: Ord
    {
        let size = self.range_len(range.clone());
        self::Range { inner: self.0.range(range), size }
    }

    pub fn split_at(&mut self, root: OptNode<K, V, usize>, index: usize)
        -> (OptNode<K, V, usize>, OptNode<K, V, usize>)
    {
        let Some(mut r) = root else { return (None, None) };

        let left = *r.left_aug().unwrap_or(&0);
        if index > left {
            let (u, v) = self.split_at(r.right, index - left - 1);
            r.right = u;
            r.aug = OrderAugFn::f(&r);
            (Some(r), v)
        }
        else {
            let (u, v) = self.split_at(r.left, index);
            r.left = v;
            r.aug = OrderAugFn::f(&r);
            (u, Some(r))
        }
    }

    pub fn split3_at(&mut self, root: OptNode<K, V, usize>, i: usize, j: usize)
        -> (OptNode<K, V, usize>, OptNode<K, V, usize>, OptNode<K, V, usize>)
    {
        let (u, vw) = self.split_at(root, i);
        let (v, w) = self.split_at(vw, j-i);
        (u, v, w)
    }

    pub fn remove_at(&mut self, range: impl Rangelike<usize>) -> usize
    {
        let ops::Range{start: i, end: j} = range.clamp(0..self.len()).expect("range must be within current length");

        let r = self.0.root.take();
        let (u, v, w) = self.split3_at(r, i, j);

        self.0.root = self.0.join(u, w);
        v.as_ref().map_or(0, |v| v.aug)
    }

    pub fn iter(&self) -> self::Range<K, V>
    {
        self::Range {
            inner: self.0.range(..),
            size: self.len()
        }
    }
}

impl<K: Ord, V> Default for OrderTreap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for OrderTreap<K, V> {
    fn from_iter<I: IntoIterator<Item=(K, V)>>(iter: I) -> Self
    {
        let mut ot = Self::new();
        for (k, v) in iter {
            ot.insert(k, v);
        }
        ot
    }
}

pub struct Range<'n, K, V> {
    inner: super::Range<'n, K, V, usize>,
    size: usize,
}
impl<'n, K, V> Iterator for Range<'n, K, V> {
    type Item = (&'n K, &'n V);

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let kv = self.inner.next()?;
        self.size -= 1;
        Some(kv)
    }
}
impl<'n, K, V> DoubleEndedIterator for Range<'n, K, V> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let kv = self.inner.next_back()?;
        self.size -= 1;
        Some(kv)
    }
}
impl<'n, K, V> ExactSizeIterator for Range<'n, K, V> {}
impl<'n, K, V> FusedIterator for Range<'n, K, V> {}
