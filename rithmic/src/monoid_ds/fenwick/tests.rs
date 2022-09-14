use itertools::Itertools;
use rand::distributions::Uniform;
use rand::prelude::*;

use super::*;

#[test]
fn test_1d() {
    let mut a = NdFenwick::<1, u32>::build_from(&NdVec::from_raw([5], vec![2, 3, 5, 7, 11]));

    assert_eq!(a.query([0]), 2);
    assert_eq!(a.query([1]), 3);
    assert_eq!(a.query([..3]), 10);
    assert_eq!(a.query([..]), 28);

    a.update([0], 13);
    a.update([1], 15);

    assert_eq!(a.query([..3]), 38);
    assert_eq!(a.query([2..]), 23);
    assert_eq!(a.query([..]), 56);
}

#[test]
fn test_3d() {
    const N: usize = 5;
    const M: usize = 1000;

    let mut u = vec![0_u8; N*N*N];
    thread_rng().fill(&mut u[..]);
    let u = u.into_iter().map(u32::from).collect_vec();
    let mut a = NdVec::from_raw([N, N, N], u);

    fn brute(a: &NdVec<3, u32>, lower: [usize; 3], upper: [usize; 3]) -> u32 {
        let mut sum = 0;
        for i in lower[0]..upper[0] {
            for j in lower[1]..upper[1] {
                for k in lower[2]..upper[2] {
                    sum += a[[i, j, k]];
                }
            }
        }
        sum
    }

    let mut b = NdFenwick::<3, u32>::build_from(&a);

    let index_range = Uniform::from(0..N);
    for _ in 0..M {
        let index = [(); 3].map(|_| index_range.sample(&mut thread_rng()));
        let value = random::<u8>() as u32;

        let upper = index.map(|i| i+1);
        let lower = index.map(|i| Uniform::from(0..=i).sample(&mut thread_rng()));
        let range = lower.zip(upper);

        a[index] += value;
        b.update(index, value);

        assert_eq!(brute(&a, [0; 3], upper), b.prefix(upper));

        assert_eq!(brute(&a, lower, upper), b.query(range));
    }
}
