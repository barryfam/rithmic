use rithmic_impl::CmpByKey;

#[test]
fn test_named() {
    #[derive(CmpByKey)]
    struct S<T> {
        a: T,
        #[key] x: i32,
        #[key] y: i32,
    }

    let a = S {x: 0, y: 5, a: 3.1};
    let b = S {x: 1, y: 2, a: f64::INFINITY};
    let c = S {x: 1, y: 3, a: f64::NAN};
    let d = S {x: 2, y: 0, a: 0.0};

    let mut v = vec![d, a, c, b];
    v.sort();
    assert_eq!(v[0].a, 3.1);
    assert_eq!(v[1].a, f64::INFINITY);
    assert!(v[2].a.is_nan());
    assert_eq!(v[3].a, 0.0);
}

#[test]
fn test_unnamed() {
    #[derive(CmpByKey)]
    struct S(#[key] i32, u64);

    assert!(S(0, 3) == S(0, 7));
    assert!(S(3, 0) != S(7, 0));
}
