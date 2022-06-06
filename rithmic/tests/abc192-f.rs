#![allow(clippy::needless_range_loop)]

mod helper;

use std::path::Path;

use proconio::input;
use rithmic::{NdVec, imax, imin};

#[test] fn abc192_f() { main() }

fn main() {
    for (input, output) in helper::dir_io_pairs("tests/abc192-f") {
        testcase(input, output);
    }
}

fn testcase(input: impl AsRef<Path>, output: impl AsRef<Path>) {
    input! {
        from helper::source_from_path(input),
        n: usize, x: usize,
        a: [i32; n]
    };

    let mut dp = NdVec::<3, i32>::full([n+1, n+1, n], i32::MIN);
    for m in 1..=n {
        for r in 0..m {
            dp[[m, 0, r]] = 0;
        }
    }
    for i in 0..n {
        for m in 1..=n {
            for j in (0..=i.min(m-1)).rev() {
                for r in 0..m {
                    let v = dp[[m, j, r]] + a[i];
                    imax!(dp[[m, j+1, v as usize % m]], v);
    }}}}

    let mut ans = usize::MAX;
    for m in 1..=n {
        let x0 = dp[[m, m, x%m]];
        if x0 > 0 {
            imin!(ans, (x - x0 as usize) / m);
        }
    }

    input! {
        from helper::source_from_path(output),
        judge_ans: usize
    };
    assert_eq!(ans, judge_ans);
}
