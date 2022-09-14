use std::fmt::Debug;
use std::ops::{Add, Sub, Mul};

use crate::{NdVec, OdometerBE};

use super::super::super::monoid_ops::MonoidOps;
use super::super::NdFenwick;

impl<T, O: MonoidOps<T, T>> NdFenwick<1, T, O>
where T: Default
{
    pub fn update(&mut self, index: [usize; 1], value: T)
    {
        assert!(self.inbounds(index), "index outside of shape: {:?} / {:?}", index, self.shape());

        let mut i0 = index[0];
        while i0 < self.shape()[0] {
            let index = [i0];

            let x;
            #[cfg(not(feature = "unsafe"))]
            { x = &mut self.0[index]; }
            #[cfg(feature = "unsafe")]
            // SAFETY: each i < shape
            unsafe { x = self.0.get_unchecked_mut(index); }
            *x = O::operator(x, &value);

            i0 = i0 | (i0+1);
        }
    }
}

impl<T> NdFenwick<1, T>
where
    T: Default + Copy + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    for<'a> &'a T: Add<Output=T> + Sub<Output=T> + Mul<Output=T>,
{
    pub fn build_from(ndvec: &NdVec<1, T>) -> Self {
        let mut ft = Self::new(ndvec.shape());
        for u in OdometerBE::new(ndvec.shape()) {
            ft.update(u, ndvec[u]);
        }
        ft
    }
}
