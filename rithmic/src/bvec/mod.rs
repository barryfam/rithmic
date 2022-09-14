#[cfg(test)] mod tests;

use std::fmt::{self, Debug, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not, Shl, ShlAssign, Shr, ShrAssign};

macro INDEX_RANGE () {"index out of range"}
macro UNEQUAL_LEN () {"bit operation on vectors of unequal length"}

const U_BITS: usize = usize::BITS as usize;

#[inline]
fn major(index: usize) -> usize {
    index / U_BITS
}
#[inline]
fn minor(index: usize) -> usize {
    index % U_BITS
}

/**
A compact boolean (bitset) vector with efficient `<<` `>>` shifting and `|` `^` `&` logical operators

8x smaller and ~20-500x faster for those operations than `Vec<bool>`

# Examples
Finding the [twin primes](https://en.wikipedia.org/wiki/Twin_prime) under 100:
```
# use rithmic::BVec;
let n = 100;
let mut primes = BVec::ones(n);

primes.set(0, false);
primes.set(1, false);
for i in 2..n {
    for j in (i*2..n).step_by(i) {
        primes.set(j, false);
    }
}

assert_eq!(primes[7], true);
# assert_eq!(primes.get(25), Some(false));
# assert_eq!(primes.first_one(), Some(2));
assert_eq!(primes.last_one(), Some(97));
# assert_eq!(primes.count_ones(), 25);
assert_eq!(primes.iter_ones().take(5).collect::<Vec<_>>(), vec![2, 3, 5, 7, 11]);
# assert_eq!(primes.first_zero(), Some(0));
# assert_eq!(primes.last_zero(), Some(99));
# assert_eq!(primes.count_zeros(), 75);
# assert_eq!(primes.iter_zeros().take(5).collect::<Vec<_>>(), vec![0, 1, 4, 6, 8]);

let twins1 = primes.clone() & (primes >> 2);
let twins2 = &twins1 << 2;

assert_eq!(twins1.iter_ones().collect::<Vec<_>>(), vec![3, 5, 11, 17, 29, 41, 59, 71]);
assert_eq!(twins2.iter_ones().collect::<Vec<_>>(), vec![5, 7, 13, 19, 31, 43, 61, 73]);
```
*/
#[derive(Default, Clone, PartialEq, Eq, Hash)]
pub struct BVec {
    vec: Vec<usize>,
    len: usize,
}

impl BVec {
    pub fn new(len: usize) -> Self {
        Self::zeros(len)
    }

    pub fn from_indexes(indexes: impl IntoIterator<Item=usize>, len: usize) -> Self {
        let mut bv = Self::new(len);
        for i in indexes {
            bv.set(i, true);
        }
        bv
    }

    pub fn zeros(len: usize) -> Self {
        let vec = vec![0; len.div_ceil(U_BITS)];
        Self{vec, len}
    }

    pub fn ones(len: usize) -> Self {
        let vec = vec![!0; len.div_ceil(U_BITS)];
        let mut bv = Self{vec, len};
        bv.top_off();
        bv
    }

    #[allow(clippy::len_without_is_empty)]
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<bool> {
        if index >= self.len { None }
        else { Some(self[index]) }
    }

    #[inline]
    pub fn set(&mut self, index: usize, b: bool)
    {
        assert!(index < self.len, INDEX_RANGE!());

        #[cfg(not(feature = "unsafe"))]
        if b {
            self.vec[major(index)] |= 1 << minor(index);
        } else {
            self.vec[major(index)] &= !(1 << minor(index));
        }

        #[cfg(feature = "unsafe")]
        // SAFETY: Asserted index < self.len. Invariant self.vec.len = self.len.div_ceil(U_BITS)
        unsafe {
            if b {
                *self.vec.get_unchecked_mut(major(index)) |= 1 << minor(index);
            } else {
                *self.vec.get_unchecked_mut(major(index)) &= !(1 << minor(index));
            }
        }
    }

    #[inline]
    pub fn replace(&mut self, index: usize, b: bool) -> bool {
        let ret = self[index];
        self.set(index, b);
        ret
    }

