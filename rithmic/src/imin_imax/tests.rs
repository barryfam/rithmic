use crate::imax;

#[test]
fn test_syntax()
{
    let mut x = 7;
    imax!(x, 3, 5);
    assert_eq!(x, 7);
    imax!(x; 3, 11, 5);
    assert_eq!(x, 11);
    imax!(x, 13, );
    assert_eq!(x, 13);
}

#[test]
fn test_entry() {
    use std::collections::BTreeMap;

    let mut m = BTreeMap::from([('x', 1)]);
    imax!(*m.entry('x').or_default(), 9);
    assert_eq!(m[&'x'], 9);

    let mut m = BTreeMap::from([('x', vec![1])]);
    imax!(m.entry('x').or_default()[0], 9);
    assert_eq!(m[&'x'][0], 9);
}
