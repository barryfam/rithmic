use std::mem;

/**
A [union-find](https://en.wikipedia.org/wiki/Disjoint-set_data_structure) data structure

An undirected graph supporting (1) edge addition, (2) queries on whether two nodes are in the same connected component, and (3) queries on the size of any node's component, each in amortized *O*(α(*N*)) time, which is effectively *O*(1)

# Examples
```
# use rithmic::UnionFind;
let mut uf = UnionFind::new(6);
# assert!(!uf.same(0, 1));
# assert_eq!(uf.size(2), 1);
uf.union(2, 3);
uf.union(3, 5);

assert!(uf.same(2, 5));
assert_eq!(uf.size(5), 3);
# assert_eq!(uf.size(2), 3);
# assert_eq!(uf.size(3), 3);
# assert!(!uf.union(2, 5));

# let r = uf.roots().collect::<Vec<_>>();
# assert_eq!(r.len(), 4);
# assert!(r.contains(&0));
# assert!(r.contains(&1));
# assert!(r.contains(&4));
# assert!(r.contains(&2) || r.contains(&3) || r.contains(&5));
#
assert_eq!(uf.components(), vec![
    vec![0],
    vec![1],
    vec![2, 3, 5],
    vec![4],
]);
```
*/
#[derive(Clone, Debug)]
pub struct UnionFind(Vec<isize>);

impl UnionFind {
    /// Construct a new [`UnionFind`] with `n` initially disconnected elements
    pub fn new(n: usize) -> Self {
        assert!(n <= isize::MAX as usize, "n too large: n={}, max={}", n, isize::MAX);
        Self(vec![-1; n])
    }

    /// Find the representative for `x`'s component
    pub fn find(&mut self, x: usize) -> usize {
        let mut r = x;
        while self.0[r] >= 0 {
            r = self.0[r] as usize;
        }
        let mut u = x;
        while self.0[u] >= 0 {
            let v = self.0[u] as usize;
            self.0[u] = r as isize;
            u = v;
        }
        r
    }

    /// Connect the components of `x` and `y`. Returns `true` if they were not already connected
    pub fn union(&mut self, x: usize, y: usize) -> bool
    {
        let (mut rx, mut ry) = (self.find(x), self.find(y));
        if rx == ry {
            return false
        }

        // if -self.0[rx] < -self.0[ry] {
        // this compiles to fewer instructions:
        if self.0[rx] > self.0[ry] {
            mem::swap(&mut rx, &mut ry);
        }
        self.0[rx] += self.0[ry];
        self.0[ry] = rx as isize;
        true
    }

    /// Return `true` if `x` and `y` are in the same component
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        if self.0[x] >= 0 && self.0[x] == self.0[y] {
            return true
        }
        self.find(x) == self.find(y)
    }

    /// Return the size of `x`'s component
    pub fn size(&mut self, x: usize) -> usize {
        let r = self.find(x);
        -self.0[r] as usize
    }

    /// Return the current component representatives
    pub fn roots(&mut self) -> impl Iterator<Item=usize> + '_ {
        (0..self.0.len()).filter(|&u| self.find(u) == u)
    }

    /// Return the current components. See [`UnionFind`] for an example
    pub fn components(&mut self) -> Vec<Vec<usize>>
    {
        let n = self.0.len();
        let mut cmpnts = vec![vec![]; n];
        for u in 0..n {
            cmpnts[self.find(u)].push(u);
        }
        cmpnts.into_iter().filter(|c| !c.is_empty()).collect()
    }

    /// Create a new [`UnionFind`] from component lists. Roughly the opposite of [`UnionFind::components`]
    pub fn from_iter_iter(n: usize, iter: impl IntoIterator<Item=impl IntoIterator<Item=usize>>) -> Self {
        let mut uf = Self::new(n);
        for u in iter {
            let mut u = u.into_iter();
            if let Some(x0) = u.next() {
                for x in u {
                    uf.union(x0, x);
                }
            }
        }
        uf
    }
}
