use std::iter::FusedIterator;

mod sealed {
    #[derive(Clone, Debug)]
    pub struct Odometer<const D: usize, const BE: bool> {
        pub(super) front: [usize; D],
        pub(super) back: [usize; D],
        pub(super) radix: [usize; D],
        pub(super) size: usize,
    }
}

/**
An [`Iterator`](std::iter::Iterator) over a mixed-radix numbers

Little-endian, meaning the lowest index is incremented first. See [`OdometerBE`] for the big-endian version

# Examples
```
# use rithmic::OdometerLE;
let mut i = OdometerLE::new([3, 2, 5]);

assert_eq!(i.next(), Some([0, 0, 0]));
assert_eq!(i.next(), Some([1, 0, 0]));
assert_eq!(i.next(), Some([2, 0, 0]));
assert_eq!(i.next(), Some([0, 1, 0]));
assert_eq!(i.next(), Some([1, 1, 0]));
assert_eq!(i.next(), Some([2, 1, 0]));
assert_eq!(i.next(), Some([0, 0, 1]));

assert_eq!(i.next_back(), Some([2, 1, 4]));
assert_eq!(i.next_back(), Some([1, 1, 4]));

assert_eq!(OdometerLE::new([3, 2, 5]).count(), 30);
```
*/
pub type OdometerLE<const D: usize> = sealed::Odometer<D, false>;

/**
An [`Iterator`](std::iter::Iterator) over a mixed-radix numbers

Big-endian, meaning the highest index is incremented first. See [`OdometerLE`] for the little-endian version

# Examples
```
# use rithmic::OdometerBE;
let mut i = OdometerBE::new([24, 60, 60]);

assert_eq!(i.next(), Some([0, 0, 0]));
assert_eq!(i.next(), Some([0, 0, 1]));

assert_eq!(i.next_back(), Some([23, 59, 59]));
assert_eq!(i.next_back(), Some([23, 59, 58]));
```
*/
pub type OdometerBE<const D: usize> = sealed::Odometer<D, true>;

impl<const D: usize, const BE: bool> sealed::Odometer<D, BE> {
    pub fn new(radix: [usize; D]) -> Self
    {
        let mut back = radix;
        for x in &mut back { *x = x.saturating_sub(1); }

        Self {
            front: [0; D],
            back,
            radix,
            size: radix.iter().product(),
        }
    }
}

impl<const D: usize, const BE: bool> Iterator for sealed::Odometer<D, BE> {
    type Item = [usize; D];

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        self.size = self.size.checked_sub(1)?;
        let next = self.front;

        let i_iter: Box<dyn Iterator<Item=usize>> =
            if !BE { Box::new(0..D) }
            else { Box::new((0..D).rev()) };

        for i in i_iter {
            if self.front[i] < self.radix[i]-1 {
                self.front[i] += 1;

                for j in if !BE {0..i} else {i+1..D} {
                    self.front[j] = 0;
                }
                break
            }
        }
        Some(next)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }
}

impl<const D: usize, const BE: bool> DoubleEndedIterator for sealed::Odometer<D, BE> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item>
    {
        self.size = self.size.checked_sub(1)?;
        let next = self.back;

        let i_iter: Box<dyn Iterator<Item=usize>> =
            if !BE { Box::new(0..D) }
            else { Box::new((0..D).rev()) };

        for i in i_iter {
            if self.back[i] > 0 {
                self.back[i] -= 1;

                for j in if !BE {0..i} else {i+1..D} {
                    self.back[j] = self.radix[j] - 1;
                }
                break
            }
        }
        Some(next)
    }
}

impl<const D: usize, const BE: bool> ExactSizeIterator for sealed::Odometer<D, BE> {}
impl<const D: usize, const BE: bool> FusedIterator for sealed::Odometer<D, BE> {}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut i = OdometerBE::new([2, 0, 3]);
        assert_eq!(i.next(), None);
    }
}
