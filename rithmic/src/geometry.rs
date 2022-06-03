use std::ops::{Add, Mul, Sub};

use euclid::Vector2D;
use num::Signed;

pub trait Vector2DMore
{
    type T;
    fn length2(self) -> Self::T;
    fn o_cross(self, u: Self, v: Self) -> Self::T;
    fn rotate_scale(self, other: Self) -> Self;
    fn same_dir(self, other: Self) -> bool;
}

impl<T, U> Vector2DMore for Vector2D<T, U>
where
    T: Copy
        + Add<Output=T> + Sub<Output=T> + Mul<Output=T>
        + PartialOrd + Signed
{
    type T = T;

    fn length2(self) -> T {
        self.x * self.x + self.y * self.y
    }

    fn o_cross(self, u: Self, v: Self) -> T {
        (u-self).cross(v-self)
    }

    fn rotate_scale(self, other: Self) -> Self {
        Self::new(
            self.x * other.x - self.y * other.y,
            self.x * other.y + self.y * other.x
        )
    }

    fn same_dir(self, other: Self) -> bool {
        self.cross(other).is_zero() && self.dot(other).is_positive()
    }
}
