use std::iter::FusedIterator;
use std::ops::Index;

use delegate::delegate;

use crate::Rangelike;

use super::OrderTreap;

pub struct List<T: Ord>(pub OrderTreap<(), T>);

impl<T: Ord> List<T> {
    pub fn new() -> Self {
        Self(OrderTreap::new())
    }

    delegate! {
    to self.0 {
        pub fn len(&self) -> usize;
        pub fn is_empty(&self) -> bool;

        #[call(insert)]
        pub fn push_back(&mut self, [()], value: T) -> usize;

        #[call(insert_left)]
        pub fn push_front(&mut self, [()], value: T) -> usize;

        #[call(remove_at)]
        pub fn remove(&mut self, range: impl Rangelike<usize>) -> usize;
    }}

    pub fn insert(&mut self, index: usize, value: T) {
        self.insert_slice(index, [value])
    }

    pub fn insert_slice(&mut self, index: usize, slice: impl IntoIterator<Item=T>)
    {
        assert!(index <= self.len(), "insertion index (is {index}) should be <= len (is {})", self.len());

        let r = self.0.0.root.take();
        let (u, w) = self.0.split_at(r, index);
        let v = Self::from_iter(slice).0.0.root;

        self.0.0.root = self.0.0.join3(u, v, w);
    }

    pub fn replace(&mut self, range: impl Rangelike<usize>, value: T) -> usize {
        self.replace_slice(range, [value])
    }

    pub fn replace_slice(&mut self, range: impl Rangelike<usize>, slice: impl IntoIterator<Item=T>) -> usize
    {
        let (i, j) = range.clamp(0..self.len()).expect("range must be within current length");

        let r = self.0.0.root.take();
        let (u, v0, w) = self.0.split3_at(r, i, j);
        let v = Self::from_iter(slice).0.0.root;

        self.0.0.root = self.0.0.join3(u, v, w);
        v0.as_ref().map_or(0, |v0| v0.aug)
    }

    pub fn iter(&self)
        -> impl Iterator<Item=&T> + DoubleEndedIterator + ExactSizeIterator + FusedIterator + '_
    {
        self.0.iter().map(|(_, v)| v)
    }
}

impl<T: Ord> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self
    {
        let mut list = Self::new();
        for value in iter {
            list.push_back(value);
        }
        list
    }
}

impl<T: Ord> Index<usize> for List<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.0.get_at(index).1
    }
}
