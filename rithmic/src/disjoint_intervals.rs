use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::mem;
use std::ops::Bound;

#[derive(Default, Clone, Debug)]
pub struct DisjointIntervals<K, V>(BTreeMap<K, (K, V)>);

impl<K, V> DisjointIntervals<K, V>
where
    K: Ord + Clone,
    V: Clone
{
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, interval: (K, K), value: V)
    {
        assert!(interval.0 < interval.1, "interval must be positive width");

        let mut cursor = self.0.upper_bound_mut(Bound::Excluded(&interval.0));
        let mut split = None;
        if let Some((_, (end0, v0))) = cursor.peek_prev() && interval.0 < *end0
        {
            let mut end_swap = interval.0.clone();
            mem::swap(end0, &mut end_swap);
            if interval.1 < end_swap {
                split = Some((end_swap, v0.clone()));
            }
        }

        while let Some((start1, (end1, _))) = cursor.peek_next() && start1 < &interval.1
        {
            if *end1 <= interval.1 {
                cursor.remove_next();
            }
            else {
                let (_, kv1) = cursor.remove_next().unwrap();
                cursor.insert_after(interval.1.clone(), kv1).unwrap();
                break
            }
        }

        if let Some(split) = split {
            cursor.insert_after(interval.1.clone(), split).unwrap();
        }

        cursor.insert_after(interval.0, (interval.1, value)).unwrap();
    }

    pub fn query<Q: Ord>(&self, point: &Q) -> Option<&V>
    where K: Borrow<Q>
    {
        let (end, value) = self.0.upper_bound(Bound::Included(point)).peek_prev()?.1;
        (point <= end.borrow()).then_some(value)
    }

    pub fn iter(&self) -> impl Iterator<Item=((&K, &K), &V)>
    {
        self.0.iter().map(|(start, (end, value))| ((start, end), value))
    }
}
