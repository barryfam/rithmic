use std::fmt::Debug;
use std::ops::{Add, Sub, Mul};

use crate::{NdVec, OdometerBE};

use super::super::super::monoid_ops::MonoidOps;
use super::super::NdFenwick;

impl<T, O: MonoidOps<T, T>> NdFenwick<4, T, O>
where T: Default
{
    pub fn update(&mut self, index: [usize; 4], value: T)
    {
        assert!(self.inbounds(index), "index outside of shape: {:?} / {:?}", index, self.shape());

        let mut i3 = index[3];
        while i3 < self.shape()[3] {
            let mut i2 = index[2];
            while i2 < self.shape()[2] {
                let mut i1 = index[1];
                while i1 < self.shape()[1] {
                    let mut i0 = index[0];
                    while i0 < self.shape()[0] {
                        let index = [i0, i1, i2, i3];

                        let x;
                        #[cfg(not(feature = "unsafe"))]
                        { x = &mut self.0[index]; }
                        #[cfg(feature = "unsafe")]
                        // SAFETY: each i < shape
                        unsafe { x = self.0.get_unchecked_mut(index); }
                        *x = O::operator(x, &value);

                        i0 = i0 | (i0+1);
                    }
                    i1 = i1 | (i1+1);
                }
                i2 = i2 | (i2+1);
            }
            i3 = i3 | (i3+1);
        }
    }
}

impl<T> NdFenwick<4, T>
where
    T: Default + Copy + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    for<'a> &'a T: Add<Output=T> + Sub<Output=T> + Mul<Output=T>,
{
    pub fn build_from(ndvec: &NdVec<4, T>) -> Self {
        let mut ft = Self::new(ndvec.shape());
        for u in OdometerBE::new(ndvec.shape()) {
            ft.update(u, ndvec[u]);
        }
        ft
    }
}
