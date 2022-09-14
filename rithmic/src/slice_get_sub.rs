/**
[`get_sub`](`GetSub::get_sub`)`(index, subtract)` indexes position `[index - subtract]` of a slice, but handles underflow (returning `None`)

Also, `index = !0` may be used to index from the end of the slice

# Examples
```
# use rithmic::GetSub;
let a = [0.0, 0.1, 0.2, 0.3, 0.4];

assert_eq!(a.get_sub(3, 1), Some(&0.2));
assert_eq!(a.get_sub(0, 1), None);
assert_eq!(a.get_sub(!0, 1), Some(&0.4));
```
*/
pub trait GetSub {
    type Item;
    fn get_sub(&self, index: usize, subtract: usize) -> Option<&Self::Item>;
    fn get_sub_mut(&mut self, index: usize, subtract: usize) -> Option<&mut Self::Item>;
}

impl<T> GetSub for [T] {
    type Item = T;

    #[inline]
    fn get_sub(&self, index: usize, subtract: usize) -> Option<&Self::Item>
    {
        let i = if index == !0 {self.len()} else {index};
        self.get(i.checked_sub(subtract)?)
    }

    #[inline]
    fn get_sub_mut(&mut self, index: usize, subtract: usize) -> Option<&mut Self::Item>
    {
        let i = if index == !0 {self.len()} else {index};
        self.get_mut(i.checked_sub(subtract)?)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mut() {
        let mut u = vec![2, 3, 5];
        u.get_sub_mut(7, 5).map(|x| *x = 11);
        u.get_sub_mut(!0, 2).map(|x| *x = 13);
        assert_eq!(u, vec![2, 13, 11]);
    }
}
