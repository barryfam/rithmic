use std::ops::{RangeBounds, Bound};
use std::sync::Mutex;

use rayon::prelude::*;

use super::int_or_float::IntOrFloat;

pub fn par_binary_search<const THREADS: usize, X>(
    domain: impl RangeBounds<X>,
    search_for: bool,
    predicate: impl Fn(X) -> bool + Sync,
) -> Option<X>
where
    X: IntOrFloat,
{
    assert_ne!(THREADS, 0);

    let (mut r, end_incl) = match domain.end_bound() {
        Bound::Excluded(&end) => (end, false),
        Bound::Included(&end) => (end, true),
        Bound::Unbounded => (X::MAX, true)
    };
    let mut l = match domain.start_bound() {
        Bound::Included(&start) => start,
        Bound::Excluded(&start) => {
            if start == r { return None }
            start.next_towards(r)
        }
        Bound::Unbounded => X::MIN
    };
    if !end_incl {
        if l == r { return None }
        r = r.next_towards(l);
    }

    // state := left, right, right_checked, fail_flag
    let lr_rc_fail = Mutex::new((l, r, false, false));
    let (mut rc, mut fail);
    loop {
        X::n1_sect::<THREADS>(l, r).into_par_iter().for_each( |m| {
            let p = predicate(m) == search_for;

            let mut lr_rc_fail = lr_rc_fail.lock().unwrap();
            let (mut l, mut r, mut rc, mut fail) = *lr_rc_fail;

            if !m.in_interval(l, r) {
                return
            }

            if p {
                r = m;
                rc = true;
            }
            else {
                if l == r {
                    fail = true;
                }
                else {
                    l = m.next_towards(r);
                }
            }

            *lr_rc_fail = (l, r, rc, fail);
        });

        (l, r, rc, fail) = *lr_rc_fail.lock().unwrap();
        if fail {
            return None
        }
        if l == r && rc {
            return Some(r)
        }
    }
}
