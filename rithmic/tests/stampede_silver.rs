#![feature(path_file_prefix)]

mod helper;

use std::cmp::Reverse;
use std::path::Path;

use itertools::Itertools;
use proconio::input;

use helper::IntoSource;
use rithmic::DisjointIntervals;

#[test] fn usaco_stampede_silver() { main() }

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
        n: usize,
        xyr: [(i64, i64, i64); n],
    };

    let intervals = xyr.into_iter()
        .sorted_unstable_by_key(|(_x, y, _r)| Reverse(*y))
        .map(|(x, _y, r)| (-(x+1)*r, -x*r));

    let mut di = DisjointIntervals::new();
    for (i, u) in intervals.enumerate() {
        di.insert(u, i);
    }
    let ans = di.iter().map(|(_, &i)| i).sorted_unstable().dedup().count();

    input! {
        from judge.into_source(),
        judge_ans: usize,
    };
    assert_eq!(ans, judge_ans);
}
