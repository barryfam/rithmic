use std::borrow::Borrow;
use std::iter::FusedIterator;
use std::ops::{Index, RangeBounds};

use delegate::delegate;

use crate::Rangelike;

use super::OrderTreap;

pub struct SortedList<T: Ord>(pub OrderTreap<T, ()>);

impl<T: Ord> SortedList<T> {
    pub fn new() -> Self {
        Self(OrderTreap::new())
    }

    delegate! {
    to self.0 {
        pub fn len(&self) -> usize;
        pub fn is_empty(&self) -> bool;

        pub fn insert(&mut self, value: T, [()]) -> usize;
        pub fn insert_left(&mut self, value: T, [()]) -> usize;
        pub fn remove_at(&mut self, range: impl Rangelike<usize>) -> usize;

        pub fn remove_all<Q>(&mut self, value: &Q) -> usize
        where T: Borrow<Q>, Q: Ord;

        pub fn remove_range<Q>(&mut self, range: impl RangeBounds<Q>) -> usize
        where T: Borrow<Q>, Q: Ord;

        pub fn contains<Q>(&mut self, value: &Q) -> bool
        where T: Borrow<Q>, Q: Ord;

        pub fn count<Q>(&mut self, value: &Q) -> usize
        where T: Borrow<Q>, Q: Ord;

        pub fn position<Q>(&mut self, value: &Q) -> Option<usize>
        where T: Borrow<Q>, Q: Ord;

        pub fn trisect<Q>(&mut self, value: &Q) -> (usize, usize)
        where T: Borrow<Q>, Q: Ord;

        pub fn trisect_range<Q>(&mut self, range: impl RangeBounds<Q>) -> (usize, usize)
        where T: Borrow<Q>, Q: Ord;

        pub fn range_len<Q>(&mut self, range: impl RangeBounds<Q>) -> usize
        where T: Borrow<Q>, Q: Ord;
    }}

    pub fn remove<Q>(&mut self, value: &Q) -> bool
    where
        T: Borrow<Q>,
        Q: Ord
    {
        self.0.remove(value).is_some()
    }

    pub fn range<'me:'i, 'r:'i, 'i, Q:'r>(&'me mut self, range: impl RangeBounds<Q> + Clone + 'r)
        -> impl Iterator<Item=&T> + DoubleEndedIterator + ExactSizeIterator + FusedIterator + 'i
    where
        T: Borrow<Q>,
        Q: Ord
    {
        self.0.range(range).map(|(k, _)| k)
    }

    pub fn iter(&self)
        -> impl Iterator<Item=&T> + DoubleEndedIterator + ExactSizeIterator + FusedIterator + '_
    {
        self.0.iter().map(|(k, _)| k)
    }
}

impl<T: Ord> Default for SortedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FromIterator<T> for SortedList<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self
    {
        let mut sl = Self::new();
        for value in iter {
            sl.insert(value);
        }
        sl
    }
}

impl<T: Ord> Index<usize> for SortedList<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.get_at(index).0
    }
}
