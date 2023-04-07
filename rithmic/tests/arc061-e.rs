mod helper;

use std::path::Path;

use ahash::AHashMap;
use proconio::input;

use helper::IntoSource;
use proconio::marker::Usize1;
use rithmic::graph::{UndirGraph, DirGraph};

#[test] fn arc061_e() { main() }

fn main() {
    for (input, output) in helper::dir_io_pairs(
        Path::new("tests/")
            .join(Path::new(Path::new(file!()).file_stem().unwrap())
    )) {
        testcase(input, output);
    }
}

fn testcase(input: impl IntoSource, judge: impl IntoSource) {
    input! {
        from input.into_source(),
        n: usize, m: usize,
        pqc: [(Usize1, Usize1, u32); m]
    };
    let g = UndirGraph::<u32>::from_iter(n, pqc);

    let mut h = DirGraph::<i32>::new(n + 2*m);
    let mut pc_u = AHashMap::<(usize, u32), usize>::new();
    let mut u_next = n;

    for p in 0..n {
        for &(q, c) in &g.adj[p] {
            let u = *pc_u.entry((p, c)).or_insert(u_next);
            if u == u_next {
                h.add_edge(p, u, 1);
                h.add_edge(u, p, 0);
                u_next += 1;
            }
            let v = *pc_u.entry((q, c)).or_insert(u_next);
            if v == u_next {
                h.add_edge(q, v, 1);
                h.add_edge(v, q, 0);
                u_next += 1;
            }
            h.add_edge(u, v, 0);
        }
    }

    let ans = if let Some(ans) = h.dijkstra([0], [n-1]).dist() {
        ans as isize
    } else {
        -1
    };

    input! {
        from judge.into_source(),
        judge_ans: isize
    };
    assert_eq!(ans, judge_ans);
}
