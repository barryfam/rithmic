#![feature(decl_macro, rustc_attrs)]

use rithmic_impl::struct_input;

#[test]
fn test_basic() {
    struct_input! {
        Case {
            n: usize,
            a: [i32; n-1],
            b: [[i32; n]; n],
            c: [i32],
            g: [(Usize1, Usize1); n],
            s: Chars,
            t: (usize, Bytes),
        }
    }

    let case_destruct!() = Case {
        n: 123,
        a: vec![3, 1, 4],
        b: vec![vec![0]],
        c: vec![2, 7, 1],
        g: vec![(0, 1), (2, 3)],
        s: vec!['a', 'b'],
        t: (0, vec![b'x', b'y']),
    };

    assert_eq!(n, 123);
    assert_eq!(a, vec![3, 1, 4]);
    assert_eq!(b, vec![vec![0]]);
    assert_eq!(c, vec![2, 7, 1]);
    assert_eq!(g, vec![(0, 1), (2, 3)]);
    assert_eq!(s, vec!['a', 'b']);
    assert_eq!(t, (0, vec![b'x', b'y']));
}
