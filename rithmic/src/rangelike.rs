use std::ops::{Range, RangeBounds, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive, Bound};

pub trait Rangelike<T> {
    fn clamp(&self, to: Range<T>) -> Option<Range<T>>;
    fn canonical(&self) -> (Option<T>, Option<T>);
}

macro impl_for_rangebounds($bounds_type: ty, $rangebounds: ty)
{
    impl Rangelike<$bounds_type> for $rangebounds
    {
        #[inline]
        fn clamp(&self, to: Range<$bounds_type>) -> Option<Range<$bounds_type>>
        {
            let start = match self.start_bound() {
                Bound::Included(&start) => to.start.max(start),
                Bound::Excluded(&start) => to.start.max(start + 1),
                Bound::Unbounded => to.start,
            };
            let end = match self.end_bound() {
                Bound::Included(&end) => to.end.min(end + 1),
                Bound::Excluded(&end) => to.end.min(end),
                Bound::Unbounded => to.end,
            };
            (start < end).then_some(start..end)
        }

        #[inline]
        fn canonical(&self) -> (Option<$bounds_type>, Option<$bounds_type>)
        {
            let start = match RangeBounds::<$bounds_type>::start_bound(self) {
                Bound::Included(&start) => Some(start),
                Bound::Excluded(&start) => Some(start.checked_add(1).unwrap()),
                Bound::Unbounded => None,
            };
            let end = match RangeBounds::<$bounds_type>::end_bound(self) {
                Bound::Included(&end) => end.checked_add(1),
                Bound::Excluded(&end) => Some(end),
                Bound::Unbounded => None,
            };
            (start, end)
        }
    }
}

macro impl_for_all_rangebounds($bounds_type: ty) {
    impl_for_rangebounds!($bounds_type, Range<$bounds_type>);
    impl_for_rangebounds!($bounds_type, RangeFrom<$bounds_type>);
    impl_for_rangebounds!($bounds_type, RangeFull);
    impl_for_rangebounds!($bounds_type, RangeInclusive<$bounds_type>);
    impl_for_rangebounds!($bounds_type, RangeTo<$bounds_type>);
    impl_for_rangebounds!($bounds_type, RangeToInclusive<$bounds_type>);
}

macro impl_rangelike {
    ($bounds_type: ty) =>
    {
        impl_for_all_rangebounds!($bounds_type);

        impl Rangelike<$bounds_type> for $bounds_type
        {
            #[inline]
            fn clamp(&self, to: Range<$bounds_type>) -> Option<Range<$bounds_type>> {
                to.contains(self).then_some(*self..(*self+1))
            }

            #[inline]
            fn canonical(&self) -> (Option<$bounds_type>, Option<$bounds_type>) {
                (Some(*self), self.checked_add(1))
            }
        }

        impl Rangelike<$bounds_type> for ($bounds_type, $bounds_type)
        {
            #[inline]
            fn clamp(&self, to: Range<$bounds_type>) -> Option<Range<$bounds_type>> {
                (self.0..self.1).clamp(to)
            }

            #[inline]
            fn canonical(&self) -> (Option<$bounds_type>, Option<$bounds_type>) {
                (Some(self.0), Some(self.1))
            }
        }

        impl Rangelike<$bounds_type> for [$bounds_type; 2]
        {
            #[inline]
            fn clamp(&self, to: Range<$bounds_type>) -> Option<Range<$bounds_type>> {
                (self[0]..self[1]).clamp(to)
            }

            #[inline]
            fn canonical(&self) -> (Option<$bounds_type>, Option<$bounds_type>) {
                (Some(self[0]), Some(self[1]))
            }
        }
    },

    ( $($bounds_type: ty),+ ) => {
        $( impl_rangelike!($bounds_type); )*
    },
}

impl_rangelike!(usize, u128, u64, u32, u16, u8);
impl_rangelike!(isize, i128, i64, i32, i16, i8);
