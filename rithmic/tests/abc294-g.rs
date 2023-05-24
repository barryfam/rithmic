mod helper;

use std::path::Path;

use proconio::input;

use helper::IntoSource;
use proconio::marker::Usize1;
use rithmic::SegTree;
use rithmic::graph::Tree;

#[test] fn abc294_g() { main() }

fn main() {
    for (input, output) in helper::dir_io_pairs(
        Path::new("tests/")
            .join(Path::new(Path::new(file!()).file_stem().unwrap()))
    ) {
        testcase(input, output);
    }
}

fn testcase(input: impl IntoSource, judge: impl IntoSource) {
    let mut input_src = input.into_source();
    input! {
        from &mut input_src,
        n: usize,
        uvw: [(Usize1, Usize1, u64); n-1],
        q: usize,
    };
    let g = Tree::from_iter(n, uvw.iter().copied());

    let hlm = g.heavy_light_map();
    let mut st = SegTree::<u64>::new(n-1);
    for &(u, v, w) in &uvw {
        st.set(hlm.edge(u, v), w);
    }

    let mut ans = vec![];
    for _ in 0..q {
        input! { from &mut input_src, t: u8 };
        match t {
            1 => {
                input! { from &mut input_src, i: Usize1, w: u64 };
                let (u, v, _) = uvw[i];
                st.set(hlm.edge(u, v), w);
            }
            2 => {
                input! { from &mut input_src, u: Usize1, v: Usize1 };
                ans.push(
                    hlm.path(u, v).map(|r| st.query(r)).sum::<u64>()
                );
            }
            _ => panic!()
        }
    }

    assert!(!ans.is_empty());
    input! {
        from judge.into_source(),
        judge_ans: [u64; ans.len()],
    };
    assert_eq!(ans, judge_ans);
}
