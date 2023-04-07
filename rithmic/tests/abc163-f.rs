mod helper;

use std::path::Path;

use proconio::input;
use proconio::marker::Usize1;
use rithmic::graph::prelude::*;

use helper::IntoSource;

#[test] fn abc163_f() { main() }

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
        c: [Usize1; n],
        ab: [(Usize1, Usize1); n-1]
    };
    let g = Tree::<()>::from_iter_unweighted(n, ab);
    let st_sizes = g.subtree_sizes_rel_to(0);

    let mut ans = vec![n*(n+1)/2; n];
    let mut stacks = vec![vec![n]; n];

    for step in g.dfs([0]) {
        let DfsStep { kind, p, u, .. } = step;

        match kind {
            TreeNodeStart => {
                if p != NONE {
                    stacks[c[p]].push(st_sizes[u]);
                }
                *stacks[c[u]].last_mut().unwrap() -= st_sizes[u];
            }
            TreeNodeFinish => {
                if p != NONE {
                    let sz = stacks[c[p]].pop().unwrap();
                    ans[c[p]] -= sz*(sz+1)/2;
                }
            }
            _ => panic!()
        }
    }

    for k in 0..n {
        let sz = stacks[k].pop().unwrap();
        debug_assert!(stacks[k].is_empty());
        ans[k] -= sz*(sz+1)/2;
    }

    input! {
        from judge.into_source(),
        judge_ans: [usize; n]
    };
    assert_eq!(ans, judge_ans);
}
