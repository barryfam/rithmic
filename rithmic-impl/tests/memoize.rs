#![feature(rustc_attrs)]

use rithmic_impl::memoize;

#[test]
fn test_static()
{
    let s = vec![1, 4, 7, 10, 13];
    let mut calls = 0;

    #[memoize[5, 2]]
    fn f(i: usize, j: bool, s: &[u32], calls: &mut usize) -> u32 {
        *calls += 1;
        if i == 0 { 1 }
        else if !j { s[i] }
        else { f!(i-1, true) + f!(i, false) }
    }

    assert_eq!(f!(2, true), 12);
    assert_eq!(f!(3, true), 22);
    assert_eq!(f!(4, true), 35);
    assert_eq!(calls, 9);
}

#[test]
fn test_dynamic()
{
    let s = vec![1, 4, 7, 10, 13];
    let mut calls = 0;

    #[memoize(..2)]
    fn f(i: usize, j: bool, s: &[u32], calls: &mut usize) -> u32 {
        *calls += 1;
        if i == 0 { 1 }
        else if !j { s[i] }
        else { f!(i-1, true) + f!(i, false) }
    }

    assert_eq!(f!(2, true), 12);
    assert_eq!(f!(3, true), 22);
    assert_eq!(f!(4, true), 35);
    assert_eq!(calls, 9);
}

#[test]
fn test_expr_dims()
{
    let n = 5;

    #[memoize[n+1]]
    fn f(i: usize) -> usize {
        if i == 0 { 100 }
        else { f!(i-1) * 3 }
    }
}

#[test]
fn test_clone_static()
{
    #[memoize[10]]
    fn f(x: i32) -> Vec<i32> {
        vec![x, x*x]
    }

    assert_eq!(f!(3), vec![3, 9]);
    assert_eq!(f!(3), vec![3, 9]);
}

#[test]
fn test_clone_dynamic()
{
    #[memoize(..1)]
    fn f(x: i32) -> Vec<i32> {
        vec![x, x*x]
    }

    assert_eq!(f!(3), vec![3, 9]);
    assert_eq!(f!(3), vec![3, 9]);
}

#[test]
fn test_closure_capture()
{
    let h = Box::new(|x: usize| x*1000);

    #[memoize[10]]
    fn f<H: Fn(usize) -> usize>(y: usize, h: H) -> usize {
        if y == 0 {
            7
        } else {
            h(y) + f!(y-1)
        }
    }

    assert_eq!(f!(7), 28007);
}
