use itertools::Itertools;
use rand::prelude::*;

use super::*;

#[test]
fn test_3d() {
    const N: usize = 5;

    let mut u = vec![0_u8; N*N*N];
    thread_rng().fill(&mut u[..]);
    let u = u.into_iter().map(u32::from).collect_vec();
    let a = NdVec::from_raw([N, N, N], u);

    let ps = a.prefix_sums();
    let ss = a.suffix_sums();

    for x in 0..=N {
    for y in 0..=N {
    for z in 0..=N {
        let mut ps1 = 0;
        for i in 0..x {
        for j in 0..y {
        for k in 0..z {
            ps1 += a[[i, j, k]];
        }}}

        let mut ss1 = 0;
        for i in x..N {
        for j in y..N {
        for k in z..N {
            ss1 += a[[i, j, k]];
        }}}

        assert_eq!(ps1, ps[[x, y, z]]);
        assert_eq!(ss1, ss[[x, y, z]]);
    }}}
}
