pub mod int_float_or_ratio;

#[cfg(test)] mod tests;

use std::cmp::Ordering;
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::marker::PhantomData;

use delegate::delegate;

use crate::{imin, imax};

use self::int_float_or_ratio::IntFloatOrRatio;

use super::{AugFn, Node, AugTreap};



#[derive(Default, Clone, Copy, Debug)]
struct ChtKey<T: IntFloatOrRatio, const MAX: bool>
{
    slope: T,
    intercept: T,
    start: T,
    end: T,
}

impl<T: IntFloatOrRatio, const MAX: bool> PartialEq for ChtKey<T, MAX> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.slope == other.slope
    }
}
impl<T: IntFloatOrRatio, const MAX: bool> Eq for ChtKey<T, MAX> {}
impl<T: IntFloatOrRatio, const MAX: bool> PartialOrd for ChtKey<T, MAX> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: IntFloatOrRatio, const MAX: bool> Ord for ChtKey<T, MAX> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        if MAX {
            self.slope.partial_cmp(&other.slope).expect("incomparable keys")
        } else {
            other.slope.partial_cmp(&self.slope).expect("incomparable keys")
        }
    }
}

impl<T: IntFloatOrRatio, const MAX: bool> ChtKey<T, MAX>
{
    #[inline]
    fn eval_at(&self, x: T) -> T {
        self.intercept + self.slope * x
    }

    #[inline]
    fn fully_left_of(&self, other: &Self) -> bool {
        let end = self.end.just_before();
        if MAX {
            self.slope <= other.slope && self.eval_at(end) >= other.eval_at(end)
        } else {
            self.slope >= other.slope && self.eval_at(end) <= other.eval_at(end)
        }
    }

    #[inline]
    fn fully_right_of(&self, other: &Self) -> bool {
        let start = self.start;
        if MAX {
            self.slope > other.slope && self.eval_at(start) >= other.eval_at(start)
        } else {
            self.slope < other.slope && self.eval_at(start) <= other.eval_at(start)
        }
    }

    #[inline]
    fn crossover(&self, other: &Self) -> T {
        if MAX {
            debug_assert!(self.slope < other.slope);
        } else {
            debug_assert!(self.slope > other.slope);
        }
        (self.intercept - other.intercept).div_ceil(other.slope - self.slope)
    }
}

#[derive(Default, Clone, Copy, Debug)]
struct ChtAug<T>
{
    start: T,
    end: T,
    len: usize,
}

#[derive(Default, Clone, Copy)]
struct ChtAugFn<T> {
    phantom: PhantomData<T>,
}
impl<T, const MAX: bool> AugFn<ChtKey<T, MAX>, (), ChtAug<T>> for ChtAugFn<T>
where T: IntFloatOrRatio
{
    #[inline]
    fn f(u: &Node<ChtKey<T, MAX>, (), ChtAug<T>>) -> ChtAug<T>
    {
        let mut start = u.key.start;
        let mut end = u.key.end;
        let mut len = 1;

        if let Some(l_aug) = u.left_aug() {
            imin!(start, l_aug.start);
            len += l_aug.len;
        }
        if let Some(r_aug) = u.right_aug() {
            imax!(end, r_aug.end);
            len += r_aug.len;
        }

        ChtAug { start, end, len }
    }
}

#[derive(Clone)]
pub struct ConvexHullTrick<T: IntFloatOrRatio, const MAX: bool>
{
    inner: AugTreap<ChtKey<T, MAX>, (), ChtAug<T>, ChtAugFn<T>>,
    start: T,
    end: T,
}

pub type ConvexHullMin<T> = ConvexHullTrick<T, false>;
pub type ConvexHullMax<T> = ConvexHullTrick<T, true>;

impl<T, const MAX: bool> ConvexHullTrick<T, MAX>
where T: IntFloatOrRatio
{
    pub fn new(start: T, end: T) -> Self {
        Self {
            inner: AugTreap::new(),
            start,
            end,
        }
    }

    delegate! {
    to self.inner {
        pub fn is_empty(&self) -> bool;
    }}

    pub fn len(&self) -> usize {
        self.inner.root.as_ref().map_or(0, |r| r.aug.len)
    }

    pub fn insert(&mut self, slope: T, intercept: T)
    {
        let mut new_line = ChtKey {
            slope, intercept,
            start: self.start,
            end: self.end,
        };

        let r = self.inner.root.take();
        let (u, v, w) = self.inner.split3_by(r, |k| k.fully_left_of(&new_line), |k| !k.fully_right_of(&new_line));

        if let Some(ref u) = u {
            imax!(new_line.start, u.aug.end);
        }
        if let Some(ref w) = w {
            imin!(new_line.end, w.aug.start);
        }

        let (mut v0, mut v2) = self.inner.into_first_last(v);

        // exactly 1 intersecting segment; we must determine if it is left or right side
        if v2.is_none() && let Some(ref v0_) = v0 {
            if MAX ^ (v0_.key.slope < new_line.slope) {
                (v0, v2) = (None, v0);
            }
        }

        if let Some(mut v0_) = v0
            && MAX ^ (v0_.key.slope > new_line.slope)
        {
            let x0 = v0_.key.crossover(&new_line);
            v0_.key.end = x0;
            imax!(new_line.start, x0);

            v0 = (v0_.key.start < v0_.key.end).then_some(v0_);
            v0.as_mut().map(|mut v0| v0.aug = ChtAugFn::<T>::f(v0));
        }
        else {
            v0 = None;
        }

        if let Some(mut v2_) = v2
            && MAX ^ (v2_.key.slope < new_line.slope)
        {
            let x1 = new_line.crossover(&v2_.key);
            imin!(new_line.end, x1);
            v2_.key.start = x1;

            v2 = (v2_.key.start < v2_.key.end).then_some(v2_);
            v2.as_mut().map(|mut v2| v2.aug = ChtAugFn::<T>::f(v2));
        }
        else {
            v2 = None;
        }

        let mut v1 = (new_line.start < new_line.end).then(|| Box::new(Node::new(new_line, ())));
        v1.as_mut().map(|mut v1| v1.aug = ChtAugFn::<T>::f(v1));

        let v = self.inner.join3(v0, v1, v2);
        let r = self.inner.join3(u, v, w);
        self.inner.root = r;
    }

    pub fn eval_at(&mut self, x: T) -> T
    {
        let mut v = &self.inner.root;
        assert!(v.is_some(), "cannot evaluate on an empty set");

        while let Some(u) = v {
            if x < u.key.start {
                v = &u.left;
            }
            else if x > u.key.end.just_before() {
                v = &u.right;
            }
            else {
                return u.key.eval_at(x)
            }
        }
        panic!("query outside bounds");
    }

    pub fn iter(&self) -> impl Iterator<Item=((T, T), (T, T))> + DoubleEndedIterator + FusedIterator + '_
    {
        self.inner.iter().map(|(k, _)| ((k.slope, k.intercept), (k.start, k.end)))
    }
}

impl<T: IntFloatOrRatio, const MAX: bool> Debug for ConvexHullTrick<T, MAX>
where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "ConvexHull{} over [{:?}, {:?})", if MAX {"Max"} else {"Min"}, self.start, self.end)?;
        write!(f, "\n(slope, intercept):")?;
        for u in self.inner.iter() {
            let key = u.0;
            write!(f, "\n({:?}, {:?}) over [{:?}, {:?})", key.slope, key.intercept, key.start, key.end)?;
        }
        Ok(())
    }
}
