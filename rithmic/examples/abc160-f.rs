#![feature(explicit_generic_args_with_impl_trait)]

mod helper;

use std::path::Path;

use ac_library_rs::ModInt1000000007 as MInt;
use proconio::input;
use proconio::marker::Usize1;
use rithmic::FactorialTable;
use rithmic::graph::prelude::*;

fn main() {
    for (input, output) in helper::dir_io_pairs("examples/abc160-f") {
        testcase(input, output);
    }
}

fn testcase(input: impl AsRef<Path>, output: impl AsRef<Path>) {
    input! {
        from helper::source_from_path(input),
        n: usize,
        ab: [(Usize1, Usize1); n-1]
    }
    let g = Tree::<()>::from_iter_unweighted(n, ab);
    let ft = FactorialTable::<MInt>::new(n);

    #[derive(Clone, Copy)]
    struct S {
        size: usize,
        ways: MInt
    }

    let rsf = g.rooted_subtree_fn::<S>( |step| {
        match step {
            RsfStep::Leaf { .. } => S { size: 1, ways: MInt::raw(1) },
            RsfStep::Sibling { r0, r1, .. } => {
                let size = r0.size + r1.size;
                S {
                    size,
                    ways: r0.ways * r1.ways * ft.comb(size, r0.size)
                }
            }
            RsfStep::Parent { r, .. } => S { size: r.size + 1, ..*r }
        }
    });

    let ans: Vec<_> = (0..n).map(|k| rsf[&(NONE, k)].ways).collect();

    input! {
        from helper::source_from_path(output),
        judge_ans: [MInt; n]
    }
    assert_eq!(ans, judge_ans);
}
