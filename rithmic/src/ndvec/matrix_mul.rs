use std::ops::{Add, Mul};

use super::NdVec;

impl<T> NdVec<2, T>
where T: Default + Copy + Mul<Output=T> + Add<Output=T>
{
    pub fn mat_mul(&self, rhs: &Self) -> Self {
        assert_eq!(self.shape[1], rhs.shape[0]);

        let mut a = NdVec::<2, T>::new([self.shape[0], rhs.shape[1]]);
        for i in 0..self.shape[0] {
            for j in 0..rhs.shape[1] {
                a[[i, j]] = (0..self.shape[1])
                    .map(|k| self[[i, k]] * rhs[[k, j]])
                    .reduce(T::add).unwrap();
            }
        }
        a
    }

    pub fn mat_vec_mul(&self, rhs: &[T]) -> Vec<T> {
        assert_eq!(self.shape[1], rhs.len());

        let mut v = vec![T::default(); self.shape[0]];
        for i in 0..self.shape[0] {
            v[i] = (0..self.shape[1])
                .map(|j| self[[i, j]] * rhs[j])
                .reduce(T::add).unwrap();
        }
        v
    }

    pub fn pow(&self, p: usize) -> Self {
        assert_eq!(self.shape[0], self.shape[1]);
        assert!(p >= 1);
        if p == 1 { return self.clone() }

        let mut u = self.pow(p>>1);
        u = u.mat_mul(&u);
        if p & 1 == 1 { u = u.mat_mul(self) }
        u
    }

    pub fn transposed(&self) -> Self {
        let mut t = Self::new([self.shape[1], self.shape[0]]);

        for i in 0..self.shape[0] {
            for j in 0..self.shape[1] {
                t[[j, i]] = self[[i, j]];
            }
        }
        t
    }
}
