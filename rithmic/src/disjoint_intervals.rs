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
        if let Some((end, v0)) = cursor.value_mut() && &interval.0 < end
        {
            let mut end_swap = interval.0.clone();
            mem::swap(end, &mut end_swap);
            if interval.1 < end_swap {
                split = Some((end_swap, v0.clone()));
            }
        }

        cursor.move_next();
        while let Some((start, (end, _))) = cursor.key_value_mut() && start < &interval.1
        {
            if *end <= interval.1 {
                cursor.remove_current();
            }
            else {
                let (_, kv0) = cursor.remove_current().unwrap();
                cursor.insert_before(interval.1.clone(), kv0);
                cursor.move_prev();
                break
            }
        }

        if let Some(split) = split {
            cursor.insert_before(interval.1.clone(), split);
            cursor.move_prev();
        }
        cursor.insert_before(interval.0, (interval.1, value));
    }

    pub fn query<Q: Ord>(&self, point: &Q) -> Option<&V>
    where K: Borrow<Q>
    {
        let (end, value) = self.0.upper_bound(Bound::Included(point)).value()?;
        (point <= end.borrow()).then_some(value)
    }

    pub fn iter(&self) -> impl Iterator<Item=((&K, &K), &V)>
    {
        self.0.iter().map(|(start, (end, value))| ((start, end), value))
    }
}
