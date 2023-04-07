use std::ops::{Add, Sub, Mul};

use num::Integer;
use num::rational::Ratio;

pub trait IntFloatOrRatio: Default + Copy + PartialOrd
    + Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self>
{
    fn div_ceil(self, rhs: Self) -> Self;
    fn just_before(self) -> Self;
}

macro impl_for_int {
    ($type:ty) => {
        impl IntFloatOrRatio for $type
        {
            #[inline]
            fn div_ceil(self, rhs: Self) -> Self {
                self.div_ceil(rhs)
            }

            #[inline]
            fn just_before(self) -> Self {
                self - 1
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
    ($type:ty) => {
        impl IntFloatOrRatio for $type
        {
            #[inline]
            fn div_ceil(self, rhs: Self) -> Self {
                self / rhs
            }

            #[inline]
            fn just_before(self) -> Self {
                self
            }
        }
    },

    ( $( $type:ty ),+ ) => {
        $( impl_for_float!($type); )*
    },
}

impl_for_float!(f64, f32);

impl<T: Default + Copy + Integer> IntFloatOrRatio for Ratio<T> {
    #[inline]
    fn div_ceil(self, rhs: Self) -> Self {
        self / rhs
    }

    #[inline]
    fn just_before(self) -> Self {
        self
    }
}
