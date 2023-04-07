#![feature(array_windows)]

mod helper;

use std::collections::BTreeSet;
use std::path::Path;

use proconio::input;

use helper::IntoSource;
use proconio::marker::Usize1;
use rithmic::graph::SimpDirGraph;

#[test] fn abc218_f() { main() }

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
        n: usize, m: usize,
        st: [(Usize1, Usize1); m],
    };
    let g = SimpDirGraph::<()>::from_iter_unweighted(n, st.iter().copied());

    let mut ans = vec![-1; m];
    let p = g.bfs([0], [n-1]).path();
    if let Some(p) = p {
        let p_edges: BTreeSet<[usize; 2]> = p.array_windows().copied().collect();

        for i in 0..m {
            let &(s, t) = &st[i];
            if !p_edges.contains(&[s, t]) {
                ans[i] = (p.len()-1) as isize;
            }
            else {
                let h = SimpDirGraph::<()>::from_iter_unweighted(n,
                    st.iter().copied().filter(|&(u, v)| (u, v) != (s, t))
                );
                let p0 = h.bfs([0], [n-1]).path();
                if let Some(p0) = p0 {
                    ans[i] = (p0.len()-1) as isize;
                }
            }
        }
    }

    input! {
        from judge.into_source(),
        judge_ans: [isize; m],
    };
    assert_eq!(ans, judge_ans);
}
