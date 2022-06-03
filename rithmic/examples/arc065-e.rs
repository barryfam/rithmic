#![feature(decl_macro)]

mod helper;

use std::path::Path;

use itertools::chain;
use proconio::marker::Usize1;
use proconio::input;
use rithmic::SortedList;

fn main() {
    for (input, output) in helper::dir_io_pairs("examples/arc065-e") {
        testcase(input, output);
    }
}

fn testcase(input: impl AsRef<Path>, output: impl AsRef<Path>) {
    input! {
        from helper::source_from_path(input),
        n: usize, a: Usize1, b: Usize1,
        xy: [(i64, i64); n]
    };
    let mut xy: Vec<_> = xy.into_iter().map(|(x, y)| (x-y, x+y)).collect();

    let r =  (xy[a].0 - xy[b].0).abs()
        .max((xy[a].1 - xy[b].1).abs());

    let mut by_x = SortedList::from_iter(xy.iter().copied());
    let mut by_y = SortedList::from_iter(xy.iter().map(|&(x, y)| (y, x)));

    let mut stack = vec![];
    macro push($u:expr) {
        let (x, y) = $u;
        by_x.remove(&(x, y));
        by_y.remove(&(y, x));
        stack.push((x, y));
    }
    push!(xy[a]);

    while let Some((x, y)) = stack.pop() {
        let (x0, x1) = (x-r, x+r);
        let (y0, y1) = (y-r, y+r);

        for u in chain!(
            by_x.range((x0, y0)..=(x0, y1)).copied().collect::<Vec<_>>(),
            by_x.range((x1, y0)..=(x1, y1)).copied().collect::<Vec<_>>(),
            by_y.range((y0, x0)..=(y0, x1)).map(|&(y, x)| (x, y)).collect::<Vec<_>>(),
            by_y.range((y1, x0)..=(y1, x1)).map(|&(y, x)| (x, y)).collect::<Vec<_>>(),
        ) {
            push!(u);
        }
    }

    xy.retain(|u| !by_x.contains(u));

    let mut by_x = SortedList::from_iter(xy.iter().copied());
    let mut by_y = SortedList::from_iter(xy.iter().map(|&(x, y)| (y, x)));

    let mut ans = 0;
    for (x, y) in xy {
        let (x0, x1) = (x-r, x+r);
        let (y0, y1) = (y-r, y+r);

        ans += by_x.range_len((x0, y0)..(x0, y1));
        ans += by_y.range_len((y1, x0)..(y1, x1));
    }

    input! {
        from helper::source_from_path(output),
        judge_ans: usize
    };
    assert_eq!(ans, judge_ans);
}
