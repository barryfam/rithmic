#![feature(path_file_prefix)]

mod helper;

use std::path::Path;

use std::cmp::Reverse;

use proconio::input;
use rithmic::graph::SimpUndirGraph;

use helper::IntoSource;

#[test] fn usaco_superbull_silver() { main() }

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
        n: usize,
        id: [u32; n],
    };

    let mut g = SimpUndirGraph::new(n);
    for u in 0..n {
        for v in u+1..n {
            g.add_edge(u, v, Reverse(id[u] ^ id[v]));
        }
    }

    let ans = g.mst()
        .all_edges().map(|(_, _, e)| e.0 as u64).sum::<u64>();

    input! {
        from judge.into_source(),
        judge_ans: u64,
    };
    assert_eq!(ans, judge_ans);
}
