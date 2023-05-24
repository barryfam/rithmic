use rithmic_impl::CmpFromOrd;

#[test]
fn test_basic()
{
    #[derive(CmpFromOrd, Clone, Copy)]
    struct S(f64, f64);

    impl S {
        fn len2(&self) -> f64 {
            self.0*self.0 + self.1*self.1
        }
    }

    impl Ord for S {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.len2().partial_cmp(&other.len2()).unwrap()
        }
    }

    let u = S(2.0, 3.5);
    let v = S(5.1, 0.7);

    assert!(u < v);
    assert!(u == u.clone());
}
