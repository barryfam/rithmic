#![allow(clippy::needless_range_loop)]

mod helper;

use std::path::Path;

use proconio::input;
use rithmic::NdFenwick;

#[test] fn arc136_d() { main() }

fn main() {
    for (input, output) in helper::dir_io_pairs("tests/arc136-d") {
        testcase(input, output);
    }
}

fn testcase(input: impl AsRef<Path>, output: impl AsRef<Path>) {
    input! {
        from helper::source_from_path(input),
        n: usize,
        a: [u32; n]
    };

    fn digits(mut a: u32) -> [usize; 6] {
        let mut digits = [0; 6];
        for d in 0..6 {
            digits[d] = (a % 10) as usize;
            a /= 10;
        }
        digits
    }

    let mut ans: u64 = 0;
    let mut ft = NdFenwick::<6, u32>::new([10; 6]);
    for u in a.into_iter().map(digits) {
        ans += ft.prefix(u.map(|x| 10-x)) as u64;
        ft.update(u, 1);
    }

    input! {
        from helper::source_from_path(output),
        judge_ans: u64
    };
    assert_eq!(ans, judge_ans);
}