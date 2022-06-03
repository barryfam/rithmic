#![feature(test)]

#![feature(decl_macro)]

extern crate test;

use paste::paste;
use rand::prelude::*;
use rand::rngs::mock::StepRng;
use rithmic::NdVec;
use test::Bencher;

const N: usize = 700;
const M: usize = 100;

trait Adapter {
    fn set_ij(&mut self, i: usize, j: usize, x: usize);
    fn get_ij(&self, i: usize, j: usize) -> usize;
}

impl Adapter for NdVec<2, usize> {
    #[inline]
    fn set_ij(&mut self, i: usize, j: usize, x: usize) {
        self[[i, j]] = x;
    }
    #[inline]
    fn get_ij(&self, i: usize, j: usize) -> usize {
        self[[i, j]]
    }
}

impl Adapter for Vec<Vec<usize>> {
    #[inline]
    fn set_ij(&mut self, i: usize, j: usize, x: usize) {
        self[i][j] = x;
    }
    #[inline]
    fn get_ij(&self, i: usize, j: usize) -> usize {
        self[i][j]
    }
}

impl Adapter for ndarray::Array2<usize> {
    #[inline]
    fn set_ij(&mut self, i: usize, j: usize, x: usize) {
        self[[i, j]] = x;
    }
    #[inline]
    fn get_ij(&self, i: usize, j: usize) -> usize {
        self[[i, j]]
    }
}

fn set_get_m(u: &mut impl Adapter, seeds: (u64, u64)) -> usize
{
    let mut rng = StepRng::new(seeds.0, seeds.1);

    let mut x = 0;
    for _ in 0..M {
        let i = rng.gen_range(0..N);
        let j = rng.gen_range(0..N);
        u.set_ij(i, j, rng.gen());

        let i = rng.gen_range(0..N);
        let j = rng.gen_range(0..N);
        x ^= u.get_ij(i, j);
    }
    x
}

macro bench($name:ident, $new:expr) {
    paste! {
        #[bench]
        fn [<bench_ndvec_get_set_ $name>](b: &mut Bencher) {
            let mut rng = SmallRng::from_entropy();
            let mut u = $new;
            b.iter(|| set_get_m(&mut u, rng.gen()))
        }
    }
}

bench!(ndvec, NdVec::<2, usize>::new([N, N]));
bench!(vec_vec, vec![vec![0_usize; N]; N]);
bench!(ndarray, ndarray::Array2::<usize>::zeros((N, N)));
