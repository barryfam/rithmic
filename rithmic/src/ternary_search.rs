use std::ops::Range;

/**
Ternary search for the maximum point of a function `f()` in the given `domain`

`f()` must be unimodal — i.e. it must have only one local maximum — within `domain`

# Examples

```
# use approx::assert_abs_diff_eq;
# use rithmic::ternary_search;
let x = ternary_search(0. .. 6., |x| -x.cos());

assert_abs_diff_eq!(x, 3.141593, epsilon = 1e-6);
```
*/
pub fn ternary_search<F, Y>(domain: Range<f64>, mut f: F) -> f64
where
    F: FnMut(f64) -> Y,
    Y: PartialOrd
{
    assert!(domain.start <= domain.end);

    const INV_PHI_2: f64 = 0.3819660112501051;  // (3 - sqrt(5)) / 2

    let (mut a, mut d) = (domain.start, domain.end);
    let mut b = a + INV_PHI_2 * (d-a);
    let mut c = b + INV_PHI_2 * (d-b);
    let mut fb = f(b);
    let mut fc = f(c);

    while b < c {
        if fb < fc {
            a = b;
            (b, fb) = (c, fc);
            c = b + INV_PHI_2 * (d-b);
            fc = f(c);
        }
        else {
            d = c;
            (c, fc) = (b, fb);
            b = c - INV_PHI_2 * (c-a);
            fb = f(b);
        }
    }
    b
}
