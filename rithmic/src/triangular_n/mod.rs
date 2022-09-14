#[cfg(test)] mod tests;

use std::ops::RangeInclusive;

use num::Integer;

use crate::Rangelike;

/**
Return the *n*th [triangular number](https://en.wikipedia.org/wiki/Triangular_number), i.e the sum of `1..=n`, in *O*(1)

# Examples
```
# use rithmic::triangular_n;
assert_eq!(triangular_n(100), 5050);
```
*/
pub fn triangular_n<T>(n: T) -> T
where
    T: Copy + Integer,
    RangeInclusive<T>: Rangelike<T>
{
    triangular_slice(T::one()..=n)
}

/**
Return the sum of the integers in `range`, in *O*(1)

# Examples
```
# use rithmic::triangular_slice;
assert_eq!(triangular_slice(5..=10), 5+6+7+8+9+10);
```
*/
pub fn triangular_slice<T>(range: impl Rangelike<T>) -> T
where T: Copy + Integer
{
    triangular_steps(range, T::one())
}

/**
Return the sum of the integers in `range`, stepping by `step`, in *O*(1)

# Examples
```
# use rithmic::triangular_steps;
assert_eq!(triangular_steps(5..15, 3), 5+8+11+14);
```
*/
pub fn triangular_steps<T>(range: impl Rangelike<T>, step: T) -> T
where T: Copy + Integer
{
    let (Some(i), Some(j)) = range.canonical() else { panic!("unbounded range") };
    let zero = T::zero();
    assert!(i >= zero && j >= zero, "range must be non-negative");

    if j <= i { return T::zero() }

    let one = T::one();
    let two = one + one;

    let d = j-i - one;
    let steps = d / step;
    let d = step * steps;
    (i + i+d) * (steps + one) / two
}
