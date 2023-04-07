#[cfg(test)] mod tests;

/// This trait provides methods for working with unsigned integers at the bit level
pub trait IntBitOps
where Self: Sized
{
    /// Returns the size in bits of this integer's type
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// assert_eq!(0b_0001_0000_u8.bit_width(), 8);
    /// ```
    fn bit_width(self) -> u32;

    /// Returns the minimum number of bits needed to represent this integer
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// assert_eq!(0b_0001_0000_u8.bit_length(), 5);
    /// ```
    fn bit_length(self) -> u32;

    /// Returns an integer with the lowest `len` bits set
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// assert_eq!(u8::mask(5), 0b_0001_1111);
    /// ```
    fn mask(len: u32) -> Self;

    /// Returns `self & `[`mask`](Self::mask)`(len)`
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// assert_eq!(0b_1101_0011_u8.masked(5), 0b_0001_0011);
    /// ```
    fn masked(self, len: u32) -> Self;

    /// Returns the least-significant-bit of this integer, isolated
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// assert_eq!(0b_0111_0100_u8.lsb(), 0b_0000_0100);
    /// ```
    fn lsb(self) -> Self;

    /// Returns the most-significant-bit of this integer, isolated
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// assert_eq!(0b_0111_0100_u8.msb(), 0b_0100_0000);
    /// ```
    fn msb(self) -> Self;

    /// Iterates over each set bit of this integer, isolated, from lowest to highest
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// let u = 0b_11010_u8.iter_lsb().collect::<Vec<u8>>();
    ///
    /// assert_eq!(u, vec![
    ///     0b00010,
    ///     0b01000,
    ///     0b10000,
    /// ]);
    /// ```
    fn iter_lsb(self) -> IterLsb<Self>;

    /// Iterates over each set bit of this integer, isolated, from highest to lowest
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// let u = 0b_11010_u8.iter_msb().collect::<Vec<u8>>();
    ///
    /// assert_eq!(u, vec![
    ///     0b10000,
    ///     0b01000,
    ///     0b00010,
    /// ]);
    /// ```
    fn iter_msb(self) -> IterMsb<Self>;

    /// Iterates over the index of each set bit of this integer
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// let u = 0b_11010_u8.iter_ones().collect::<Vec<usize>>();
    ///
    /// assert_eq!(u, vec![1, 3, 4]);
    /// ```
    fn iter_ones(self) -> IterOnes<Self>;

    /// Iterates over every subset of the set bits of this integer
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// let u = 0b11010_u8.iter_subsets().collect::<Vec<u8>>();
    ///
    /// assert_eq!(u, vec![
    ///     0b00000,
    ///     0b00010,
    ///     0b01000,
    ///     0b01010,
    ///     0b10000,
    ///     0b10010,
    ///     0b11000,
    ///     0b11010,
    /// ]);
    /// ```
    fn iter_subsets(self) -> IterSubsets<Self>;

    /// Let *k* be the number of bits set in this integer; then iterate over every integer with *k*+1 bits set that is a superset of this one and has length not greater than `n`
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// let u = 0b_0001_1010_u8.iter_add_one(7).collect::<Vec<u8>>();
    ///
    /// assert_eq!(u, vec![
    ///     0b_0001_1011,
    ///     0b_0001_1110,
    ///     0b_0011_1010,
    ///     0b_0101_1010,
    /// ]);
    /// ```
    fn iter_add_one(self, n: u32) -> IterAddOne<Self>;

    /// "Gosper's Hack" - Iterate over every integer with exactly `k` set bits and length not greater than `n`
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// let u = u8::iter_gosper(5, 3).collect::<Vec<u8>>();
    ///
    /// assert_eq!(u, vec![
    ///     0b00111,
    ///     0b01011,
    ///     0b01101,
    ///     0b01110,
    ///     0b10011,
    ///     0b10101,
    ///     0b10110,
    ///     0b11001,
    ///     0b11010,
    ///     0b11100,
    /// ]);
    /// ```
    fn iter_gosper(n: u32, k: u32) -> IterGosper<Self>;

    /// "Gosper's Hack" but restricted to subsets of this integer - Iterate over every subset that has exactly `k` bits set
    ///
    /// # Examples
    /// ```
    /// use rithmic::IntBitOps;
    ///
    /// let u = 0b_1101_1010_u8.iter_gosper_subsets(3).collect::<Vec<u8>>();
    ///
    /// assert_eq!(u, vec![
    ///     0b_0001_1010,
    ///     0b_0100_1010,
    ///     0b_0101_0010,
    ///     0b_0101_1000,
    ///     0b_1000_1010,
    ///     0b_1001_0010,
    ///     0b_1001_1000,
    ///     0b_1100_0010,
    ///     0b_1100_1000,
    ///     0b_1101_0000,
    /// ]);
    /// ```
    fn iter_gosper_subsets(self, k: u32) -> IterGosperSubsets<Self>;
}

#[derive(Clone, Copy, Debug)]
pub struct IterLsb<T> {
    remaining: T
}

#[derive(Clone, Copy, Debug)]
pub struct IterMsb<T> {
    remaining: T
}

#[derive(Clone, Copy, Debug)]
pub struct IterOnes<T> {
    remaining: T
}

