#![feature(test)]

#![feature(decl_macro)]

extern crate test;

use paste::paste;
use rand::prelude::*;
use rand::rngs::mock::StepRng;
use rithmic::BVec;
use test::Bencher;

const N: usize = 500_000;
const M: usize = 100;

trait Adapter {
    fn set_i(&mut self, i: usize, b: bool);
    fn get_i(&self, i: usize) -> bool;

    fn shift_xor(&mut self, s: usize);
}

impl Adapter for BVec {
    #[inline]
    fn set_i(&mut self, i: usize, b: bool) {
        self.set(i, b);
    }
    #[inline]
    fn get_i(&self, i: usize) -> bool {
        self[i]
    }
    fn shift_xor(&mut self, s: usize) {
        *self ^= self.clone() << s;
    }
}

impl Adapter for Vec<bool> {
    #[inline]
    fn set_i(&mut self, i: usize, b: bool) {
        self[i] = b;
    }
    #[inline]
    fn get_i(&self, i: usize) -> bool {
        self[i]
    }
    fn shift_xor(&mut self, s: usize) {
        for i in s..N {
            self[i-s] ^= self[i];
        }
    }
}

fn set_get_m(u: &mut impl Adapter, seeds: (u64, u64)) -> bool
{
    let mut rng = StepRng::new(seeds.0, seeds.1);

    let mut x = false;
    for _ in 0..M {
        let i = rng.gen_range(0..N);
        u.set_i(i, rng.gen());

        let i = rng.gen_range(0..N);
        x ^= u.get_i(i);
    }
    x
}

macro bench($name:ident, $new:expr) {
    paste! {
        #[bench]
        fn [<bench_bvec_get_set_ $name>](b: &mut Bencher) {
            let mut rng = SmallRng::from_entropy();
            let mut u = $new;
            b.iter(|| set_get_m(&mut u, rng.gen()))
        }

        #[bench]
        fn [<bench_bvec_shift_xor_ $name>](b: &mut Bencher) {
            let mut rng = SmallRng::from_entropy();
            let mut u = $new;
            for i in 0..N {
                u.set_i(i, rng.gen());
            }
            b.iter(|| u.shift_xor(rng.gen_range(N/3..2*N/3)))
        }
    }
}

bench!(bvec, BVec::new(N));
bench!(vec_bool, vec![false; N]);
