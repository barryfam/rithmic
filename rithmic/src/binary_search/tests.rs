use super::*;

#[test]
fn test_vec() {
    let a = vec![3, 5, 7, 11, 13, 17, 19, 23];
    assert_eq!(binary_search(0..a.len(), true, |i| a[i] >= 18), Some(6));
    assert_eq!(binary_search(0..a.len(), true, |i| a[i] >= 23), Some(7));
    assert_eq!(binary_search(0..a.len(), true, |i| a[i] >= 99), None);
}

#[test]
fn test_f() {
    assert_eq!(binary_search(-100..100, true , |x| x/2 >= 21), Some(42));
    assert_eq!(binary_search(100..-100, false, |x| x/2 >= 21), Some(41));
    assert_eq!(binary_search(100..-100, true , |x| x*2 <= 84), Some(42));
    assert_eq!(binary_search(-100..100, false, |x| x*2 <= 84), Some(43));
}

#[test]
fn test_rev() {
    #![allow(unused_comparisons)]
    #![allow(clippy::absurd_extreme_comparisons)]

    assert_eq!(binary_search(100..0_u32, false, |x| x >= 2), Some(1));
    assert_eq!(binary_search(100..0_u32, false, |x| x >= 1), None);
    assert_eq!(binary_search(100..0_u32, false, |x| x >= 0), None);

    assert_eq!(binary_search(100..=0_u32, false, |x| x >= 2), Some(1));
    assert_eq!(binary_search(100..=0_u32, false, |x| x >= 1), Some(0));
    assert_eq!(binary_search(100..=0_u32, false, |x| x >= 0), None);
}
