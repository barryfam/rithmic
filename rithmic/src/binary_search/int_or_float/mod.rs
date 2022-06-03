#[cfg(test)] mod tests;

pub trait IntOrFloat: Copy + PartialEq
{
    const MIN: Self;
    const MAX: Self;

    fn next_towards(self, other: Self) -> Self;
    fn midpoint(self, other: Self) -> Self;  // must round towards self, i.e. never return other
}

macro_rules! impl_for_int {
    ($type:ty) => {
        impl IntOrFloat for $type
        {
            const MIN: $type = <$type>::MIN;
            const MAX: $type = <$type>::MAX;

            fn next_towards(self, other: Self) -> Self {
                debug_assert!(self != other);

                if self < other { self+1 }
                else { self-1 }
            }

            fn midpoint(self, other: Self) -> Self {
                debug_assert!(self != other);

                let (xor, carry) = (self ^ other, self & other);
                let ceil = if other < self {1} else {0};
                carry + (xor >> 1) + (xor & ceil)
            }
        }
    };

    ( $( $type:ty ),+ ) => {
        $( impl_for_int!($type); )*
    };
}

impl_for_int!(usize, u128, u64, u32, u16, u8);
impl_for_int!(isize, i128, i64, i32, i16, i8);

macro_rules! impl_for_float {
    ($type:ty, $nextafter:expr) => {
        impl IntOrFloat for $type
        {
            const MIN: $type = <$type>::MIN;
            const MAX: $type = <$type>::MAX;

            fn next_towards(self, other: Self) -> Self {
                debug_assert!(self != other);

                $nextafter(self, other)
            }

            fn midpoint(self, other: Self) -> Self {
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
        }
    };
}

impl_for_float!(f64, libm::nextafter);
impl_for_float!(f32, libm::nextafterf);
