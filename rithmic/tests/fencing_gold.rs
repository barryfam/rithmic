#![feature(decl_macro)]
#![feature(path_file_prefix)]

mod helper;

use std::path::Path;

use num::rational::Ratio;
use proconio::input;
use rithmic::{ConvexHullMin, ConvexHullMax, imin, imax};

use helper::IntoSource;

#[test] fn usaco_fencing_gold() { main() }

fn main() {
    let p = Path::new("tests/usaco/").join(
        Path::new(Path::new(file!()).file_prefix().unwrap()).with_extension("zip")
    );
    for (input, output) in helper::zip_io_pairs(p) {
        testcase(input, output);
    }
}

fn testcase(input: impl IntoSource, judge: impl IntoSource) {
    let mut input_src = input.into_source();
    input! {
        from &mut input_src,
        n: usize, q: usize,
        xy: [(i64, i64); n],
    };

    let r = |x| Ratio::new(x, 1);

    let mut x_min = r(i64::MAX);
    let mut x_max = r(i64::MIN);
    let mut hull_min = ConvexHullMin::new(r(-1<<32), r(1<<32));
    let mut hull_max = ConvexHullMax::new(r(-1<<32), r(1<<32));

    macro add($x:expr, $y:expr) {
        let (x, y) = (r($x), r($y));
        imin!(x_min, x);
        imax!(x_max, x);
        hull_max.insert(-x, y);
        hull_min.insert(-x, y);
    }
    for (x, y) in xy {
        add!(x, y);
    }

    let mut ans = vec![];
    for _ in 0..q {
        input! { from &mut input_src, t: u8 };
        match t {
            1 => {
                input! { from &mut input_src, x: i64, y: i64 };
                add!(x, y);
            }
            2 => {
                input! { from &mut input_src, a: i64, b: i64, c: i64 };
                if b == 0 {
                    // x = C/A
                    let x0 = Ratio::new(c, a);
                    ans.push(if x0 < x_min || x_max < x0 {"YES"} else {"NO"});
                }
                else {
                    // y = -A/B x + C/B
                    let m = Ratio::new(-a, b);
                    let y0 = Ratio::new(c, b);
                    if  hull_min.eval_at(m) > y0 ||
                        hull_max.eval_at(m) < y0
                    {
                        ans.push("YES");
                    } else {
                        ans.push("NO");
                    }
                }
            }
            _ => panic!()
        }
    }

    input! {
        from judge.into_source(),
        judge_ans: [String; ans.len()],
    };
    assert_eq!(ans, judge_ans);
}
