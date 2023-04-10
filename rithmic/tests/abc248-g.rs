#![feature(array_windows, box_patterns, decl_macro, exclusive_range_pattern, int_roundings, rustc_attrs)]
#![allow(clippy::collapsible_else_if, clippy::int_plus_one, clippy::needless_range_loop, clippy::nonminimal_bool, clippy::option_map_unit_fn, clippy::reversed_empty_ranges)]
#![warn(clippy::dbg_macro, clippy::imprecise_flops, clippy::print_stderr)]

mod helper;

use std::collections::BTreeMap;
use std::iter;
use std::path::Path;

use ac_library::ModInt998244353 as MInt;
use derive_more::{Add, AddAssign};
use itertools::{Itertools, EitherOrBoth};
use num::Integer;
use proconio::marker::Usize1;
use proconio::input;
use rithmic::graph::prelude::*;

use helper::IntoSource;

#[test] fn abc248_g() { main() }

fn main() {
    for (input, output) in helper::dir_io_pairs(
        Path::new("tests/")
            .join(Path::new(Path::new(file!()).file_stem().unwrap()))
    ) {
        testcase(input, output);
    }
}

fn testcase(input: impl IntoSource, judge: impl IntoSource) {
    input! {
        from input.into_source(),
        n: usize,
        a: [usize; n],
        uv: [(Usize1, Usize1); n-1]
    };
    const A_MAX: usize = 1e5 as usize;
    let g = Tree::<()>::from_iter_unweighted(n, uv);

    let mut divs = vec![vec![]; A_MAX+1];
    for i in 2..=A_MAX {
        for j in (2*i..=A_MAX).step_by(i) {
            divs[j].push(i);
        }
    }
    let mut bonus = vec![0; A_MAX+1];
    for i in 2..=A_MAX {
        bonus[i] = i-1 - divs[i].iter().map(|&j| bonus[j]).sum::<usize>();
    }

    #[derive(Default, Clone, Copy, Add, AddAssign)]
    struct Subset {
        n: MInt,
        sum_depths: MInt
    }
    impl Subset {
        fn leaf() -> Self {
            Self { n: MInt::raw(1), sum_depths: MInt::raw(0) }
        }
        fn score_with_sibling(self, other: Self) -> MInt {
            self.sum_depths * other.n +
            self.n * other.sum_depths +
            self.n * other.n * 3
        }
        fn deepen(self) -> Self {
            Self {
                n: self.n,
                sum_depths: self.sum_depths + self.n,
            }
        }
    }

    #[derive(Default, Clone)]
    struct State {
        score: MInt,
        subsets: BTreeMap<usize, Subset>,
        total: Subset,
    }
    impl State {
        fn mark(&mut self, x: usize, subset: Subset, divs: &[Vec<usize>]) {
            if x > 1 {
                for d in divs[x].iter().copied().chain(iter::once(x)) {
                    *self.subsets.entry(d).or_default() += subset;
                }
            }
        }
    }

    let rsf = g.rooted_subtree_fn_half( |step| -> State {
    match step {
        RsfStep::Leaf { p, u, .. } => {
            let mut state = State::default();
            state.mark(a[u].gcd(&a[p]), Subset::leaf(), &divs);
            state.total = Subset::leaf();
            state
        }
        RsfStep::Sibling { r0, r1, .. } => {
            let mut state = State::default();

            for (&g, &ssx, &ssy) in r0.subsets.iter()
                .merge_join_by(r1.subsets.iter(), |&a, &b| a.0.cmp(b.0))
                .filter_map(|i| if let EitherOrBoth::Both(a, b) = i { Some((a.0, a.1, b.1)) } else { None } )
            {
                state.score += ssx.score_with_sibling(ssy) * bonus[g];
            }
            state.score += r0.total.score_with_sibling(r1.total);
            state.score += r0.score + r1.score;

            for (&x, &ssx) in r0.subsets.iter().chain(r1.subsets.iter()) {
                *state.subsets.entry(x).or_default() += ssx;
            }

            state.total = r0.total + r1.total;
            state
        }
        RsfStep::Parent { pp, p, r, .. } => {
            let mut state = State::default();

            let g = if pp != NONE { a[p].gcd(&a[pp]) } else {1};
            state.mark(g, Subset::leaf(), &divs);
            state.total += Subset::leaf();

            for (&x, &ssx) in r.subsets.iter() {
                state.score += (ssx.sum_depths + ssx.n * 2) * bonus[x];
            }
            state.score += r.total.sum_depths + r.total.n * 2;
            state.score += r.score;

            for d in divs[g].iter().copied().chain(iter::once(g)) {
                if let Some(&ssd) = r.subsets.get(&d) {
                    *state.subsets.entry(d).or_default() += ssd.deepen();
                }
            }

            state.total += r.total.deepen();
            state
        }
    }});

    let ans = rsf[&(NONE, 0)].score;
    input! {
        from judge.into_source(),
        judge_ans: MInt
    };
    assert_eq!(ans, judge_ans);
}
