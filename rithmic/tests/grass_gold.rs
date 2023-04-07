#![feature(option_zip)]
#![feature(path_file_prefix)]

mod helper;

use std::ops::Add;
use std::path::Path;

use proconio::input;

use helper::IntoSource;
use proconio::marker::Usize1;
use rithmic::OptionMerge;
use rithmic::graph::{SimpDirGraph, SIMPLE, ACYCLIC};

#[test] fn usaco_grass_gold() { main() }

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
        n: usize, m: usize,
        xy: [(Usize1, Usize1); m],
    };

    let g = SimpDirGraph::from_iter_unweighted(n, xy.iter().copied());
    let sccs = g.tarjan_scc();
    const SA: usize = SIMPLE | ACYCLIC;
    let g1 = g.map::<SA>(sccs.len(), |u| sccs.map(u));
    let h1 = g1.rev();

    let s1 = sccs.map(0);
    let mut ans = sccs[s1].len();

    let mut dp_g1 = vec![None; g1.size()];
    dp_g1[s1] = Some(0);
    for u1 in 0..g1.size() {
        let Some(dpu) = dp_g1[u1] else { continue };

        for &(v1, _) in &g1.adj[u1] {
            dp_g1[v1] = Some(dp_g1[v1].merge_or(dpu + sccs[v1].len(), usize::max));
        }
    }

    let mut dp_h1 = vec![None; h1.size()];
    dp_h1[s1] = Some(0);
    for u1 in (0..h1.size()).rev() {
        let Some(dpu) = dp_h1[u1] else { continue };

        for &(v1, _) in &h1.adj[u1] {
            dp_h1[v1] = Some(dp_h1[v1].merge_or(dpu + sccs[v1].len(), usize::max));
        }
    }

    ans += xy.into_iter()
        .filter_map(|(u, v)| {
            let (u1, v1) = (sccs.map(u), sccs.map(v));
            dp_g1[v1].zip_with(dp_h1[u1], usize::add)
        })
        .max().unwrap_or(0);

    input! {
        from judge.into_source(),
        judge_ans: usize,
    };
    assert_eq!(ans, judge_ans);
}
