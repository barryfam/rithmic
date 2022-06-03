use super::*;

#[test]
fn test_add() {
    let f = [1, 2, 3];
    let g = [3, 5, 7];

    assert_eq!(polynomial_add(&f, &g), vec![4, 7, 10]);
}

#[test]
fn test_mul() {
    let f = [1, 2, 3];
    let g = [3, 5, 7];

    assert_eq!(polynomial_mul(&f, &g), vec![3, 11, 26, 29, 21]);
}

#[test]
fn test_div() {
    let f = [3, 11, 26, 29, 21];
    let g = [3, 5, 7];

    assert_eq!(polynomial_div(&f, &g), (vec![1, 2, 3], vec![]));

    let f = [-4, 0, -2, 1];
    let g = [-3, 1];

    assert_eq!(polynomial_div(&f, &g), (vec![3, 1, 1], vec![5]));
}

#[test]
fn test_types() {
    let f = [-4., 0., -2., 1.];
    let g = [-3., 1.];

    assert_eq!(polynomial_div(&f, &g), (vec![3., 1., 1.], vec![5.]));

    let f = [3_usize, 11, 26, 29, 21];
    let g = [3, 5, 7];

    assert_eq!(polynomial_div(&f, &g), (vec![1, 2, 3], vec![]));
}
