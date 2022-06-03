use std::ops::{Index, IndexMut};

use super::NdVec;

impl<T> NdVec<2, T>
where T: Clone + From<u8>
{
    pub fn eye(n: usize) -> Self
    {
        let mut a = Self::full([n, n], 0.into());
        for i in 0..n {
            a[[i, i]] = 1.into();
        }
        a
    }
}

impl<T> Index<usize> for NdVec<2, T>
{
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        let m = self.shape[1];
        let i = index * m;
        &self.vec[i..i+m]
    }
}

impl<T> IndexMut<usize> for NdVec<2, T>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let m = self.shape[1];
        let i = index * m;
        &mut self.vec[i..i+m]
    }
}
