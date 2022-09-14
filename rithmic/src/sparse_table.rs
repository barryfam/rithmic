use crate::{IntBitOps, Rangelike};

#[derive(Clone)]
pub struct SparseTable<T> {
    len: usize,
    table: Vec<Vec<T>>,
    operator: fn(&T, &T) -> T
}
impl<T> SparseTable<T> {
    pub fn from(v: Vec<T>, operator: fn(&T, &T) -> T) -> Self
    {
        let len = v.len();
        let h = len.saturating_sub(1).bit_length() as usize;
        let mut table = Vec::<Vec<T>>::with_capacity(h);

        table.push(v);
        for i in 1..h {
            let rw = len+1 - (1<<i);
            let mut r = Vec::<T>::with_capacity(rw);
            for j in 0..rw {
                r.push(operator(&table[i-1][j], &table[i-1][j + (1<<(i-1))]));
            }
            table.push(r);
        }

        Self{len, table, operator}
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn query(&self, range: impl Rangelike<usize>) -> T
    {
        let (i, j) = range.clamp(0..self.len).expect("query out of range");
        assert!(i < j, "empty query range");

        let row = (j-i-1).bit_length().saturating_sub(1) as usize;
        (self.operator)(&self.table[row][i], &self.table[row][j - (1<<row)])
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rmq() {
        let v = vec![3, 1, 4, 1, 5, 9];
        let rmq = SparseTable::from(v, |&a, &b| a.max(b));

        assert_eq!(rmq.query(..), 9);
        assert_eq!(rmq.query(..3), 4);
        assert_eq!(rmq.query(3..), 9);
        assert_eq!(rmq.query(1..4), 4);
        assert_eq!(rmq.query(1..=4), 5);
        assert_eq!(rmq.query(1..=1), 1);
    }

    #[test]
    fn test_edge_cases()
    {
        let rmq = SparseTable::from(Vec::<i32>::new(), |&a, &b| a.max(b));
        assert_eq!(rmq.len(), 0);

        let rmq = SparseTable::from(vec![13], |&a, &b| a.max(b));
        assert_eq!(rmq.query(..), 13);

        let rmq = SparseTable::from(vec![13, 5], |&a, &b| a.max(b));
        assert_eq!(rmq.query(..), 13);
        assert_eq!(rmq.query(1..), 5);

        let rmq = SparseTable::from(vec![5, 13], |&a, &b| a.max(b));
        assert_eq!(rmq.query(..), 13);
        assert_eq!(rmq.query(..1), 5);
    }
}
