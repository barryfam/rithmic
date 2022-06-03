/*!
Given a set of binary integers, determine if some other integer can be created by the XOR of any subset. `insert` and `contains` are both *O*(*d*) operations, where *d* is the bit length of the integers

# Examples
```
# use rithmic::XorBasis8;
let mut basis = XorBasis8::new();

assert!( basis.insert(  0b0000_0001));
assert!( basis.insert(  0b0000_0101));

assert!( basis.contains(0b0000_0100));
assert!(!basis.contains(0b0000_0010));

# assert!( basis.insert(  0b1000_0000));
# assert!( basis.insert(  0b1111_1111));
# assert!(!basis.insert(  0b1000_0100));
# assert!( basis.contains(0b0111_1010));
```
*/

#[rustc_macro_transparency = "transparent"]
macro impl_xor_basis($name:ident, $t:ty, $n:literal)
{
    /// See [`module-level documentation`](crate::xor_basis) for more information
    pub struct $name([$t; $n]);

    impl $name
    {
        #[inline]
        pub fn new() -> Self {
            Self([0; $n])
        }

        #[inline]
        fn reduce(&self, mut u: $t) -> $t {
            while u != 0 {
                let i = u.leading_zeros() as usize;
                let b = self.0[i];
                if b == 0 {
                    break
                }
                u ^= b;
            }
            u
        }

        /// If `u` is independent of the basis, add it and return `true`
        #[inline]
        pub fn insert(&mut self, u: $t) -> bool {
            let u = self.reduce(u);
            if u != 0 {
                self.0[u.leading_zeros() as usize] = u;
                true
            } else {
                false
            }
        }

        /// Return `true` if `u` is **not** independent of the basis
        #[inline]
        pub fn contains(&self, u: $t) -> bool {
            self.reduce(u) == 0
        }
    }

    impl Default for $name {
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }
}

impl_xor_basis!(XorBasis8  , u8  , 8  );
impl_xor_basis!(XorBasis16 , u16 , 16 );
impl_xor_basis!(XorBasis32 , u32 , 32 );
impl_xor_basis!(XorBasis64 , u64 , 64 );
impl_xor_basis!(XorBasis128, u128, 128);
