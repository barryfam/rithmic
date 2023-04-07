use std::collections::VecDeque;

use crate::BVec;

use super::super::*;
use super::DistPred;

impl<const FLAGS: usize> Graph<(), FLAGS>
{
    pub fn bfs(&self, s: impl IntoIterator<Item=usize>, t: impl IntoIterator<Item=usize>) -> DistPred<usize>
    {
        let mut t0 = NONE;
        let t = BVec::from_indexes(t.into_iter().inspect(|&t| t0 = t), self.size());
        let mut t_count = t.count_ones();
        if t_count == 0 { t_count = usize::MAX; }
        if t_count != 1 { t0 = NONE; }

        let mut dp = vec![(0, NONE); self.size()];
        let mut queue = VecDeque::new();

        for s in s {
            dp[s] = (0, s);
            queue.push_back(s);
        }
        while let Some(u) = queue.pop_front() {
            if t[u] {
                t_count -= 1;
                if t_count == 0 {
                    break
                }
            }

            for &(v, _) in &self.adj[u] {
                if dp[v].1 == NONE {
                    dp[v] = (dp[u].0 + 1, u);
                    queue.push_back(v);
                }
            }
        }
        DistPred::new(dp, t0)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_path() {
        let mut g = UndirGraph::<()>::new(5);
        g.add_edge(0, 1, ());
        g.add_edge(1, 2, ());
        g.add_edge(2, 3, ());
        g.add_edge(3, 0, ());
        g.add_edge(4, 0, ());

        assert_eq!(g.bfs([1], [4]).path(), Some(vec![1, 0, 4]));
        assert_eq!(g.bfs([0], [0]).path(), Some(vec![0]));
        assert_eq!(g.bfs([0], []).dist_to(2), Some(2));
        assert_eq!(g.bfs([0], []).dist_to(3), Some(1));
    }
}
