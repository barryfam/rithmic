use super::*;

#[test]
fn test_0_shift() {
    let u = BVec::from_indexes([2, 3, 107], 128);
    assert_eq!(u.clone() >> 0, u);
    assert_eq!(u.clone() << 0, u);
}

#[test]
fn test_ref_op_ref() {
    let u = BVec::from_indexes([2, 3, 5], 10);
    let v = BVec::from_indexes([0, 2, 4, 6, 8], 10);
    let w = &u ^ &v;
    assert_eq!(w, u ^ v);
}

#[cfg(target_pointer_width = "64")]
#[test]
fn test_debug() {
    use indoc::indoc;

    let u = BVec::from_indexes([3, 62, 66, 127, 129], 132);
    assert_eq!(format!("{u:?}"), indoc!("
        \n                                            (rows ↓, columns ←):
        0100000000000000000000000000000000000000000000000000000000001000
        1000000000000000000000000000000000000000000000000000000000000100
                                                                    0010"
    ));
}
