use std::fmt::Debug;
use std::ops::{Add, Sub, Mul};

use crate::{NdVec, OdometerBE};

use super::super::super::monoid_ops::MonoidOps;
use super::super::NdFenwick;

impl<T, O: MonoidOps<T, T>> NdFenwick<13, T, O>
where T: Default
{
    pub fn update(&mut self, index: [usize; 13], value: T)
    {
        assert!(self.inbounds(index), "index outside of shape: {:?} / {:?}", index, self.shape());

        let mut i12 = index[12];
        while i12 < self.shape()[12] {
            let mut i11 = index[11];
            while i11 < self.shape()[11] {
                let mut i10 = index[10];
                while i10 < self.shape()[10] {
                    let mut i9 = index[9];
                    while i9 < self.shape()[9] {
                        let mut i8 = index[8];
                        while i8 < self.shape()[8] {
                            let mut i7 = index[7];
                            while i7 < self.shape()[7] {
                                let mut i6 = index[6];
                                while i6 < self.shape()[6] {
                                    let mut i5 = index[5];
                                    while i5 < self.shape()[5] {
                                        let mut i4 = index[4];
                                        while i4 < self.shape()[4] {
                                            let mut i3 = index[3];
                                            while i3 < self.shape()[3] {
                                                let mut i2 = index[2];
                                                while i2 < self.shape()[2] {
                                                    let mut i1 = index[1];
                                                    while i1 < self.shape()[1] {
                                                        let mut i0 = index[0];
                                                        while i0 < self.shape()[0] {
                                                            let index = [i0, i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12];

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
                                            i4 = i4 | (i4+1);
                                        }
                                        i5 = i5 | (i5+1);
                                    }
                                    i6 = i6 | (i6+1);
                                }
                                i7 = i7 | (i7+1);
                            }
                            i8 = i8 | (i8+1);
                        }
                        i9 = i9 | (i9+1);
                    }
                    i10 = i10 | (i10+1);
                }
                i11 = i11 | (i11+1);
            }
            i12 = i12 | (i12+1);
        }
    }
}

impl<T> NdFenwick<13, T>
where
    T: Default + Copy + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    for<'a> &'a T: Add<Output=T> + Sub<Output=T> + Mul<Output=T>,
{
    pub fn build_from(ndvec: &NdVec<13, T>) -> Self {
        let mut ft = Self::new(ndvec.shape());
        for u in OdometerBE::new(ndvec.shape()) {
            ft.update(u, ndvec[u]);
        }
        ft
    }
}
