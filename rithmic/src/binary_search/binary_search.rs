use std::ops::{RangeBounds, Bound};

use super::int_or_float::IntOrFloat;

/**
Binary search `domain` for the first point at which `predicate` returns `true` or `false` (matching `search_for`)

`domain` may be a:
- fully- or half- open, closed, or unbounded interval
- in positive or negative direction
- of integers or floats

All 4 possible "configurations" of (left-to-right / right-to-left), (search for true / search for false) are supported. The intent is to reduce off-by-one and edge case errors by choosing the most human-intuitive form

# Examples
```
# use approx::assert_ulps_eq;
# use rithmic::binary_search;
let x = binary_search(1..=100, true, |x| x*x > 2000);
assert_eq!(x, Some(45));

assert_eq!(     binary_search( 100..=1  , false, |x| x*x >  2000)  , Some(44));
assert_eq!(     binary_search(   1..=100, false, |x| x*x <= 2000)  , Some(45));
assert_eq!(     binary_search( 100..=1  , true , |x| x*x <= 2000)  , Some(44));

assert_eq!(     binary_search(    .. 0  , true , |x| x > -27)      , Some(-26));
assert_eq!(     binary_search(    ..    , true , |x| x >= i32::MAX), Some(i32::MAX));
assert_ulps_eq!(binary_search( 0.0..=1e3, true , |x| x*x > 2e3).unwrap(), 44.721359549995796);
```

# Notes
If searching unbounded intervals, `predicate` may be called with very large arguments; it must be able to handle these without overflow

As usual for binary search, there must exist some `x` such that all `predicate(l)` for all `l < x` and `predicate(r)` for all `r >= x` are opposite values. Results will be incorrect if this is not true
*/
pub fn binary_search<X>(
    domain: impl RangeBounds<X>,
    search_for: bool,
    mut predicate: impl FnMut(X) -> bool
) -> Option<X>
where
    X: IntOrFloat
{
    let (mut r, end_incl) = match domain.end_bound() {
        Bound::Excluded(&end) => (end, false),
        Bound::Included(&end) => (end, true),
        Bound::Unbounded => (X::MAX, true)
    };
    let mut l = match domain.start_bound() {
        Bound::Included(&start) => start,
        Bound::Excluded(&start) => {
            if start == r { return None }
            start.next_towards(r)
        }
        Bound::Unbounded => X::MIN
    };

    let r0 = r;
    while l != r {
        let m = l.midpoint_shy(r);
        if predicate(m) == search_for {
            r = m;
        } else {
            l = m.next_towards(r);
        }
    }

    (r != r0 || end_incl && predicate(r) == search_for)
        .then_some(r)
}
