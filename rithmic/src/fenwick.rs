use std::{convert::TryInto, mem};
use std::ops::{AddAssign, SubAssign};

use itertools::Itertools;

use crate::{NdVec, Rangelike};

fn inc_lsb(i: usize) -> usize {
    i + (i & (!i + 1))
}

fn dec_lsb(i: usize) -> usize {
    i & (i-1)
}

type InclExcl<T> = (i8, T);
struct InclExclProduct {
    p: i8
}
impl Default for InclExclProduct {
    fn default() -> Self { Self { p: 1 } }
}
impl Extend<i8> for InclExclProduct {
    fn extend<T: IntoIterator<Item = i8>>(&mut self, iter: T) {
        for x in iter { self.p *= x }
    }
}

struct ArrayExtender<const D: usize> {
    a: [usize; D],
    i: usize
}
impl<const D: usize> Default for ArrayExtender<D> {
    fn default() -> Self {
        Self {
            a: [0; D],
            i: 0
        }
    }
}
impl<const D: usize> Extend<usize> for ArrayExtender<D> {
    fn extend<T: IntoIterator<Item = usize>>(&mut self, iter: T) {
        for x in iter {
            self.a[self.i] = x;
            self.i += 1;
        }
    }
}

// Iterate over the tree indexes affected by a change to source index i
#[derive(Clone)]
#[must_use]
struct DisperseIterator {
    i: usize,
    end: usize,
}
impl Iterator for DisperseIterator {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.i >= self.end {
            None
        }
        else if self.i == 0 {
            self.i = usize::MAX;
            Some(0)
        }
        else {
            let next_i = inc_lsb(self.i);
            Some(mem::replace(&mut self.i, next_i))
        }
    }
}

// Iterate over the tree indexes whose sum covers source range (j, i]. Note that j < i
#[derive(Clone)]
#[must_use]
struct AggregateIterator {
    and_0: bool,
    i: usize,
    j: usize
}
impl Iterator for AggregateIterator {
    type Item = InclExcl<usize>;

    fn next(&mut self) -> Option<InclExcl<usize>> {
        if self.and_0 {
            self.and_0 = false;
            Some((1, 0))
        }
        else if self.i > self.j {
            let next_i = dec_lsb(self.i);
            Some((1, mem::replace(&mut self.i, next_i)))
        }
        else if self.j > self.i {
            let next_j = dec_lsb(self.j);
            Some((-1, mem::replace(&mut self.j, next_j)))
        }
        else { None }
    }
}

#[derive(Clone, Debug)]
#[must_use]
pub struct Fenwick<T, const D: usize>
where T: Default + AddAssign + SubAssign
{
    tree: NdVec<D, T>
}

impl<T, const D: usize> Fenwick<T, D>
where T: Default + AddAssign + SubAssign
{
    pub fn new(shape: [usize; D]) -> Self {
        Self { tree: NdVec::<D, T>::new(shape) }
    }

    pub fn update<U>(&mut self, index: [usize; D], update: U)
    where
        T: AddAssign<U>,
    {
        for u in self.disperse(index) {
            self.tree.vec[u] += update;
        }
    }

    pub fn query<R>(&self, index: [R; D]) -> T
    where R: Rangelike<usize>
    {
        let index = index.into_ijk_ranges_of(&self.tree);

        let mut sum = T::default();
        for (ie, u) in aggregate(&index) {
            match ie {
                1 => sum += self.tree[u].clone(),
                -1 => sum -= self.tree[u].clone(),
                _ => unreachable!()
            }
        }

        sum
    }

    fn disperse(&self, index: [usize; D]) -> Vec<usize>
    {
        index.into_iter().zip(self.tree.shape.into_iter())
            .map(|(i, end)| DisperseIterator { i, end } )
            .multi_cartesian_product()
            .map(|v| self.tree.ravel(v))
            .collect_vec()
    }

    fn aggregate(&self, index: [usize; D]) -> impl Iterator<Item = InclExcl<IJK<D>>>
    {
        index.iter()
            .map(|r| AggregateIterator {
                and_0: r[0] == 0 && r[1] > 0,
                i: r[1].saturating_sub(1),
                j: r[0].saturating_sub(1)
            })
            .multi_cartesian_product()
            .map(|ie_ijk| -> InclExcl<IJK<D>> {
                let (ie, ijk): (InclExclProduct, ArrayExtender<D>) = ie_ijk.into_iter().unzip();
                (ie.p, ijk.a)
            })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut ft = Fenwick::<i64, 2>::new([3, 3]);
        ft.update([1, 1], 5);
        ft.update([2, 0], 30);
        assert_eq!(ft.query((..2, 1..3)), 5);
        assert_eq!(ft.query((.., ..)), 35);
        assert_eq!(ft.query((.., ..0)), 0);
    }
}
