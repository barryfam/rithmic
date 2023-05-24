pub mod convex_hull_trick;
pub mod finger;
pub mod order_treap;

use std::borrow::Borrow;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::ops::{RangeBounds, Bound};
use std::{ptr, mem};

use crate::OptionMerge;

use finger::Finger;

#[derive(Clone)]
pub struct Node<K, V, A> {
    pub key: K,
    pub value: V,
    pub aug: A,
    pub left: OptNode<K, V, A>,
    pub right: OptNode<K, V, A>,
    heap_key: usize,
}
pub type OptNode<K, V, A> = Option<Box<Node<K, V, A>>>;

impl<K, V, A> Node<K, V, A>
{
    /// Create a new node
    ///
    /// Note, the augment function is not in scope here and therefore not called; it must be called afterwards to maintain invariants
    pub fn new(key: K, value: V) -> Self
    where A: Default
    {
        Self {
            key, value,
            aug: A::default(),
            heap_key: rand::random(),
            left: None,
            right: None
        }
    }

    #[inline]
    pub fn left_key(&self) -> Option<&K> {
        self.left.as_ref().map(|l| &l.key)
    }

    #[inline]
    pub fn right_key(&self) -> Option<&K> {
        self.right.as_ref().map(|r| &r.key)
    }

    #[inline]
    pub fn left_value(&self) -> Option<&V> {
        self.left.as_ref().map(|l| &l.value)
    }

    #[inline]
    pub fn right_value(&self) -> Option<&V> {
        self.right.as_ref().map(|r| &r.value)
    }

    #[inline]
    pub fn left_aug(&self) -> Option<&A> {
        self.left.as_ref().map(|l| &l.aug)
    }

    #[inline]
    pub fn right_aug(&self) -> Option<&A> {
        self.right.as_ref().map(|r| &r.aug)
    }
}

pub trait AugFn<K, V, A> {
    fn f(u: &Node<K, V, A>) -> A;
}

#[derive(Clone)]
pub struct AugTreap<K, V, A, F> {
    pub root: OptNode<K, V, A>,
    phantom: PhantomData<F>
}

