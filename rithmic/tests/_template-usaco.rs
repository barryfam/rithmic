#![cfg(__)]

#![feature(path_file_prefix)]

mod helper;

use std::path::Path;

use proconio::input;

use helper::IntoSource;

#[test] fn usaco_xxx() { main() }

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
    };

    let ans = 0;

    input! {
        from judge.into_source(),
        judge_ans: u64
    };
    assert_eq!(ans, judge_ans);
}
