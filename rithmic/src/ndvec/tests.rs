use super::*;

#[test]
fn test_display_debug() {
    use indoc::indoc;

    let mut a = NdVec::<3, usize>::new([3, 4, 5]);
    for i in 0..3 {
        for j in 0..4 {
            for k in 0..5 {
                a[[i, j, k]] = i*100 + j*10 + k;
            }
        }
    }

    assert_eq!(format!("{}", a), indoc!("
        0 1 2 3 4
        10 11 12 13 14
        20 21 22 23 24
        30 31 32 33 34
        100 101 102 103 104
        110 111 112 113 114
        120 121 122 123 124
        130 131 132 133 134
        200 201 202 203 204
        210 211 212 213 214
        220 221 222 223 224
        230 231 232 233 234
    "));

    assert_eq!(format!("{:?}", a), indoc!("
        \n[0, 1, 2, 3, 4]
        [10, 11, 12, 13, 14]
        [20, 21, 22, 23, 24]
        [30, 31, 32, 33, 34]
        -
        [100, 101, 102, 103, 104]
        [110, 111, 112, 113, 114]
        [120, 121, 122, 123, 124]
        [130, 131, 132, 133, 134]
        -
        [200, 201, 202, 203, 204]
        [210, 211, 212, 213, 214]
        [220, 221, 222, 223, 224]
        [230, 231, 232, 233, 234]"
    ));
}

#[cfg(debug_assertions)]
#[test]
#[should_panic]
fn test_oob() {
    let a = NdVec::<2, usize>::new([3, 5]);
    let _x = a[[0, 7]];
}
