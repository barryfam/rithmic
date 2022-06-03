#[cfg(test)] mod tests;

use std::ops::{Add, Div, Mul, Sub, Neg};

/// _O(n)_
pub fn polynomial_add<T>(f: &[T], g: &[T]) -> Vec<T>
where
    T: Copy + From<u8> + PartialEq + Add<Output=T>
{
    let mut h = Vec::<T>::with_capacity(f.len().max(g.len()));

    let b = f.len().min(g.len());
    (0..b).map(|i| f[i] + g[i]).collect_into(&mut h);
    h.extend_from_slice(&(if f.len() >= g.len() {f} else {g})[b..]);

    while Some(&0.into()) == h.last() { h.pop(); }
    h
}

/// Note: This runs in _O(n²)_, so DFT convolution in _O(n log n)_ should be used instead when possible
pub fn polynomial_mul<T>(f: &[T], g: &[T]) -> Vec<T>
where
    T: Copy + Add<Output=T> + Mul<Output=T>
{
    if f.is_empty() || g.is_empty() { return vec![] }

    let n = (f.len() + g.len()) - 1;

    (0..n).map( |i|
        (i.saturating_sub(g.len()-1) ..= i.min(f.len()-1)).map( |j|
            f[j] * g[i-j]
        )
        .reduce(T::add).unwrap()
    )
    .collect()
}

/// _O(n²)_
pub fn polynomial_div<T>(f: &[T], g: &[T]) -> (Vec<T>, Vec<T>)
where
    T: Copy + Default + From<u8> + PartialEq + Sub<Output=T> + Mul<Output=T> + Div<Output=T>
{
    let mut gl = g.len();
    while gl > 0 && g[gl-1] == 0.into() { gl -= 1; }
    assert!(gl > 0, "Division by zero polynomial");

    if gl > f.len() { return (vec![], f.to_vec()) }
    let n = f.len() - gl + 1;
    let mut q = vec![T::default(); n];
    let mut r = f.to_vec();

    for k in (0..n).rev() {
        let d = r.pop().unwrap() / g[gl-1];
        q[k] = d;

        for j in 0..gl-1 {
            r[k+j] = r[k+j] - g[j] * d;
        }
    }

    while Some(&0.into()) == r.last() { r.pop(); }
    (q, r)
}

pub fn polynomial_div_exact<T>(f: &[T], g: &[T]) -> Vec<T>
where
    T: Copy + Default + From<u8> + PartialEq + Sub<Output=T> + Mul<Output=T> + Div<Output=T>
{
    let (q, r) = polynomial_div(f, g);
    assert!(r.is_empty(), "Polynomials not exactly divisible");
    q
}

/// _O(n²)_
pub fn lagrange_interpolation<T>(x: &[T], y: &[T]) -> Vec<T>
where
    T: Copy + Default + From<u8>
        + PartialEq + Neg<Output=T>
        + Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>
{
    assert_eq!(x.len(), y.len(), "Points & values of different lengths");
    let n = x.len();

    let binom = x.iter().map(|&xi| vec![-xi, 1.into()]).collect::<Vec<_>>();
    let all = binom.iter().cloned().reduce(|f, g| polynomial_mul(&f, &g)).unwrap();

    (0..n).map( |i| {
        let mut term = polynomial_div_exact(&all, &binom[i]);
        let d = (0..i).chain(i+1..n).map(|j| x[i]-x[j]).reduce(T::mul).unwrap();
        for c in &mut term { *c = *c * y[i] / d; }
        term
    })
    .reduce(|f, g| polynomial_add(&f, &g)).unwrap()
}
