#![feature(path_file_prefix)]
#![feature(rustc_attrs)]

mod helper;

use std::path::Path;

use proconio::input;
use proconio::marker::Chars;
use rithmic::{AhoCorasick, memoize};

use helper::IntoSource;

#[test] fn usaco_censor_gold() { main() }

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
        s: Chars,
        n: usize,
        t: [Chars; n],
    };

    let ac = AhoCorasick::new(t);

    #[memoize(..2)]
    fn ac_step(u: usize, c: char, ac: &AhoCorasick<char>) -> usize {
        ac.step(u, c)
    }

    let mut u = 0;
    let mut u_rollback = vec![0];
    let mut ans = vec![];

    for c in s {
        ans.push(c);
        u = ac_step!(u, c);
        u_rollback.push(u);

        if let Some(&pid) = ac.matches(u).get(0) {
            let m = ac.patterns[pid].len();

            ans.truncate(ans.len() - m);
            u_rollback.truncate(u_rollback.len() - m);
            u = *u_rollback.last().unwrap();
        }
    }

    input! {
        from judge.into_source(),
        judge_ans: Chars,
    };
    assert_eq!(ans, judge_ans);
}
