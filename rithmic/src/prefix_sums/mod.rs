#[cfg(test)] mod tests;

use std::ops::{Add, Sub};

use crate::{NdVec, OdometerBE, IntBitOps};

/**
Calculate prefix or suffix sums

# Examples
```
use rithmic::PrefixSums;

let a = [3, 5, 7];
assert_eq!(a.prefix_sums(), vec![0, 3, 8, 15]);
assert_eq!(a.suffix_sums(), vec![15, 12, 7, 0]);

let a = rithmic::NdVec::from_raw([3, 3], (0..9).collect());
assert_eq!(a[0], [0, 1, 2]);
assert_eq!(a[1], [3, 4, 5]);
assert_eq!(a[2], [6, 7, 8]);

let ps = a.prefix_sums();
assert_eq!(ps[0], [0, 0,  0,  0]);
assert_eq!(ps[1], [0, 0,  1,  3]);
assert_eq!(ps[2], [0, 3,  8, 15]);
assert_eq!(ps[3], [0, 9, 21, 36]);

let ss = a.suffix_sums();
assert_eq!(ss[0], [36, 27, 15, 0]);
assert_eq!(ss[1], [33, 24, 13, 0]);
assert_eq!(ss[2], [21, 15,  8, 0]);
assert_eq!(ss[3], [ 0,  0,  0, 0]);
```
*/
pub trait PrefixSums {
    type Output;
    fn prefix_sums(&self) -> Self::Output;
    fn suffix_sums(&self) -> Self::Output;
}

impl<T> PrefixSums for [T]
where
    T: Default,
    for<'a> &'a T: Add<Output=T>
{
    type Output = Vec<T>;

    /// See [`PrefixSums`] for more information
    fn prefix_sums(&self) -> Vec<T>
    {
        let n = self.len();
        let mut ps = Vec::with_capacity(n+1);

        ps.push(T::default());
        for next in self {
            let sum = ps.last().unwrap() + next;
            ps.push(sum);
        }
        ps
    }

    /// See [`PrefixSums`] for more information
    fn suffix_sums(&self) -> Vec<T>
    {
        let n = self.len();
        let mut ss = Vec::with_capacity(n+1);
        ss.resize_with(n+1, T::default);

        for i in (0..n).rev() {
            ss[i] = &ss[i+1] + &self[i];
        }
        ss
    }
}

impl<const D: usize, T> PrefixSums for NdVec<D, T>
where
    T: Default,
    for<'a> &'a T: Add<Output=T> + Sub<Output=T>
{
    type Output = Self;

    /// See [`PrefixSums`] for more information
    fn prefix_sums(&self) -> Self
    {
        let mut shape1 = self.shape();
        for n in shape1.iter_mut() { *n += 1; }

        let mut ps = Self::new(shape1);
        for i in OdometerBE::new(self.shape()) {
            let mut i1 = i;
            for ii in i1.iter_mut() { *ii += 1; }

            let mut incl = T::default();
            let mut excl = T::default();
            incl = &incl + &self[i];

            for u in 1 ..= usize::mask(D as _) {
                let mut j1 = i1;
                for d in 0..D {
                    if u >> d &1==1 {
                        j1[d] -= 1;
                    }
                }

                if u.count_ones() &1==1 {
                    incl = &incl + &ps[j1];
                } else {
                    excl = &excl + &ps[j1];
                }
            }
            ps[i1] = &incl - &excl;
        }
        ps
    }

    /// See [`PrefixSums`] for more information
    fn suffix_sums(&self) -> Self
    {
        let mut shape1 = self.shape();
        for n in shape1.iter_mut() { *n += 1; }

        let mut ss = Self::new(shape1);
        for i in OdometerBE::new(self.shape()).rev()
        {
            let mut incl = T::default();
            let mut excl = T::default();
            incl = &incl + &self[i];

            for u in 1 ..= usize::mask(D as _) {
                let mut j1 = i;
                for d in 0..D {
                    if u >> d &1==1 {
                        j1[d] += 1;
                    }
                }

                if u.count_ones() &1==1 {
                    incl = &incl + &ss[j1];
                } else {
                    excl = &excl + &ss[j1];
                }
            }
            ss[i] = &incl - &excl;
        }
        ss
    }
}
