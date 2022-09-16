use derive_more::Deref;
use itertools::Itertools;

/**
Index compress a sequence of length *n* by mapping every value to its ordered rank

The compressed sequence is therefore guaranteed to have all its elements be integers less than *n*

The returned [`IndexCompressed`] implements
- `Deref<Target=[usize]>` to access the compressed sequence
- `.compress(&x)` to look up the rank of value `x`
- `.decompress(y)` to look up the value of rank `y`

# Examples
```
use rithmic::IndexCompress;

let x = [777, 9001, -40, -40, 777, -273];
let y = x.index_compress();

assert_eq!(&*y, &[2, 3, 1, 1, 2, 0]);
# assert_eq!(y[0], 2);
assert_eq!(y.compress(&-40), 1);
assert_eq!(y.decompress(2), &777)
```
*/
pub trait IndexCompress {
    type T;
    fn index_compress(&self) -> IndexCompressed<Self::T>;
}

impl<T: Ord> IndexCompress for [T] {
    type T = T;
    fn index_compress(&self) -> IndexCompressed<T> {
        let mut y = vec![0; self.len()];
        let mut yx = vec![];

        for (x, i) in self.iter().zip(0..).sorted_unstable() {
            if yx.last() != Some(&x) {
                yx.push(x);
            }
            y[i] = yx.len() - 1;
        }
        IndexCompressed { y, yx }
    }
}

/// This `struct` is created by [`IndexCompress::index_compress()`](IndexCompress). See its documentation for more
#[derive(Default, Clone, Deref, Debug)]
pub struct IndexCompressed<'me, T>
{
    #[deref] y: Vec<usize>,
    yx: Vec<&'me T>,
}

impl<'me, T: Ord> IndexCompressed<'me, T>
{
    #[inline]
    pub fn compress(&self, x: &T) -> usize {
        self.yx.binary_search(&x).expect("value was not present during compression")
    }
    #[inline]
    pub fn decompress(&self, y: usize) -> &T {
        self.yx[y]
    }
    #[inline]
    pub fn unique_count(&self) -> usize {
        self.yx.len()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let x = [314, 999, 271, 101, 999, 271, 500];
        let y = x.index_compress();

        assert_eq!(&*y, &[2, 4, 1, 0, 4, 1, 3]);
        assert_eq!(y.compress(&101), 0);
        assert_eq!(y.compress(&271), 1);
        assert_eq!(y.compress(&314), 2);
        assert_eq!(y.compress(&500), 3);
        assert_eq!(y.compress(&999), 4);
        assert_eq!(y.decompress(0), &101);
        assert_eq!(y.decompress(1), &271);
        assert_eq!(y.decompress(2), &314);
        assert_eq!(y.decompress(3), &500);
        assert_eq!(y.decompress(4), &999);
    }
}
