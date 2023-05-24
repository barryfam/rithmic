use std::fmt::Debug;
use std::ops::Range;

use itertools::Itertools;

use crate::{OrdPair, imax};

use super::prelude::*;

const UMASK: usize = !0 >> 1;
const UFLAG: usize = !UMASK;

impl<E> Tree<E>
where E: Copy
{
    pub fn heavy_light_map(&self) -> HeavyLightMap {
        let n = self.size();
        let mut size = vec![1; n];
        let mut heavy_child = vec![(0, 0); n];

        for (u, p, _) in self.dfs_up_tree(0) {
            size[p] += size[u];
            imax!(heavy_child[p], (size[u], u));
        }

        let mut map = vec![0; n];
        let mut i = n-1;
        let mut ptr = vec![0; n];

        let mut stack = vec![(0, 0)];
        while let Some((u, v)) = stack.pop() {
            map[v] = i;
            ptr[i] = map[u] | UFLAG;
            i = i.wrapping_sub(1);

            let mut wp = u;
            let mut w = v;
            while let hc = heavy_child[w] && hc.0 != 0
            {
                for &(x, _) in self.adj[w].iter().filter(|&&(x, _)| x != wp && x != hc.1) {
                    stack.push((w, x));
                }

                wp = w;
                w = hc.1;
                map[w] = i;
                ptr[i] = map[v];
                i = i.wrapping_sub(1);
            }
        }
        ptr.pop();
        debug_assert_eq!(ptr.len(), n-1);
        debug_assert_eq!(i, !0);

        HeavyLightMap { map, ptr }
    }
}

#[derive(Default, Clone, Debug)]
pub struct HeavyLightMap
{
    map: Vec<usize>,
    ptr: Vec<usize>,
}

impl HeavyLightMap {
    pub fn path(&self, u: usize, v: usize) -> impl Iterator<Item=Range<usize>> + Debug + '_
    {
        HeavyLightPath { hlm: self, i: self.map[u], j: self.map[v] }
    }

    pub fn edge(&self, u: usize, v: usize) -> usize
    {
        macro NON_ADJ() { "u and v are not adjacent" }

        let r = self.path(u, v).exactly_one().expect(NON_ADJ!());
        assert_eq!(r.len(), 1, NON_ADJ!());
        r.start
    }

    fn step(&self, i: usize) -> Step {
        let Some(&ptr) = self.ptr.get(i) else { return Step::None };

        if ptr & UFLAG == 0 {
            Step::Sum(ptr)
        } else {
            Step::Jump(ptr & UMASK)
        }
    }

    fn same_chain(&self, i: usize, j: usize) -> bool {
        debug_assert!(i < j);
        self.ptr[i] & UFLAG == 0 && j <= self.ptr[i]
    }
}

enum Step {
    None,
    Sum(usize),
    Jump(usize),
}

#[derive(Clone, Debug)]
struct HeavyLightPath<'me> {
    hlm: &'me HeavyLightMap,
    i: usize,
    j: usize,
}

impl<'me> Iterator for HeavyLightPath<'me> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let &mut Self { hlm, i, j } = self;
        let (i, j) = (i, j).ordered();

        if i == j {
            None
        }
        else if hlm.same_chain(i, j) {
            (self.i, self.j) = (j, j);
            Some(i..j)
        }
        else {
            match hlm.step(i) {
                Step::Sum(mut i1) => {
                    let i2 = match hlm.step(i1)
                    {
                        Step::Sum(_) => unreachable!(),
                        Step::Jump(i2) => {
                            i1 += 1;
                            i2
                        }
                        Step::None => {
                            i1
                        }
                    };
                    (self.i, self.j) = (i2, j);
                    Some(i..i1)
                }
                Step::Jump(i2) => {
                    (self.i, self.j) = (i2, j);
                    Some(i..i+1)
                }
                Step::None => unreachable!()
            }
        }
    }
}
