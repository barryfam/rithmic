use std::ops::Add;

use num::Bounded;

use crate::{NdVec, imin, IntBitOps};

use super::Graph;

impl<E, const FLAGS: usize> Graph<E, FLAGS>
where E: Copy + Default + Add<Output=E> + PartialOrd + Bounded
{
    pub fn tsp(&self, s: impl IntoIterator<Item=usize>) -> NdVec<2, E> {
        let n = self.size();
        let mut dp = NdVec::full([1<<n, n], E::max_value());

        for s in s.into_iter() {
            dp[[1<<s, s]] = E::default();
        }
        for visited in 1..=usize::mask(n as u32) {
            for u in 0..n {
                if dp[[visited, u]] == E::max_value() { continue }
                for &(v, e) in &self.adj[u] {
                    imin!(dp[[visited | 1<<v, v]], dp[[visited, u]] + e);
                }
            }
        }

        dp
    }
}
