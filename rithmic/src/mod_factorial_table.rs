use std::{iter::Product, ops::{Div, Index, Mul}};

use ac_library::{ModInt, Modulus, StaticModInt};

pub trait Integer: Copy + Mul<Output = Self> + Div<Output = Self> + Product {
    fn new(x: usize) -> Self;
    fn pow(self, exp: usize) -> Self;
    const INVERTIBLE: bool;
    fn inv(self) -> Self;
}
impl Integer for usize {
    fn new(x: usize) -> Self { x }
    fn pow(self, exp: usize) -> Self { usize::pow(self, exp as u32) }
    const INVERTIBLE: bool = false;
    fn inv(self) -> Self { unreachable!() }
}
impl Integer for ModInt {
    fn new(x: usize) -> Self { ModInt::new(x) }
    fn pow(self, exp: usize) -> Self { self.pow(exp as u64) }
    const INVERTIBLE: bool = false;
    fn inv(self) -> Self { unreachable!() }
}
impl<M: Modulus> Integer for StaticModInt<M> {
    fn new(x: usize) -> Self { StaticModInt::raw(x as u32) }
    fn pow(self, exp: usize) -> Self { self.pow(exp as u64) }
    const INVERTIBLE: bool = true;
    fn inv(self) -> Self { StaticModInt::inv(self) }
}

pub struct FactorialTable<T>
{
    f: Vec<T>,
    f_inv: Option<Vec<T>>
}

impl<T> FactorialTable<T>
where T: Integer
{
    pub fn new(max_n: usize) -> Self
    {
        assert!(max_n < u32::MAX as usize);

        let mut f = Vec::with_capacity(max_n + 1);
        f.push(T::new(1));
        for x in 1..=max_n {
            let y = f[x-1] * T::new(x);
            f.push(y);
        }

        let f_inv = T::INVERTIBLE.then( ||
        {
            let mut f_inv = Vec::with_capacity(max_n + 1);
            f_inv.push(f[max_n].inv());
            for x in (1..=max_n).rev() {
                let y = *f_inv.last().unwrap() * T::new(x);
                f_inv.push(y);
            }
            f_inv.reverse();
            f_inv
        });

        Self { f, f_inv }
    }

    pub fn f(&self, n: usize) -> T {
        self.f[n]
    }

    pub fn f_inv(&self, n: usize) -> T {
        self.f_inv.as_ref().unwrap()[n]
    }

    pub fn perm(&self, n: usize, r: usize) -> T {
        if r > n { T::new(0) }
        else {
            if let Some(ref f_inv) = self.f_inv {
                self.f[n] * f_inv[n-r]
            } else {
                self.f[n] / self.f[n-r]
            }
        }
    }

    pub fn perm_with_rep(&self, n: usize, r: usize) -> T {
        T::new(n).pow(r)
    }

    pub fn comb(&self, n: usize, r: usize) -> T {
        if r > n { T::new(0) }
        else {
            if let Some(ref f_inv) = self.f_inv {
                self.f[n] * f_inv[r] * f_inv[n-r]
            } else {
                self.f[n] / self.f[r] / self.f[n-r]
            }
        }
    }

    pub fn comb_with_rep(&self, n: usize, r: usize) -> T {
        if n == 0 { T::new(0) }
        else {
            if let Some(ref f_inv) = self.f_inv {
                self.f[n+r-1] * f_inv[n-1] * f_inv[r]
            } else {
                self.f[n+r-1] / self.f[n-1] / self.f[r]
            }
        }
    }

    pub fn multinomial(&self, n: usize, k: &[usize]) -> T {
        debug_assert_eq!(k.iter().sum::<usize>(), n);

        if let Some(ref f_inv) = self.f_inv {
            let d = k.iter()
                .map(|&k| f_inv[k])
                .product::<T>();

            self.f[n] * d
        }
        else {
            let d = k.iter()
                .map(|&k| self.f[k])
                .product::<T>();

            self.f[n] / d
        }
    }
}

impl<T> Index<usize> for FactorialTable<T>
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.f[index]
    }
}


#[cfg(test)]
mod tests {
    use ac_library::ModInt1000000007;

    use super::*;

    type MInt = ModInt1000000007;

    #[test]
    fn test_basic() {
        let ft = FactorialTable::<MInt>::new(10);
        assert_eq!(ft[4], MInt::new(24));

        ModInt::set_modulus(17);
        let ft = FactorialTable::<ModInt>::new(10);
        assert_eq!(ft[4], ModInt::new(7));

        let ft = FactorialTable::<usize>::new(10);
        assert_eq!(ft[4], 24);
    }

    #[test]
    fn test_combinatorics() {
        let ft = FactorialTable::<MInt>::new(20);
        assert_eq!(ft.perm(16, 3), MInt::new(3360));
        assert_eq!(ft.perm_with_rep(10, 3), MInt::new(1000));
        assert_eq!(ft.comb(16, 3), MInt::new(560));
        assert_eq!(ft.comb_with_rep(5, 3), MInt::new(35));
        assert_eq!(ft.multinomial(11, &[1, 4, 4, 2]), MInt::new(34650));
    }

    #[test]
    fn test_edge_cases() {
        let ft = FactorialTable::<usize>::new(10);
        assert_eq!(ft.comb(0, 0), 1);
        assert_eq!(ft.comb(0, 7), 0);
        assert_eq!(ft.comb(1, 7), 0);
        assert_eq!(ft.comb(6, 7), 0);
        assert_eq!(ft.comb(7, 7), 1);
        assert_eq!(ft.comb(8, 7), 8);
        assert_eq!(ft.comb(7, 8), 0);
        assert_eq!(ft.comb(7, 0), 1);
        assert_eq!(ft.comb(7, 1), 7);
    }
}
