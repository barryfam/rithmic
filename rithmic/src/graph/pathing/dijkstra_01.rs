use std::collections::VecDeque;

use crate::BVec;

use super::super::*;
use super::DistPred;

impl<const FLAGS: usize> Graph<bool, FLAGS>
{
    pub fn dijkstra_01(&self, s: impl IntoIterator<Item=usize>, t: impl IntoIterator<Item=usize>) -> DistPred<usize>
    {
        // in the queue, use the high bit of the usize to flag 1-edges (vs 0-edges)
        const UMASK: usize = !0 >> 1;
        const UFLAG: usize = !UMASK;
        assert!(self.size() <= UFLAG, "graph too large: size={}, max={}", self.size(), UFLAG);

        let mut t0 = NONE;
        let t = BVec::from_indexes(t.into_iter().inspect(|&t| t0 = t), self.size());
        let mut t_count = t.count_ones();
        if t_count == 0 { t_count = usize::MAX; }
        if t_count != 1 { t0 = NONE; }

        let mut dp = vec![(!0, NONE); self.size()];
        // dp[u] states:
        //      (!0, !0)    initial/unvisited
        //      (!0,  p)    touched by a 1-edge from p => mutate if 0-edge found
        //      ( d,  p)    touched by a 0-edge from p, or finalized from queue => do not mutate

        let mut queue = VecDeque::new();

        for s in s {
            dp[s] = (0, s);
            queue.push_back(s);
        }
        while let Some(u) = queue.pop_front() {
            let (e, u) = (u & UFLAG != 0, u & UMASK);

            if e {
                if dp[u].0 != !0 {
                    continue
                } else {
                    dp[u].0 = dp[dp[u].1].0 + 1;
                }
            }
            debug_assert!(dp[u].0 != !0 && dp[u].1 != NONE);

            if t[u] {
                t_count -= 1;
                if t_count == 0 {
                    break
                }
            }

            for &(v, e) in &self.adj[u]
            {
                if !e && dp[v].0 == !0 {
                    dp[v] = (dp[u].0, u);
                    queue.push_front(v);
                }
                else if e && dp[v].1 == NONE {
                    dp[v].1 = u;
                    queue.push_back(v | UFLAG);
                }
            }
        }

        for (d, p) in &mut dp {
            if *d == !0 {
                *p = NONE;
            }
        }
        DistPred::new(dp, t0)
    }
}
