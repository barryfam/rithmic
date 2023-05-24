#![feature(array_windows)]

mod helper;

use std::path::Path;

use itertools::{MinMaxResult, Itertools};
use num::rational::Ratio;
use proconio::input;

use helper::IntoSource;
use rithmic::{ConvexHullMin, ConvexHullMax};

#[test] fn abc296_g() { main() }

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
        mut xy: [(i128, i128); n],
        q: usize,
        ab: [(i128, i128); q],
    };
    xy.push(xy[0]);

    let MinMaxResult::MinMax(&x0, &x1) = xy.iter().map(|(x, _y)| x).minmax()
        else { panic!() };

    let r = |x| Ratio::new(x, 1);
    let mb = |u: (i128, i128), v: (i128, i128)| {
        let m = (r(v.1)-r(u.1)) / (r(v.0)-r(u.0));
        (m, r(u.1) - m*u.0)
    };

    let mut top = ConvexHullMin::new(r(x0), r(x1));
    let mut bot = ConvexHullMax::new(r(x0), r(x1));
    for &[u, v] in xy.array_windows() {
        if u.0 > v.0 {
            let (m, b) = mb(u, v);
            top.insert(m, b);
        }
        else if u.0 < v.0 {
            let (m, b) = mb(u, v);
            bot.insert(m, b);
        }
    }

    let mut ans = vec![];
    for (a, b) in ab {
        ans.push({
            if a < x0 || x1 < a {
                "OUT"
            } else {
                let y0 = bot.eval_at(r(a));
                let y1 = top.eval_at(r(a));

                if r(b) < y0 || y1 < r(b) {"OUT"}
                else if a == x0 || a == x1 || r(b) == y0 || r(b) == y1 {"ON"}
                else {"IN"}
            }
        });
    }

    input! {
        from judge.into_source(),
        judge_ans: [String; q],
    };
    assert_eq!(ans, judge_ans);
}
