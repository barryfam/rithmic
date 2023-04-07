use super::*;

use itertools::Itertools;

impl<T, const MAX: bool> ConvexHullTrick<T, MAX>
where T: IntFloatOrRatio
{
    fn self_check(&self)
    {
        #[allow(unused_variables)]
        for (((m0, b0), (i0, j0)), ((m1, b1), (i1, j1))) in self.iter().tuple_windows()
        {
            assert!(i0 < j0);
            assert!(j0 == i1);
            assert!(i1 < j1);

            assert!(MAX ^ (m0 > m1));
            assert!(MAX ^ (m0*i1 + b0 >= m1*i1 + b1));
        }
    }
}

#[test]
fn test_basic() {
    let mut cht = ConvexHullMin::new(0, 5);

    assert!(cht.is_empty());
    assert_eq!(cht.len(), 0);

    cht.insert(2, 0);
    cht.self_check();
    assert_eq!(cht.len(), 1);
    assert_eq!(cht.eval_at(0), 0);
    assert_eq!(cht.eval_at(1), 2);
    assert_eq!(cht.eval_at(2), 4);
    assert_eq!(cht.eval_at(3), 6);
    assert_eq!(cht.eval_at(4), 8);

    cht.insert(-1, 5);
    cht.self_check();
    assert_eq!(cht.len(), 2);
    assert_eq!(cht.eval_at(0), 0);
    assert_eq!(cht.eval_at(1), 2);
    assert_eq!(cht.eval_at(2), 3);
    assert_eq!(cht.eval_at(3), 2);
    assert_eq!(cht.eval_at(4), 1);

    cht.insert(0, 1);
    cht.self_check();
    assert_eq!(cht.len(), 3);
    assert_eq!(cht.eval_at(0), 0);
    assert_eq!(cht.eval_at(1), 1);
    assert_eq!(cht.eval_at(2), 1);
    assert_eq!(cht.eval_at(3), 1);
    assert_eq!(cht.eval_at(4), 1);

    cht.insert(0, 1);
    cht.self_check();
    assert_eq!(cht.len(), 3);

    cht.insert(3, 0);
    cht.self_check();
    assert_eq!(cht.len(), 3);

    cht.insert(-2, 11);
    cht.self_check();
    assert_eq!(cht.len(), 3);

    assert_eq!(cht.eval_at(0), 0);
    assert_eq!(cht.eval_at(1), 1);
    assert_eq!(cht.eval_at(2), 1);
    assert_eq!(cht.eval_at(3), 1);
    assert_eq!(cht.eval_at(4), 1);

    cht.insert(-1, 1);
    cht.self_check();
    assert_eq!(cht.len(), 2);
    assert_eq!(cht.eval_at(0), 0);
    assert_eq!(cht.eval_at(1), 0);
    assert_eq!(cht.eval_at(2), -1);
    assert_eq!(cht.eval_at(3), -2);
    assert_eq!(cht.eval_at(4), -3);
}
