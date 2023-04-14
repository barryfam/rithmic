pub trait Waterline {
    fn waterline_left(&self) -> Vec<usize>;
    fn waterline_right(&self) -> Vec<usize>;
}

impl<T> Waterline for [T]
where T: PartialOrd
{
    fn waterline_left(&self) -> Vec<usize> {
        let n = self.len();
        let mut dp = vec![0; n];

        for i in 1..n {
            let mut j = i;
            while j > 0 && self[j-1] <= self[i] {
                j = dp[j-1];
            }
            dp[i] = j;
        }
        dp
    }

    fn waterline_right(&self) -> Vec<usize> {
        let n = self.len();
        let mut dp = vec![0; n];

        for i in (0..n).rev() {
            let mut j = i;
            while j < n-1 && self[j+1] <= self[i] {
                j = dp[j+1];
            }
            dp[i] = j;
        }
        dp
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let v = vec![3, 7, 5, 2, 13, 17, 1];
        assert_eq!(v.waterline_left(), vec![0, 0, 2, 3, 0, 0, 6]);
        assert_eq!(v.waterline_right(), vec![0, 3, 3, 3, 4, 6, 6]);
    }
}
