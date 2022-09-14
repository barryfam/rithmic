#!/usr/bin/env python3

from pathlib import Path
from string import Template
from textwrap import indent
from typing import TextIO

class AtTemplate(Template):
    delimiter = '@'

class Writer:
    def __init__(self, mapping: dict, file: TextIO):
        self.mapping = mapping
        self.file = file

    def __call__(self, template: str='', tabs=0, newlines=1):
        template = indent(template.rstrip(' ').strip('\n'), ' '*4 * tabs)
        print(AtTemplate(template).substitute(self.mapping), end='\n' * newlines, file=self.file)



for i in [0, 1]:
    with open(Path(__file__).parent / ['imin.rs', 'imax.rs'][i], 'w') as f:
        writer = Writer(locals(), f)

        min_ = ['min', 'max'][i]
        op = '<>'[i]
        example0 = [1, 8][i]

        writer(r'''

/**
Incremental @{min_}imization

```
# use rithmic::i@min_;
# let (mut a, b) = (0, 0);
i@min_!(a, b);

// effectively
a = a.@min_(b);
```

with some advantages:
- Each expression is evaluated separately, avoiding borrow conflicts
- No code duplication of the left-hand expression, and it is evaluated only once
- Multiple right-hand expressions are allowed

# Examples
```
# use rithmic::i@min_;
let mut x = [2, 7, 1, 8];

i@min_!(x[0]; x[1], x[2], x[3]);
assert_eq!(x, [@example0, 7, 1, 8]);
```

# Notes
The first argument may be separated from the others by `,` or `;` for clarity
*/
pub macro i@min_ {
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
    },

    ($lhs:expr; $($tail:tt)*) => { i@min_!($lhs, $($tail)*); },
}

        ''')