#[derive(Clone, Copy, Debug)]
pub struct IterSubsets<T> {
    set: T,
    next: T,
    fused: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct IterAddOne<T> {
    set: T,
    subset: T,
    used: T,
}

#[derive(Clone, Copy, Debug)]
pub struct IterGosper<T> {
    last: T,
    next: T,
    fused: bool,
}

pub struct IterGosperSubsets<T> {
    set: T,
    gosper: IterGosper<T>,
}

macro impl_bitops {
    ($type:ty) =>
    {
        impl IntBitOps for $type
        {
            #[inline]
            fn bit_width(self) -> u32 {
                Self::BITS
            }

            #[inline]
            fn bit_length(self) -> u32 {
                Self::BITS - self.leading_zeros()
            }

            #[inline]
            fn mask(len: u32) -> Self {
                if len == 0 { return 0 }
                !0 >> Self::BITS - len
            }

            #[inline]
            fn masked(self, len: u32) -> Self {
                self & Self::mask(len)
            }

            #[inline]
            fn lsb(self) -> Self {
                self & (!self).wrapping_add(1)
            }

            #[inline]
            fn msb(self) -> Self {
                if self == 0 { return 0 }
                1 << Self::BITS - 1 >> self.leading_zeros()
            }

            fn iter_lsb(self) -> IterLsb<Self> {
                IterLsb { remaining: self }
            }

            fn iter_msb(self) -> IterMsb<Self> {
                IterMsb { remaining: self }
            }

            fn iter_ones(self) -> IterOnes<Self> {
                IterOnes { remaining: self }
            }

            fn iter_subsets(self) -> IterSubsets<Self> {
                IterSubsets {
                    set: self,
                    next: 0,
                    fused: false,
                }
            }

            fn iter_add_one(self, n: u32) -> IterAddOne<Self> {
                debug_assert_ne!(n, 0);
                debug_assert!(self.bit_length() <= n);

                IterAddOne {
                    set: Self::mask(n),
                    subset: self,
                    used: self
                }
            }

            fn iter_gosper(n: u32, k: u32) -> IterGosper<Self> {
                debug_assert_ne!(k, 0);
                debug_assert!(k <= n);
                debug_assert!(n <= Self::BITS);

                IterGosper {
                    last: Self::mask(k) << (n-k),
                    next: Self::mask(k),
                    fused: false,
                }
            }

            fn iter_gosper_subsets(self, k: u32) -> IterGosperSubsets<Self> {
                debug_assert_ne!(k, 0);
                debug_assert!(k <= self.count_ones());

                IterGosperSubsets {
                    set: self,
                    gosper: Self::iter_gosper(self.count_ones(), k),
                }
            }
        }

        impl Iterator for IterLsb<$type> {
            type Item = $type;

            #[inline]
            fn next(&mut self) -> Option<Self::Item>
            {
                let r = self.remaining;
                if r == 0 { return None }

                let lsb = r.lsb();
                self.remaining = r ^ lsb;
                Some(lsb)
            }
        }

        impl Iterator for IterMsb<$type> {
            type Item = $type;

            #[inline]
            fn next(&mut self) -> Option<Self::Item>
            {
                let r = self.remaining;
                if r == 0 { return None }

                let msb = r.msb();
                self.remaining = r ^ msb;
                Some(msb)
            }
        }

        impl Iterator for IterOnes<$type> {
            type Item = usize;

            #[inline]
            fn next(&mut self) -> Option<Self::Item>
            {
                let r = self.remaining;
                if r == 0 { return None }

                let tz = r.trailing_zeros();
                self.remaining = r ^ 1 << tz;
                Some(tz as usize)
            }
        }

        impl Iterator for IterSubsets<$type> {
            type Item = $type;

            #[inline]
            fn next(&mut self) -> Option<Self::Item>
            {
                let Self{set: s, next: u, fused} = *self;
                if fused { return None }

                if u == s { self.fused = true; }
                else { self.next = (u | !s) + 1 & s; }
                Some(u)
            }
        }

        impl Iterator for IterAddOne<$type> {
            type Item = $type;

            #[inline]
            fn next(&mut self) -> Option<Self::Item>
            {
                let Self{set: s, subset: u, used: v} = *self;
                if v == s { return None }

                let w = (v+1) | u;
                self.used |= w;
                Some(w)
            }
        }

        impl Iterator for IterGosper<$type> {
            type Item = $type;

            #[inline]
            fn next(&mut self) -> Option<Self::Item>
            {
                let Self{last, next: u, fused} = *self;
                if fused { return None }

                if u == last { self.fused = true; }
                else {
                    let lsb = u.lsb();
                    let v = u + lsb;
                    self.next = (((v^u) >> 2) / lsb) | v;
                }
                Some(u)
            }
        }

        impl Iterator for IterGosperSubsets<$type> {
            type Item = $type;

            #[inline]
            fn next(&mut self) -> Option<Self::Item>
            {
                let g = self.gosper.next()?;

                let mut u = 0;
                for (i, v) in self.set.iter_lsb().enumerate() {
                    if g >> i &1==1 {
                        u |= v;
                    }
                }
                Some(u)
            }
        }
    },

    ( $( $type:ty ),+ ) => {
        $( impl_bitops!($type); )*
    },
}

impl_bitops!(usize, u128, u64, u32, u16, u8);
