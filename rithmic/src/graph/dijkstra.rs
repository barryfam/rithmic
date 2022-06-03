use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};
use std::ops::Add;

use crate::BVec;

use super::*;

impl<E, const FLAGS: usize> Graph<E, FLAGS>
where E: Copy + Default + Add<Output=E> + Ord
{
    pub fn dijkstra(&self, s: usize, t: usize) -> Option<E>
    {
        let mut queue = BinaryHeap::new();
        let mut visited = BVec::new(self.size());

        queue.push((Reverse(E::default()), s));
        while let Some((d, u)) = queue.pop()
        {
            if visited[u] { continue }
            visited.set(u, true);

            if u == t {
                return Some(d.0)
            }

            for &(v, e) in &self.adj[u] {
                let d = d.0 + e;
                queue.push((Reverse(d), v));
            }
        }
        None
    }
}

impl<const FLAGS: usize> Graph<bool, FLAGS>
{
    pub fn dijkstra_01(&self, s: usize, t: usize) -> Option<usize>
    {
        let mut queue = VecDeque::new();
        let mut visited = vec![0_u8; self.size()];  // 1 = queue-back, 2 = queue-front, 3 = visited

        queue.push_front((0, s));
        while let Some((d, u)) = queue.pop_front()
        {
            if visited[u] == 3 { continue }
            visited[u] = 3;

            if u == t {
                return Some(d)
            }

            for &(v, e) in &self.adj[u] {
                match (visited[v], e) {
                    (0 | 1, false) => {
                        queue.push_front((d, v));
                        visited[v] = 2;
                    }
                    (0, true) => {
                        queue.push_back((d+1, v));
                        visited[v] = 1;
                    }
                    _ => {}
                }
            }
        }
        None
    }
}
