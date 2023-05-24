/**
Order a pair (2-[`tuple`] or 2-[`array`])

```
use rithmic::OrdPair;

let (a, b) = (11, 3).ordered();
assert_eq!((a, b), (3, 11));

let [a, b] = [5, 17].ordered();
assert_eq!([a, b], [5, 17]);
```
*/
pub trait OrdPair {
    #[must_use]
    fn ordered(self) -> Self;
}

impl<T: PartialOrd> OrdPair for (T, T) {
    #[inline]
    fn ordered(self) -> (T, T) {
        if self.0 <= self.1 {
            self
        } else {
            (self.1, self.0)
        }
    }
}

impl<T: PartialOrd> OrdPair for [T; 2] {
    #[inline]
    fn ordered(mut self) -> [T; 2] {
        if self[0] > self[1] {
            self.as_mut_slice().swap(0, 1);
        }
        self
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_tuple() {
        assert_eq!(((3, 7), (3, 5)).ordered(), ((3, 5), (3, 7)));
    }
}
