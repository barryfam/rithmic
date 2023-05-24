#[cfg(test)] mod tests;

use num::Integer;
use rand::{thread_rng, Rng};

use crate::OrdPair;

pub trait IntOrFloat: Copy + Send + PartialEq + PartialOrd
{
    const MIN: Self;
    const MAX: Self;

    fn next_towards(self, other: Self) -> Self;
    fn midpoint_shy(self, other: Self) -> Self;  // must round towards `self`, i.e. never return `other`
    fn n1_sect<const N: usize>(l: Self, r: Self) -> [Self; N];  // may (and should, eventually) return `other`

    fn in_interval(self, l: Self, r: Self) -> bool {
        let (a, b) = (l, r).ordered();
        a <= self && self <= b
    }
}

macro impl_for_int {
    ($type:ty) => {
        impl IntOrFloat for $type
        {
            const MIN: $type = <$type>::MIN;
            const MAX: $type = <$type>::MAX;

            #[inline]
            fn next_towards(self, other: Self) -> Self {
                debug_assert!(self != other);

                if self < other { self+1 }
                else { self-1 }
            }

            #[inline]
            fn midpoint_shy(self, other: Self) -> Self {
                debug_assert!(self != other);

                let (xor, carry) = (self ^ other, self & other);
                let ceil = if other < self {1} else {0};
                carry + (xor >> 1) + (xor & ceil)
            }

            fn n1_sect<const N: usize>(l: Self, r: Self) -> [Self; N] {
                assert!(N >= 1);

                let n: Self = N.try_into().unwrap();
                assert!(n.checked_mul(n).unwrap().checked_mul(2).is_some(),
                    "integer type must be able to contain 2*N^2 without overflow (N={})", N);

                let (a, b) = (l, r).ordered();

                let mut ret = [0; N];
                if a.saturating_add(n-1) >= b {
                    for i in 0..n {
                        ret[i as usize] = a.saturating_add(i).min(b);
                    }
                }
                else {
                    let (qa, ra) = a.div_rem(&(n+1));
                    let (qb, rb) = b.div_rem(&(n+1));
                    for i in 0..n {
                        let (xa, xb) = (i+1, n+1 - (i+1));
                        ret[i as usize] = xa*qa + xb*qb + (xa*ra + xb*rb)/(n+1);
                    }
                }
                ret
            }
        }
    },

    ( $( $type:ty ),+ ) => {
        $( impl_for_int!($type); )*
    },
}

impl_for_int!(usize, u128, u64, u32, u16, u8);
impl_for_int!(isize, i128, i64, i32, i16, i8);

macro impl_for_float {
    ($type:ty, $nextafter:expr) => {
        impl IntOrFloat for $type
        {
            const MIN: $type = <$type>::MIN;
            const MAX: $type = <$type>::MAX;

            #[inline]
            fn next_towards(self, other: Self) -> Self {
                debug_assert!(self != other);

                $nextafter(self, other)
            }

            #[inline]
            fn midpoint_shy(self, other: Self) -> Self {
                debug_assert!(self != other);

                let half_max = Self::MAX / 2.;

                let m = if self.abs() <= half_max && other.abs() <= half_max {
                    (self + other) / 2.
                } else {
                    self / 2. + other / 2.
                };

                if m == other { self }
                else { m }
            }

            fn n1_sect<const N: usize>(l: Self, r: Self) -> [Self; N] {
                assert!(N >= 1);

                let (a, b) = (l, r).ordered();
                [0; N].map(|_| thread_rng().gen_range(a..=b))  // TODO: evenly-spaced implementation
            }
        }
    },
}

impl_for_float!(f64, libm::nextafter);
impl_for_float!(f32, libm::nextafterf);
