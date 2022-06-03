/**
Mutably borrow two indexes of a [`slice`] simultaneously.

# Examples
```compile_fail
let mut v = vec![vec![1, 2], vec![3, 4]];

// borrow conflict
v[0].append(v[1]);
```
```
# use rithmic::PairMut;
let mut v = vec![vec![1, 2], vec![3, 4]];

let (x, y) = v.pair_mut(0, 1);
x.append(y);

assert_eq!(v, vec![vec![1, 2, 3, 4], vec![]]);
```
# Safety
[`slice::split_at_mut`] is used to enable this method without `unsafe`
*/
pub trait PairMut {
    type Item;
    fn pair_mut(&mut self, i: usize, j: usize) -> (&mut Self::Item, &mut Self::Item);
}

impl<T> PairMut for [T] {
    type Item = T;
    fn pair_mut(&mut self, i: usize, j: usize) -> (&mut Self::Item, &mut Self::Item)
    {
        debug_assert_ne!(i, j, "indexes must be different");

        if i < j {
            let (u, v) = self.split_at_mut(j);
            (&mut u[i], &mut v[0])
        }
        else {
            let (u, v) = self.split_at_mut(i);
            (&mut v[0], &mut u[j])
        }
    }
}
