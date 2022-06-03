#!/usr/bin/env python3

from pathlib import Path
from string import Template

class AtTemplate(Template):
    delimiter = '@'

for i in [0, 1]:
    with open(Path(__file__).parent / ['imin.rs', 'imax.rs'][i], 'w') as f:
        d = dict(
            min = ['min', 'max'][i],
            op = '<>'[i],
            example0 = [1, 8][i]
        )
        f.write(AtTemplate(r'''
/**
Incremental @{min}imization

```
# use rithmic::i@min;
# let (mut a, b) = (0, 0);
i@min!(a, b);

// effectively
a = a.@min(b);
```

with some advantages:
- Each expression is evaluated separately, avoiding borrow conflicts
- No code duplication of the left-hand expression, and it is evaluated only once
- Multiple right-hand expressions are allowed

# Examples
```
# use rithmic::i@min;
let mut x = [2, 7, 1, 8];

i@min!(x[0]; x[1], x[2], x[3]);
assert_eq!(x, [@example0, 7, 1, 8]);
```

# Notes
The first argument may be separated from the others by `,` or `;` for clarity
*/
#[macro_export]
macro_rules! i@min {
    ($lhs:expr, $rhs0:expr $(, $rhs:expr )* $(,)?) =>
    {
        #[allow(unused_mut)]
        let mut x0 = $rhs0;
        $(
            let x = $rhs;
            if x @op x0 { x0 = x; }
        )*
        let dest = &mut $lhs;
        if x0 @op *dest { *dest = x0; }
    };

    ($lhs:expr; $($tail:tt)*) => { i@min!($lhs, $($tail)*); };
}
'''[1:]
        ).substitute(d))