    #[inline]
    pub fn first(&self) -> Option<bool> {
        self.get(0)
    }

    #[inline]
    pub fn last(&self) -> Option<bool> {
        Some(self[self.len.checked_sub(1)?])
    }

    pub fn fill(&mut self, b: bool) {
        if b {
            self.vec.fill(!0);
            self.top_off();
        }
        else {
            self.vec.fill(0);
        }
    }

    pub fn flip(&mut self) {
        self.vec.iter_mut().for_each(|u| *u = !*u);
        self.top_off();
    }

    pub fn resize(&mut self, new_len: usize, b: bool) {
        if b && minor(self.len) != 0 {
            *self.vec.last_mut().unwrap() |= !0 << minor(self.len);
        }

        self.vec.resize(new_len.div_ceil(U_BITS), if b {!0} else {0});
        self.len = new_len;

        self.top_off();
    }

    pub fn any(&self) -> bool {
        self.vec.iter().any(|&u| u != 0)
    }

    pub fn all(&self) -> bool {
        self.first_zero().is_none()
    }

    pub fn first_one(&self) -> Option<usize> {
        for (i, &u) in self.vec.iter().enumerate() {
            if u != 0 {
                return Some(i * U_BITS + u.trailing_zeros() as usize)
            }
        }
        None
    }

    pub fn last_one(&self) -> Option<usize> {
        for (i, &u) in self.vec.iter().enumerate().rev() {
            if u != 0 {
                return Some(i * U_BITS + U_BITS - 1 - u.leading_zeros() as usize)
            }
        }
        None
    }

    pub fn first_zero(&self) -> Option<usize>
    {
        let mut fz = usize::MAX;
        for (i, &u) in self.vec.iter().enumerate() {
            if u != !0 {
                fz = i * U_BITS + u.trailing_ones() as usize;
                break
            }
        }
        (fz < self.len).then_some(fz)
    }

    pub fn last_zero(&self) -> Option<usize>
    {
        let mut i = self.vec.len().checked_sub(1)?;
        let mut u = self.vec[i];

        'z: {
            if minor(self.len) != 0 {
                u |= !0 << minor(self.len);
            }
            if u != !0 {
                break 'z
            }

            while i > 0 {
                i -= 1;
                u = self.vec[i];
                if u != !0 {
                    break 'z
                }
            }
            return None
        }
        Some(i * U_BITS + U_BITS - 1 - u.leading_ones() as usize)
    }

    pub fn count_ones(&self) -> usize {
        self.vec.iter().map(|u| u.count_ones() as usize).sum()
    }

    pub fn count_zeros(&self) -> usize {
        self.len - self.count_ones()
    }

    pub fn iter_ones(&self) -> Iter<true> {
        Iter::new(self)
    }

    pub fn iter_zeros(&self) -> Iter<false> {
        Iter::new(self)
    }

    pub fn iter_usize(&self) -> impl Iterator<Item=usize> + '_ {
        self.vec.iter().copied()
    }

    pub fn raw_vec(&self) -> &Vec<usize> {
        &self.vec
    }

    #[inline]
    fn top_off(&mut self) {
        if minor(self.len) != 0 {
            *self.vec.last_mut().unwrap() &= (1 << minor(self.len)) - 1;
        }
    }
}

pub struct Iter<'me, const P: bool> {
    bvec: &'me BVec,
    vec_i: usize,
    sub_iter: usize,
}
impl<'me, const P: bool> Iter<'me, P> {
    fn new(bvec: &'me BVec) -> Self {
        Iter::<P> {
            bvec,
            vec_i: 0,
            sub_iter: *bvec.vec.first().unwrap_or(&0) ^ if P {0} else {!0},
        }
    }
}
impl<'me, const P: bool> Iterator for Iter<'me, P> {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        while self.sub_iter == 0 {
            if self.vec_i + 1 < self.bvec.vec.len() {
                self.vec_i += 1;
                self.sub_iter = self.bvec.vec[self.vec_i] ^ if P {0} else {!0};
            }
            else {
                return None
            }
        }
        let tz = self.sub_iter.trailing_zeros();
        self.sub_iter ^= 1 << tz;

