#[cfg(test)] mod tests;

pub trait IntBitOps
where Self: Sized
{
    fn bit_width(self) -> u32;
    fn bit_length(self) -> u32;
    fn mask(len: u32) -> Self;
    fn masked(self, len: u32) -> Self;
    fn lsb(self) -> Self;
    fn msb(self) -> Self;
    fn iter_lsb(self) -> IterLsb<Self>;
    fn iter_msb(self) -> IterMsb<Self>;
    fn iter_ones(self) -> IterOnes<Self>;
    fn iter_subsets(self) -> IterSubsets<Self>;
    fn iter_add_one(self, n: u32) -> IterAddOne<Self>;
    fn iter_gosper(n: u32, k: u32) -> IterGosper<Self>;
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
