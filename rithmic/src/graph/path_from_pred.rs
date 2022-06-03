
pub fn path_from_pred<E>(s: usize, t: usize, dp: &[(E, usize)]) -> Vec<usize> {
    let mut path = vec![];

    let mut v = t;
    while v != s {
        path.push(v);
        v = dp[v].1;
    }
    path.push(s);
    path.reverse();
    path
}
