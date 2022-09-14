mod update_build;

#[cfg(test)] mod tests;

use std::fmt::{Debug, self};
use std::marker::PhantomData;

use delegate::delegate;

use crate::{IntBitOps, NdVec, ndvec, Rangelike, OdometerBE};
use super::monoid_ops::{MonoidOps, USumQSum};

#[derive(Clone)]
pub struct NdFenwick<const D: usize, T: Default, O: MonoidOps<T, T> = USumQSum>
    (NdVec<D, T>, PhantomData<O>);

impl<const D: usize, T, O: MonoidOps<T, T>> NdFenwick<D, T, O>
where T: Default
{
    pub fn new(shape: [usize; D]) -> Self
    {
        let size = ndvec::size_of(shape);
        let mut vec = Vec::<T>::with_capacity(size);
        vec.resize_with(size, O::operator_identity);
        Self(NdVec::from_raw(shape, vec), PhantomData)
    }

    delegate! {
    to self.0 {
        pub fn size(&self) -> usize;
        pub fn shape(&self) -> [usize; D];
        fn inbounds(&self, index: [usize; D]) -> bool;
    }}

    pub fn prefix(&self, end: [usize; D]) -> T
    {
        if end.iter().any(|&i| i == 0) {
            return O::operator_identity()
        }

        let u0 = end.map(|i| i-1);
        assert!(self.inbounds(u0), "range larger than shape: {:?}", end);

        #[inline]
        fn query_next(i: usize) -> Option<usize> {
            let i1 = (i & (i+1)).wrapping_sub(1);
            (i1 != !0).then_some(i1)
        }

        let mut u = u0;
        let mut sum = O::operator_identity();
        'odo: loop {
            #[cfg(not(feature = "unsafe"))]
            { sum = O::operator(&sum, &self.0[u]); }
            #[cfg(feature = "unsafe")]
            // SAFETY: Asserted u0 inbounds. 0 <= u[d] <= u0[d] for all d
            unsafe { sum = O::operator(&sum, self.0.get_unchecked(u)); }

            for d in (0..D).rev() {
                if let Some(next) = query_next(u[d]) {
                    u[d] = next;
                    continue 'odo
                }
                else {
                    u[d] = u0[d];
                }
            }
            break
        }
        sum
    }

    pub fn query(&self, range: [impl Rangelike<usize>; D]) -> T
    {
        let mut r = [(0, 0); D];
        for d in 0..D {
            (r[d].0, r[d].1) = range[d].clamp(0..self.shape()[d])
                .expect("invalid query range");
        }
        if r.iter().any(|&(start, end)| start == end) {
            return O::operator_identity()
        }
        if r.iter().all(|&(start, _end)| start == 0) {
            return self.prefix(r.map(|(_start, end)| end))
        }

        #[derive(Clone, Copy, Debug)]
        struct IE {
            excl: bool,
            i: usize,
        }
        let mut i_lists = [(); D].map(|_| vec![]);

        let mut odo = [0; D];
        let mut stride = 1;
        for d in (0..D).rev() {
            i_lists[d] = Vec::with_capacity(self.shape()[d].bit_length() as usize * 2 - 1);

            let mut i = r[d].1.wrapping_sub(1);
            let mut j = r[d].0.wrapping_sub(1);
            while (j as isize) < (i as isize) {
                i_lists[d].push(IE{excl: false, i: i * stride});
                i = (i & (i+1)).wrapping_sub(1);
            }
            while (i as isize) < (j as isize) {
                i_lists[d].push(IE{excl: true, i: j * stride});
                j = (j & (j+1)).wrapping_sub(1);
            }

            odo[d] = i_lists[d].len();

            stride *= self.shape()[d];
        }

        let mut incl = O::operator_identity();
        let mut excl = O::operator_identity();

        for u in OdometerBE::new(odo) {
            let mut ie = false;
            let mut index = 0;
            for d in 0..D {
                let IE { excl, i } = i_lists[d][u[d]];
                ie ^= excl as bool;
                index += i;
            }

            let dest = if !ie { &mut incl } else { &mut excl };
            #[cfg(not(feature = "unsafe"))]
            { *dest = O::operator(dest, &self.0.vec[index]); }
            #[cfg(feature = "unsafe")]
            // SAFETY: index = sum(i * stride) where each i < shape
            unsafe { *dest = O::operator(dest, self.0.vec.get_unchecked(index)); }
        }

        O::operator_inverse(&incl, &excl)
    }
}

impl<const D: usize, T, O: MonoidOps<T, T>> Debug for NdFenwick<D, T, O>
where T: Default + Debug
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
