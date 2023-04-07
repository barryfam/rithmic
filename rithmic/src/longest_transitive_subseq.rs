use crate::binary_search;

pub trait LongestTransitiveSubseq<T> {
    fn longest_transitive_subseq(&self, f: impl FnMut(&T, &T) -> bool) -> Vec<usize>;
}

impl<T> LongestTransitiveSubseq<T> for [T] {
    fn longest_transitive_subseq(&self, mut f: impl FnMut(&T, &T) -> bool) -> Vec<usize>
    {
        if self.is_empty() { return vec![] }

        let mut pred: Vec<usize> = Vec::with_capacity(self.len());
        let mut hull: Vec<(&T, usize)> = vec![];

        for (i, y) in self.iter().enumerate()
        {
            let j = binary_search(0..hull.len(), false, |j| f(hull[j].0, y))
                .unwrap_or(hull.len());

            pred.push(if j > 0 {hull[j-1].1} else {!0});

            if j < hull.len() {
                hull[j] = (y, i);
            } else {
                hull.push((y, i));
            }
        }

        let mut i = hull.last().unwrap().1;
        let mut seq = Vec::with_capacity(hull.len());
        while i != !0 {
            seq.push(i);
            i = pred[i];
        }
        seq.reverse();
        seq
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let x = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 99];
        let ss = x.longest_transitive_subseq(|a, b| a < b);

        assert_eq!(ss, vec![0, 4, 6, 9, 13, 15]);
    }
}
