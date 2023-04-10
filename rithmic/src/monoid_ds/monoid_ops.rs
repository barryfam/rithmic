use std::fmt::Debug;
use std::ops::{Add, Sub, Mul};

#[allow(unused_variables)]
pub trait MonoidOps<T: Default, U: Default>
{
    fn operator(x: &T, y: &T) -> T;
    fn operator_identity() -> T { T::default() }
    const INVERTIBLE: bool = false;
    fn operator_inverse(x: &T, y: &T) -> T { unimplemented!() }

    fn update(x: &T, a: &U) -> T { unimplemented!() }
    fn update_identity() -> U { U::default() }
    const LAZY: bool = false;
    fn update_distributive(span: (usize, usize), x: &T, a: &U) -> Option<T> { unimplemented!() }
    fn update_composition(a: &U, b: &U) -> U { unimplemented!() }
}

#[derive(Default, Clone, Copy)]
pub struct USumQSum;
impl<T> MonoidOps<T, T> for USumQSum
where
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    for<'a> &'a T: Add<Output=T> + Sub<Output=T> + Mul<Output=T>,
{
    #[inline] fn operator(x: &T, y: &T) -> T { x + y }
    const INVERTIBLE: bool = true;
    #[inline] fn operator_inverse(x: &T, y: &T) -> T { x - y }

    #[inline] fn update(x: &T, a: &T) -> T { x + a }
    const LAZY: bool = true;
    #[inline] fn update_distributive((i, j): (usize, usize), x: &T, a: &T) -> Option<T> {
        Some(x + &(a * &(j-i).try_into().unwrap()))
    }
    #[inline] fn update_composition(a: &T, b: &T) -> T { a + b }
}

pub macro monoid_ops {
    ($v:vis $name:ident<$t:ty, $u:ty> = $($tail:tt)*) => {
        #[derive(Default, Clone, Copy)]
        $v struct $name;
        impl MonoidOps<$t, $u> for $name {
            monoid_ops!(@ $name<$t, $u> @ $($tail)*);
        }
    },

    (@ $name:ident<$t:ty, $u:ty> @ operator($x:pat, $y:pat) $f:block $($tail:tt)*) => {
        #[inline] fn operator($x: &$t, $y: &$t) -> $t $f
        monoid_ops!(@ $name<$t, $u> @ $($tail)*);
    },
    (@ $name:ident<$t:ty, $u:ty> @ operator_identity() $f:block $($tail:tt)*) => {
        #[inline] fn operator_identity() -> $t $f
        monoid_ops!(@ $name<$t, $u> @ $($tail)*);
    },
    (@ $name:ident<$t:ty, $u:ty> @ operator_inverse($x:pat, $y:pat) $f:block $($tail:tt)*) => {
        const INVERTIBLE: bool = true;
        #[inline] fn operator_inverse($x: &$t, $y: &$t) -> $t $f
        monoid_ops!(@ $name<$t, $u> @ $($tail)*);
    },

    (@ $name:ident<$t:ty, $u:ty> @ update($x:pat, $a:pat) $f:block $($tail:tt)*) => {
        #[inline] fn update($x: &$t, $a: &$u) -> $t $f
        monoid_ops!(@ $name<$t, $u> @ $($tail)*);
    },
    (@ $name:ident<$t:ty, $u:ty> @ update_identity() $f:block $($tail:tt)*) => {
        #[inline] fn update_identity() -> $u $f
        monoid_ops!(@ $name<$t, $u> @ $($tail)*);
    },
    (@ $name:ident<$t:ty, $u:ty> @ update_distributive($span:pat, $x:pat, $a:pat) $f:block $($tail:tt)*) => {
        const LAZY: bool = true;
        #[inline] fn update_distributive($span: (usize, usize), $x: &$t, $a: &$u) -> Option<$t> $f
        monoid_ops!(@ $name<$t, $u> @ $($tail)*);
    },
    (@ $name:ident<$t:ty, $u:ty> @ update_composition($a:pat, $b:pat) $f:block $($tail:tt)*) => {
        #[inline] fn update_composition($a: &$u, $b: &$u) -> $u $f
        monoid_ops!(@ $name<$t, $u> @ $($tail)*);
    },

    (@ $name:ident<$t:ty, $u:ty> @) => {}
}



#[cfg(test)]
mod tests {
    use super::*;

    monoid_ops! {
        Test<usize, usize> =
        operator(x, y) { x+y }
        update_distributive((i, j), _x, a) { Some((j-i)*a) }
        update_composition(_a, b) { *b }
    }
}
