use crate::Rangelike;

/**
Return the *n*th [triangular number](https://en.wikipedia.org/wiki/Triangular_number), i.e the sum of `1..=n`, in *O*(1)

# Examples
```
# use rithmic::triangular_n;
assert_eq!(triangular_n(100), 5050);
```
*/
pub fn triangular_n(n: usize) -> usize {
    triangular_slice(1..=n)
}

/**
Return the sum of the integers in `range`, in *O*(1)

# Examples
```
# use rithmic::triangular_slice;
assert_eq!(triangular_slice(5..=10), 5+6+7+8+9+10);
```
*/
pub fn triangular_slice(range: impl Rangelike<usize>) -> usize {
    triangular_steps(range, 1)
}

/**
Return the sum of the integers in `range`, stepping by `step`, in *O*(1)

# Examples
```
# use rithmic::triangular_steps;
assert_eq!(triangular_steps(5..12, 3), 5+8+11);
```
*/
pub fn triangular_steps(range: impl Rangelike<usize>, step: usize) -> usize
{
    let (Some(i), Some(j)) = range.canonical() else { panic!("unbounded range") };
    if j <= i { return 0 }

    let d = j-i - 1;
    let steps = d / step;
    let d = step * steps;
    (i + i+d) * (steps + 1) / 2
}



#[cfg(test)] mod tests;
