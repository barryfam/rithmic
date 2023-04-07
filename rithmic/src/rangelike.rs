use std::ops::{Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive, Bound};

/**
This trait is intended to allow function APIs to interchangeably accept integers or any of their [`Range`] variants

2-[`tuples`](tuple) and 2-[`arrays`](array) are also accepted

# Examples
```
let mut st = rithmic::SegTree::<i32>::from([2, 3, 5, 7, 11]);

assert_eq!(st.query(2), 5);
assert_eq!(st.query(2..=3), 5+7);
assert_eq!(st.query(2..), 5+7+11);
assert_eq!(st.query(..), 2+3+5+7+11);
```
*/
pub trait Rangelike<T>
{
    /// Convert to `(start_included, end_excluded)` and check the range is a subset of `to`, otherwise return `None`. Unbounded side(s) are substituted with `to.start` or `to.end` respectively.
    fn clamp(&self, to: Range<T>) -> Option<(T, T)>;

    /// Convert to `(start_included, end_excluded)` where each side is `None` if unbounded
    fn canonical(&self) -> (Option<T>, Option<T>);
}

macro impl_for_rangebounds($int_type: ty, $rangebounds: ty)
{
    impl Rangelike<$int_type> for $rangebounds
    {
        #[inline]
        fn clamp(&self, to: Range<$int_type>) -> Option<($int_type, $int_type)>
        {
            let (start, end) = self.canonical();
            let (start, end) = (start.unwrap_or(to.start), end.unwrap_or(to.end));

            debug_assert!(start <= end);
            (to.start <= start && end <= to.end)
                .then_some((start, end))
        }

        #[inline]
        fn canonical(&self) -> (Option<$int_type>, Option<$int_type>)
        {
            let start = match RangeBounds::<$int_type>::start_bound(self) {
                Bound::Included(&start) => Some(start),
                Bound::Excluded(&start) => Some(start.checked_add(1).unwrap()),
                Bound::Unbounded => None,
            };
            let end = match RangeBounds::<$int_type>::end_bound(self) {
                Bound::Included(&end) => end.checked_add(1),
                Bound::Excluded(&end) => Some(end),
                Bound::Unbounded => None,
            };
            (start, end)
        }
    }
}

macro impl_for_all_rangebounds($int_type: ty) {
    impl_for_rangebounds!($int_type, Range<$int_type>);
    impl_for_rangebounds!($int_type, RangeFrom<$int_type>);
    impl_for_rangebounds!($int_type, RangeFull);
    impl_for_rangebounds!($int_type, RangeInclusive<$int_type>);
    impl_for_rangebounds!($int_type, RangeTo<$int_type>);
    impl_for_rangebounds!($int_type, RangeToInclusive<$int_type>);
}

macro impl_rangelike {
    ($int_type: ty) =>
    {
        impl_for_all_rangebounds!($int_type);

        impl Rangelike<$int_type> for $int_type
        {
            #[inline]
            fn clamp(&self, to: Range<$int_type>) -> Option<($int_type, $int_type)> {
                to.contains(self).then_some((*self, *self+1))
            }

            #[inline]
            fn canonical(&self) -> (Option<$int_type>, Option<$int_type>) {
                (Some(*self), self.checked_add(1))
            }
        }

        impl Rangelike<$int_type> for ($int_type, $int_type)
        {
            #[inline]
            fn clamp(&self, to: Range<$int_type>) -> Option<($int_type, $int_type)> {
                (self.0..self.1).clamp(to)
            }

            #[inline]
            fn canonical(&self) -> (Option<$int_type>, Option<$int_type>) {
                (Some(self.0), Some(self.1))
            }
        }

        impl Rangelike<$int_type> for [$int_type; 2]
        {
            #[inline]
            fn clamp(&self, to: Range<$int_type>) -> Option<($int_type, $int_type)> {
                (self[0]..self[1]).clamp(to)
            }

            #[inline]
            fn canonical(&self) -> (Option<$int_type>, Option<$int_type>) {
                (Some(self[0]), Some(self[1]))
            }
        }
    },

    ( $($int_type: ty),+ ) => {
        $( impl_rangelike!($int_type); )*
    },
}

impl_rangelike!(usize, u128, u64, u32, u16, u8);
impl_rangelike!(isize, i128, i64, i32, i16, i8);