        let i = tz as usize + self.vec_i * U_BITS;
        if !P && i >= self.bvec.len { return None }
        Some(i)
    }
}

impl Index<usize> for BVec {
    type Output = bool;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output
    {
        assert!(index < self.len, INDEX_RANGE!());

        let u;
        { #![cfg(not(feature = "unsafe"))] u = self.vec[major(index)]; }

        // SAFETY: Asserted index < self.len. Invariant self.vec.len = self.len.div_ceil(U_BITS)
        { #![cfg(feature = "unsafe")] unsafe { u = self.vec.get_unchecked(major(index)); }}

        match u >> minor(index) &1==1 {
            true => &true,
            false => &false
        }
    }
}

impl ShrAssign<usize> for BVec {
    fn shr_assign(&mut self, rhs: usize) {
        let offset = major(rhs);
        let sr0 = minor(rhs);
        let sl1 = U_BITS - sr0;

        let v_len = self.vec.len();
        if sr0 == 0 {
            self.vec.copy_within(offset..v_len, 0);
        }
        else {
            #[cfg(not(feature = "unsafe"))]
            for i in offset..v_len {
                self.vec[i - offset] = (self.vec[i] >> sr0) + (self.vec.get(i+1).unwrap_or(&0) << sl1);
            }

            #[cfg(feature = "unsafe")]
            // SAFETY: offset <= i < v_len-1
            unsafe {
                let u = self.vec.as_mut_ptr();
                for i in offset..v_len-1 {
                    *u.add(i - offset) =
                        ((*u.add(i)) >> sr0) +
                        ((*u.add(i+1)) << (sl1));
                }
                if v_len > 0 {
                    (*u.add(v_len-1) = (*u.add(v_len-1)) >> sr0);
                }
            }
        }
        self.vec[v_len - offset ..].fill(0);
    }
}

impl Shr<usize> for BVec {
    type Output = BVec;

    fn shr(mut self, rhs: usize) -> Self::Output {
        self >>= rhs;
        self
    }
}

impl Shr<usize> for &BVec {
    type Output = BVec;

    fn shr(self, rhs: usize) -> Self::Output {
        self.clone() >> rhs
    }
}

impl ShlAssign<usize> for BVec {
    fn shl_assign(&mut self, rhs: usize) {
        let offset = major(rhs);
        let sl0 = minor(rhs);
        let sr1 = U_BITS - sl0;

        let v_len = self.vec.len();
        if sl0 == 0 {
            self.vec.copy_within(0..(v_len - offset), offset)
        }
        else {
            #[cfg(not(feature = "unsafe"))]
            for i in (0..(v_len - offset)).rev() {
                self.vec[i + offset] = (self.vec[i] << sl0) + if i > 0 { self.vec[i-1] >> sr1 } else { 0 };
            }

            #[cfg(feature = "unsafe")]
            // SAFETY: 1 <= i < v_len - offset
            unsafe {
                let u = self.vec.as_mut_ptr();
                for i in (1..(v_len - offset)).rev() {
                    *u.add(i + offset) =
                        ((*u.add(i)) << sl0) +
                        ((*u.add(i-1)) >> sr1);
                }
                if v_len - offset > 0 {
                    *u = (*u) << sl0;
                }
            }
        }
        self.vec[..offset].fill(0);
    }
}

impl Shl<usize> for BVec {
    type Output = BVec;

    fn shl(mut self, rhs: usize) -> Self::Output {
        self <<= rhs;
        self
    }
}

impl Shl<usize> for &BVec {
    type Output = BVec;

    fn shl(self, rhs: usize) -> Self::Output {
        self.clone() << rhs
    }
}

impl Not for BVec {
    type Output = BVec;

    fn not(mut self) -> Self::Output {
        self.flip();
        self
    }
}

impl Not for &BVec {
    type Output = BVec;

    fn not(self) -> Self::Output {
        !self.clone()
    }
}

macro impl_bitops {
    ($trait:ident, $method:ident, $op:tt, $a_trait:ident, $a_method:ident, $a_op:tt) => {
        impl $trait<BVec> for BVec {
            type Output = BVec;

            #[inline]
            fn $method(mut self, rhs: BVec) -> Self::Output {
                self $a_op rhs;
                self
            }
        }

        impl $trait<&BVec> for BVec {
            type Output = BVec;

            #[inline]
            fn $method(mut self, rhs: &BVec) -> Self::Output {
                self $a_op rhs;
                self
            }
        }

        impl $trait<BVec> for &BVec {
            type Output = BVec;

            #[inline]
            fn $method(self, mut rhs: BVec) -> Self::Output {
                rhs $a_op self;
                rhs
            }
        }

        impl $trait<&BVec> for &BVec {
            type Output = BVec;

            #[inline]
            fn $method(self, rhs: &BVec) -> Self::Output
            {
                assert_eq!(self.len, rhs.len, UNEQUAL_LEN!());

                #[cfg(not(feature = "unsafe"))]
                {
                    let mut vec = Vec::with_capacity(self.vec.len());
                    let mut i = 0;
                    vec.resize_with(self.vec.len(), || {
                        let u = self.vec[i] $op rhs.vec[i];
                        i += 1;
                        u
                    });
                    BVec { vec, len: self.len }
                }
                #[cfg(feature = "unsafe")]
                {
                    let n = self.vec.len();
                    let mut vec = Vec::<usize>::with_capacity(n);
                    let u = self.vec.as_ptr();
                    let v = rhs.vec.as_ptr();
                    let w = vec.as_mut_ptr();

                    // SAFETY: Asserted self.len == rhs.len. Invariant BVec.vec.len == self.len.div_ceil(U_BITS)
                    // SAFETY: Created `w` with `n` capacity
                    unsafe {
                        for i in 0..n {
                            *w.add(i) = *u.add(i) $op *v.add(i);
                        }
                        vec.set_len(n);
                    }
                    BVec { vec, len: self.len }
                }
            }
        }

        impl $a_trait<BVec> for BVec {
            #[inline]
            fn $a_method(&mut self, rhs: BVec) {
                self.$a_method(&rhs)
            }
        }

        impl $a_trait<&BVec> for BVec {
            #[inline]
            fn $a_method(&mut self, rhs: &BVec)
            {
                assert_eq!(self.len, rhs.len, UNEQUAL_LEN!());

                #[cfg(not(feature = "unsafe"))]
                {
                    for (u, &v) in self.vec.iter_mut().zip(rhs.vec.iter()) {
                        *u $a_op v;
                    }
                }
                #[cfg(feature = "unsafe")]
                {
                    let n = self.vec.len();
                    let u = self.vec.as_mut_ptr();
                    let v = rhs.vec.as_ptr();

                    // SAFETY: Asserted self.len == rhs.len. Invariant BVec.vec.len == self.len.div_ceil(U_BITS)
                    unsafe {
                        for i in 0..n {
                            *u.add(i) $a_op *v.add(i);
                        }
                    }
                }
            }
        }
    }
}

impl_bitops!(BitAnd, bitand, &, BitAndAssign, bitand_assign, &=);
impl_bitops!(BitOr , bitor , |, BitOrAssign , bitor_assign , |=);
impl_bitops!(BitXor, bitxor, ^, BitXorAssign, bitxor_assign, ^=);

impl Debug for BVec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
    {
        write!(f, "\n{:>w$}", "(rows ↓, columns ←):", w=U_BITS)?;

        for i in 0..self.vec.len()
        {
            let u = self.vec[i];
            if i == self.vec.len() - 1 && minor(self.len) != 0
            {
                let w = minor(self.len);
                let pad = U_BITS - w;
                write!(f, "\n{:pad$}{u:0w$b}", "")?;
            }
            else {
                write!(f, "\n{u:0w$b}", w=U_BITS)?;
            }
        }

        Ok(())
    }
}
