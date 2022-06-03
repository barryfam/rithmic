use crate::UnwrapAny;

pub trait Insort {
    type T;
    fn insort(&mut self, element: Self::T);
}
impl<T> Insort for Vec<T>
where T: Ord
{
    type T = T;
    fn insort(&mut self, element: T) {
        let i = self.binary_search(&element).unwrap_any();
        self.insert(i, element);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut v = vec![2, 3, 7, 11];
        v.insort(5);
        assert_eq!(v, vec![2, 3, 5, 7, 11]);
    }
}
