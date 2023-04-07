#![cfg(__)]

mod helper;

use std::path::Path;

use proconio::input;

use helper::IntoSource;

#[test] fn abc000_x() { main() }

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
    };

    let ans = 0;

    input! {
        from judge.into_source(),
        judge_ans: u64
    };
    assert_eq!(ans, judge_ans);
}
