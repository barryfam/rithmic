use std::ops::Add;

use crate::NdVec;

use super::*;

impl<E, const FLAGS: usize> Graph<E, FLAGS>
where E: Copy + Default + Add<Output=E> + PartialOrd
{
    pub fn floyd_warshall(&self) -> NdVec<2, Option<(E, usize)>> {
        let n = self.size();
        let mut dp = NdVec::<2, Option<(E, usize)>>::new([n, n]);

        for u in 0..n {
            for &(v, e) in &self.adj[u] {
                if dp[[u, v]] == None || dp[[u, v]].unwrap().0 > e {
                    dp[[u, v]] = Some((e, u));
        }}}

        for v in 0..n {
            for u in 0..n {
                for w in 0..n {
                    if  let Some((uv, _pv)) = dp[[u, v]] &&
                        let Some((vw,  pw)) = dp[[v, w]]
                    {
                        let uvw = uv + vw;
                        if dp[[u, w]] == None || dp[[u, w]].unwrap().0 > uvw {
                            dp[[u, w]] = Some((uvw, pw));
        }}}}}

        dp
    }
}
