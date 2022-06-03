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

pub struct Sum;
impl<T> MonoidOps<T, T> for Sum
where
    T: Default + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: Debug,
    for<'a> &'a T: Add<Output=T> + Sub<Output=T> + Mul<Output=T>,
{
    fn operator(x: &T, y: &T) -> T { x + y }
    const INVERTIBLE: bool = true;
    fn operator_inverse(x: &T, y: &T) -> T { x - y }

    fn update(x: &T, a: &T) -> T { x + a }
    const LAZY: bool = true;
    fn update_distributive((i, j): (usize, usize), x: &T, a: &T) -> Option<T> {
        Some(x + &(a * &(j-i).try_into().unwrap()))
    }
    fn update_composition(a: &T, b: &T) -> T { a + b }
}
