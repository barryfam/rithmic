/**
Incremental minimization

```
# use rithmic::imin;
# let (mut a, b) = (0, 0);
imin!(a, b);

// effectively
a = a.min(b);
```

with some advantages:
- Each expression is evaluated separately, avoiding borrow conflicts
- No code duplication of the left-hand expression, and it is evaluated only once
- Multiple right-hand expressions are allowed

# Examples
```
# use rithmic::imin;
let mut x = [2, 7, 1, 8];

imin!(x[0]; x[1], x[2], x[3]);
assert_eq!(x, [1, 7, 1, 8]);
```

# Notes
The first argument may be separated from the others by `,` or `;` for clarity
*/
pub macro imin {
    ($lhs:expr, $rhs0:expr $(, $rhs:expr )* $(,)?) =>
    {
        #[allow(unused_mut)]
        let mut x0 = $rhs0;
        $(
            let x = $rhs;
            if x < x0 { x0 = x; }
        )*
        let dest = &mut $lhs;
        if x0 < *dest { *dest = x0; }
    },

    ($lhs:expr; $($tail:tt)*) => { imin!($lhs, $($tail)*); },
}
