mod helper;

use std::path::Path;

use proconio::input;

use helper::IntoSource;
use proconio::marker::{Usize1, Chars};
use rithmic::graph::SimpDirGraph;

#[test] fn abc246_e() { main() }

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
        a: (Usize1, Usize1),
        b: (Usize1, Usize1),
        s: [Chars; n]
    };

    let mut g = SimpDirGraph::<bool>::new(n*n*5);
    let u = |i, j, k| {
        k*n*n + i*n + j
    };

    let p = (a.0 + a.1) & 1;
    for i in 0..n {
        for j in (i&1^p..n).step_by(2) {
            if s[i][j] == '#' { continue }

            for k in 0b00..=0b11
            {
                g.add_edge(u(i, j, 4), u(i, j, k), true);
                g.add_edge(u(i, j, k), u(i, j, 4), false);

                let (i, j) = (i as isize, j as isize);
                let (vi, vj) = match k {
                    0b00 => (i+1, j+1),
                    0b01 => (i+1, j-1),
                    0b10 => (i-1, j+1),
                    0b11 => (i-1, j-1),
                    _ => unreachable!()
                };
                if (0..n as isize).contains(&vi) && (0..n as isize).contains(&vj)
                    && s[vi as usize][vj as usize] != '#'
                {
                    g.add_edge(u(i as usize, j as usize, k), u(vi as usize, vj as usize, k), false);
                }
            }
        }
    }

    let ans = if let Some(ans) = g.dijkstra_01([u(a.0, a.1, 4)], [u(b.0, b.1, 4)]).dist() {
        ans as isize
    } else {
        -1
    };

    input! {
        from judge.into_source(),
        judge_ans: isize,
    };
    assert_eq!(ans, judge_ans);
}
