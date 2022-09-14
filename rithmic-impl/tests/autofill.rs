#![feature(rustc_attrs)]

use rithmic_impl::autofill;

#[allow(clippy::only_used_in_recursion, clippy::ptr_arg)]
#[test]
fn test_dfs() {
    let n = 3_usize;
    let adj = vec![vec![1], vec![2], vec![]];
    let mut ans = vec![0; n];

    #[autofill(1..)]
    fn dfs(u: usize,
        n: usize, adj: &Vec<Vec<usize>>, ans: &mut Vec<usize>)
    {
        ans[u] = 42;

        for &v in &adj[u] {
            dfs!(v);
        }
    }

    dfs!(0);
    assert_eq!(ans[2], 42);
}

#[test]
fn test_expr() {
    let mut memo = vec![0, 1];

    #[autofill(1..)]
    fn f(x: usize, memo: &mut Vec<usize>) -> usize
    {
        if let Some(&y) = memo.get(x) { return y }

        let y = f!(x-2) + f!(x-1);

        assert_eq!(memo.len(), x);
        memo.push(y);
        y
    }

    assert_eq!(f!(10), 55);
}

#[test]
fn test_full_range() {
    let x = 3;

    #[autofill(..)]
    fn f(x: usize) -> usize {
        x * x
    }

    assert_eq!(f!(), 9);
}

// #[test]
// fn test_compile_errors()
// {
//     #[autofill(9..)]
//     fn f(x: usize) {}

//     #[autofill(..=9)]
//     fn f(x: usize) {}

//     #[autofill(9)]
//     fn f(x: usize) {}

//     #[autofill(9F9..)]
//     fn f(x: usize) {}

//     #[autofill(9.9..)]
//     fn f(x: usize) {}

//     #[autofill(0.., 9)]
//     fn f(x: usize) {}
// }