impl<K, V, A, F> AugTreap<K, V, A, F>
where
    K: Ord,
    A: Default,
    F: AugFn<K, V, A>
{
    pub fn new() -> Self {
        Self { root: None, phantom: PhantomData }
    }

    #[inline]
    pub fn aug_fn(&self, u: &Node<K, V, A>) -> A {
        F::f(u)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    pub fn contains<Q>(&mut self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord
    {
        let r = self.root.take();
        let (u, v, w) = self.split3_by_key(r, key);

        let v_is = v.is_some();
        self.root = self.join3(u, v, w);
        v_is
    }

    pub fn insert(&mut self, key: K, value: V)
    {
        let r = self.root.take();
        let (u, w) = self.split_by(r, |k| *k <= key);

        let mut v = Some(Box::new(Node::new(key, value)));
        v.as_mut().map(|v| v.aug = F::f(v));

        self.root = self.join3(u, v, w);
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord
    {
        let r = self.root.take();
        let (u, mut v, w) = self.split3_by_key(r, key);

        let v0 = self.join(
            v.as_mut().and_then(|v| v.left.take()),
            v.as_mut().and_then(|v| v.right.take())
        );
        self.root = self.join3(u, v0, w);
        v.map(|v| v.value)
    }

    pub fn remove_range<Q>(&mut self, range: impl RangeBounds<Q>) -> bool
    where
        K: Borrow<Q>,
        Q: Ord
    {
        let r = self.root.take();
        let (u, v, w) = self.split3_by_range(r, range);

        self.root = self.join(u, w);
        v.is_some()
    }

    pub fn remove_all<Q>(&mut self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord
    {
        self.remove_range(key..=key)
    }

    pub fn split_by<P>(&self, root: OptNode<K, V, A>, mut predicate: P) -> (OptNode<K, V, A>, OptNode<K, V, A>)
    where P: FnMut(&K) -> bool
    {
        let Some(mut r) = root else { return (None, None) };

        if predicate(&r.key) {
            let (u, v) = self.split_by(r.right, predicate);
            r.right = u;
            r.aug = F::f(&r);
            (Some(r), v)
        }
        else {
            let (u, v) = self.split_by(r.left, predicate);
            r.left = v;
            r.aug = F::f(&r);
            (u, Some(r))
        }
    }

    pub fn split3_by<P0, P1>(&self, root: OptNode<K, V, A>, predicate0: P0, predicate1: P1)
        -> (OptNode<K, V, A>, OptNode<K, V, A>, OptNode<K, V, A>)
    where
        P0: FnMut(&K) -> bool,
        P1: FnMut(&K) -> bool
    {
        let (u, vw) = self.split_by(root, predicate0);
        let (v, w) = self.split_by(vw, predicate1);
        (u, v, w)
    }

    pub fn split3_by_range<Q>(&self, root: OptNode<K, V, A>, range: impl RangeBounds<Q>)
        -> (OptNode<K, V, A>, OptNode<K, V, A>, OptNode<K, V, A>)
    where
        K: Borrow<Q>,
        Q: Ord
    {
        self.split3_by(
            root,
            |k| match range.start_bound() {
                Bound::Included(start) => k.borrow() < start,
                Bound::Excluded(start) => k.borrow() <= start,
                Bound::Unbounded => false,
            },
            |k| match range.end_bound() {
                Bound::Included(end) => k.borrow() <= end,
                Bound::Excluded(end) => k.borrow() < end,
                Bound::Unbounded => true,
            }
        )
    }

    pub fn split3_by_key<Q>(&self, root: OptNode<K, V, A>, key: &Q)
        -> (OptNode<K, V, A>, OptNode<K, V, A>, OptNode<K, V, A>)
    where
        K: Borrow<Q>,
        Q: Ord
    {
        self.split3_by_range(root, key..=key)
    }

    pub fn join(&self, left: OptNode<K, V, A>, right: OptNode<K, V, A>) -> OptNode<K, V, A>
    {
        left.merge(right, |mut left, mut right| {
            if left.heap_key < right.heap_key {
                left.right = self.join(left.right, Some(right));
                left.aug = F::f(&left);
                left
            }
            else {
                right.left = self.join(Some(left), right.left);
                right.aug = F::f(&right);
                right
            }
        })
    }

    pub fn join3(&self, left: OptNode<K, V, A>, mid: OptNode<K, V, A>, right: OptNode<K, V, A>)
        -> OptNode<K, V, A>
    {
        self.join(self.join(left, mid), right)
    }

    pub fn range<Q>(&self, range: impl RangeBounds<Q>) -> self::Range<K, V, A>
    where
        K: Borrow<Q>,
        Q: Ord
    {
        let mut fingers = [vec![], vec![]];

        if let Some(ref r) = self.root {
            r.finger_first_false(
                |u| match range.start_bound() {
                    Bound::Included(start) => u.key.borrow() < start,
                    Bound::Excluded(start) => u.key.borrow() <= start,
                    Bound::Unbounded => false,
                },
                &mut fingers[0]
            );
            r.finger_last_true(
                |u| match range.end_bound() {
                    Bound::Included(end) => u.key.borrow() <= end,
                    Bound::Excluded(end) => u.key.borrow() < end,
                    Bound::Unbounded => true,
                },
                &mut fingers[1]
            );
        }

        let fused = fingers[0].is_empty() || fingers[1].is_empty()
            || fingers[0].last().unwrap().key > fingers[1].last().unwrap().key;

        self::Range { fingers, fused }
    }

    pub fn iter(&self) -> self::Range<K, V, A> {
        self.range(..)
    }

    pub fn into_first_last(&self, root: OptNode<K, V, A>) -> (OptNode<K, V, A>, OptNode<K, V, A>)
    {
        let Some(mut r) = root else { return (None, None) };
        let mut left = r.left.take();
        let mut right = r.right.take();

        if left.is_none() { left = Some(r); }
        else if right.is_none() { right = Some(r); }

        if right.is_none() {
            return (left, None)
        }

        let mut left = left.unwrap();
        let mut right = right.unwrap();

        while let Some(u) = left.left {
            left = u;
        }
        while let Some(u) = right.right {
            right = u;
        }

        left.right = None;
        right.left = None;

        (Some(left), Some(right))
    }
}

impl<K, V, A, F> Default for AugTreap<K, V, A, F>
where
    K: Ord,
    A: Default,
    F: AugFn<K, V, A>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V, A, F> FromIterator<(K, V)> for AugTreap<K, V, A, F>
where
    K: Ord,
    A: Default,
    F: AugFn<K, V, A>
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut at = Self::new();
        for (k, v) in iter {
            at.insert(k, v);
        }
        at
    }
}

pub struct Range<'n, K, V, A>
{
    fingers: [Vec<&'n Node<K, V, A>>; 2],
    fused: bool,
}
impl<'n, K, V, A> Iterator for Range<'n, K, V, A> {
    type Item = (&'n K, &'n V);

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.fused { return None }

        let u = self.fingers[0].tip();

        if ptr::eq(self.fingers[0].tip(), self.fingers[1].tip()) {
            self.fused = true;
        } else {
            self.fingers[0] = mem::take(&mut self.fingers[0]).successor();
        }

        Some((&u.key, &u.value))
    }
}
impl<'n, K, V, A> DoubleEndedIterator for Range<'n, K, V, A>
{
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item>
    {
        if self.fused { return None }

        let u = self.fingers[1].tip();

        if ptr::eq(self.fingers[0].tip(), self.fingers[1].tip()) {
            self.fused = true;
        } else {
            self.fingers[1] = mem::take(&mut self.fingers[1]).predecessor();
        }

        Some((&u.key, &u.value))
    }
}
impl<'n, K, V, A> FusedIterator for Range<'n, K, V, A> {}
