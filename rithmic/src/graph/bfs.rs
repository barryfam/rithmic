use std::collections::VecDeque;

use super::*;

impl<const FLAGS: usize> Graph<(), FLAGS>
{
    pub fn bfs_path(&self, s: usize, t: usize) -> Option<Vec<usize>>
    {
        let mut pred = vec![NONE; self.size()];
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(u) = queue.pop_front() {
            if u == t {
                let mut path = vec![];
                let mut u = u;
                while u != s {
                    path.push(u);
                    u = pred[u];
                }
                path.push(s);
                path.reverse();
                return Some(path)
            }

            for &(v, _) in &self.adj[u] {
                if pred[v] == NONE {
                    pred[v] = u;
                    queue.push_back(v);
                }
            }
        }
        None
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

        assert_eq!(g.bfs_path(1, 4), Some(vec![1, 0, 4]));
        assert_eq!(g.bfs_path(0, 0), Some(vec![0]));
    }
}
