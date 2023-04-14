#![feature(path_file_prefix)]

mod helper;

use std::path::Path;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;
use rithmic::BVec;

use helper::IntoSource;

#[test] fn usaco_meeting_silver() { main() }

fn main() {
    let p = Path::new("tests/usaco/").join(
        Path::new(Path::new(file!()).file_prefix().unwrap()).with_extension("zip")
    );
    for (input, output) in helper::zip_io_pairs(p) {
        testcase(input, output);
    }
}

fn testcase(input: impl IntoSource, judge: impl IntoSource) {
    input! {
        from input.into_source(),
        n: usize, m: usize,
        abcd: [(Usize1, Usize1, usize, usize); m],
    };
    const MAX_EDGE: usize = 100;

    let mut dp0 = vec![BVec::new((n-1)*MAX_EDGE + 1); n];
    dp0[0].set(0, true);
    let mut dp1 = dp0.clone();

    for (a, b, c, d) in abcd.into_iter().sorted_unstable() {
        let u = &dp0[a] << c;
        dp0[b] |= u;
        let u = &dp1[a] << d;
        dp1[b] |= u;
    }

    let ans = if let Some(ans) = (&dp0[n-1] & &dp1[n-1]).first_one() {
        ans.to_string()
    } else {
        "IMPOSSIBLE".to_string()
    };

    input! {
        from judge.into_source(),
        judge_ans: String,
    };
    assert_eq!(ans, judge_ans);
}
