mod matrix_mul;
mod matrix;

#[cfg(test)] mod tests;

use std::fmt::{self, Debug, Formatter, Display};
use std::ops::{Index, IndexMut};
use std::slice;

use delegate::delegate;

use itertools::Itertools;

#[inline]
pub(crate) fn size_of<const D: usize>(shape: [usize; D]) -> usize {
    shape.into_iter().product()
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NdVec<const D: usize, T> {
    pub(crate) shape: [usize; D],
    pub(crate) vec: Vec<T>
}

impl<const D: usize, T> NdVec<D, T>
{
    pub fn new(shape: [usize; D]) -> Self
    where T: Default
    {
        let size = size_of(shape);
        let mut vec = Vec::<T>::with_capacity(size);
        vec.resize_with(size, T::default);
        Self { shape, vec }
    }

    pub fn full(shape: [usize; D], fill_value: T) -> Self
    where T: Clone
    {
        Self { shape, vec: vec![fill_value; size_of(shape)] }
    }

    pub fn from_raw(shape: [usize; D], raw: Vec<T>) -> Self
    {
        debug_assert_eq!(size_of(shape), raw.len(), "shape must match Vec len");
        Self { shape, vec: raw }
    }

    delegate! {
    to self.vec {
        #[call(len)]
        pub fn size(&self) -> usize;

        pub fn fill(&mut self, value: T) where T: Clone;
        pub fn fill_with<F>(&mut self, f: F) where F: FnMut() -> T;
        pub fn first(&self) -> Option<&T>;
        pub fn last(&self) -> Option<&T>;
        pub fn iter(&self) -> slice::Iter<T>;
        pub fn iter_mut(&mut self) -> slice::IterMut<T>;
    }}

    #[inline]
    pub fn shape(&self) -> [usize; D] {
        self.shape
    }

    #[inline]
    pub fn get(&self, index: [usize; D]) -> Option<&T> {
        self.inbounds(index).then( ||
        {
            #[cfg(not(feature = "unsafe"))]
            { &self.vec[self.ravel(index)] }
            #[cfg(feature = "unsafe")]
            // SAFETY: self.inbounds(index) implies self.ravel(index) < self.vec.len
            unsafe { self.vec.get_unchecked(self.ravel(index)) }
        })
    }

    #[inline]
    pub fn get_mut(&mut self, index: [usize; D]) -> Option<&mut T> {
        self.inbounds(index).then(||
        {
            let i = self.ravel(index);
            #[cfg(not(feature = "unsafe"))]
            { &mut self.vec[i] }
            #[cfg(feature = "unsafe")]
            // SAFETY: self.inbounds(index) implies self.ravel(index) < self.vec.len
            unsafe { self.vec.get_unchecked_mut(i) }
        })
    }

    #[cfg(feature = "unsafe")]
    #[inline]
    pub unsafe fn get_unchecked(&self, index: [usize; D]) -> &T {
        self.vec.get_unchecked(self.ravel(index))
    }

    #[cfg(feature = "unsafe")]
    #[inline]
    pub unsafe fn get_unchecked_mut(&mut self, index: [usize; D]) -> &mut T {
        let i = self.ravel(index);
        self.vec.get_unchecked_mut(i)
    }

    #[inline]
    pub(crate) fn inbounds(&self, index: [usize; D]) -> bool {
        index.iter().zip(self.shape.iter())
            .all(|(i, n)| i < n)
    }

    #[inline]
    fn ravel(&self, index: impl IntoIterator<Item=usize>) -> usize
    {
        let index = index.into_iter();
        debug_assert_eq!(index.size_hint(), (D, Some(D)));

        let mut i = 0;
        for (ii, &n) in index.zip(self.shape.iter()) {
            i *= n;
            i += ii;
        }
        i
    }
}

impl<const D: usize, T> Index<[usize; D]> for NdVec<D, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: [usize; D]) -> &Self::Output
    {
        debug_assert!(self.inbounds(index), "Index outside of shape");
        &self.vec[self.ravel(index)]
    }
}

impl<const D: usize, T> IndexMut<[usize; D]> for NdVec<D, T>
{
    #[inline]
    fn index_mut(&mut self, index: [usize; D]) -> &mut Self::Output
    {
        debug_assert!(self.inbounds(index), "Index outside of shape");
        let i = self.ravel(index);
        &mut self.vec[i]
    }
}

impl<const D: usize, T> Display for NdVec<D, T>
where T: Display
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let row = self.shape[D-1];
        for i in (0..self.size()).step_by(row) {
            writeln!(f, "{}", self.vec[i..i+row].iter().join(" "))?;
        }
        Ok(())
    }
}

impl<const D: usize, T> Debug for NdVec<D, T>
where T: Debug
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        let row = self.shape[D-1];
        for i in (0..self.size()).step_by(row) {
            if D > 2 && i > 0  {
                let mut r = i / row;
                for ii in (1..D-1).rev() {
                    let d = self.shape[ii];
                    if r % d != 0 {
                        break
                    }
                    r /= d;
                    if ii == D-2 { writeln!(f)?; }
                    write!(f, "-")?;
                }
            }
            write!(f, "\n{:?}", &self.vec[i..i+row])?;
        }
        Ok(())
    }
}
