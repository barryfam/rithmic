pub trait CountInversions {
    fn sort_count_inversions(&mut self) -> usize;
}

impl<T> CountInversions for [T]
where
    T: Ord + Copy
{
    fn sort_count_inversions(&mut self) -> usize {
        if self.len() <= 1 { return 0 }

        let mut clone = self.to_vec();
        let (a, b) = clone.split_at_mut(self.len() / 2);
        let mut count = 0;
        count += a.sort_count_inversions();
        count += b.sort_count_inversions();

        let (mut i, mut j) = (0, 0);
        for k in 0..self.len()
        {
            if j == b.len() || i < a.len() && a[i] <= b[j] {
                self[k] = a[i];
                i += 1;
            }
            else {
                self[k] = b[j];
                j += 1;
                count += a.len() - i;
            }
        }
        count
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut a = vec![4, 3, 1, 2];
        let c = a.sort_count_inversions();

        assert_eq!(a, vec![1, 2, 3, 4]);
        assert_eq!(c, 5);

        let mut a = vec![3, 2, 2, 2, 2, 1, 4];
        let c = a.sort_count_inversions();

        assert_eq!(c, 9);
    }
}
