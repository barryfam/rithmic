use super::*;

#[test]
fn test_brute() {
    const N: usize = 50;
    for i in 0..N {
        for j in i..N {
            for k in 1..N {
                let sum = (i..j).step_by(k).sum::<usize>();
                assert_eq!(sum, triangular_steps(i..j, k));
            }
        }
    }
}

#[test]
fn test_u128() {
    assert_eq!(triangular_n(100_u128), 5050_u128);
    assert_eq!(triangular_n(1_u128 << 63), (1<<125) + (1<<62));
}
