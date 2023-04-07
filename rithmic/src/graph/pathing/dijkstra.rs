use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

use crate::BVec;

use super::super::*;
use super::DistPred;

impl<E, const FLAGS: usize> Graph<E, FLAGS>
where E: Copy + Default + Add<Output=E> + Ord
{
    pub fn dijkstra(&self, s: impl IntoIterator<Item=usize>, t: impl IntoIterator<Item=usize>) -> DistPred<E>
    {
        let mut t0 = NONE;
        let t = BVec::from_indexes(t.into_iter().inspect(|&t| t0 = t), self.size());
        let mut t_count = t.count_ones();
        if t_count == 0 { t_count = usize::MAX; }
        if t_count != 1 { t0 = NONE; }

        let mut dp = vec![(E::default(), NONE); self.size()];
        let mut queue = BinaryHeap::new();

        for s in s {
            dp[s].1 = s;
            queue.push((Reverse(E::default()), s));
        }
        while let Some((Reverse(du), u)) = queue.pop()
        {
            if dp[u].0 != du { continue }

            if t[u] {
                t_count -= 1;
                if t_count == 0 {
                    break
                }
            }

            for &(v, e) in &self.adj[u]
            {
                let dv = du + e;
                if dp[v].1 == NONE || dv < dp[v].0
                {
                    dp[v] = (dv, u);
                    queue.push((Reverse(dv), v));
                }
            }
        }
        DistPred::new(dp, t0)
    }
}
