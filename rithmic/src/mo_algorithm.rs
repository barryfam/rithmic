use itertools::Itertools;
use num::integer::Roots;

use crate::Rangelike;

/// This `enum` is created by [`mo_algorithm()`]. See its documentation for more
pub enum MoStep {
    Add(usize),
    Remove(usize),
    Answer(usize),
}

/**
[Mo's Algorithm](https://cp-algorithms.com/data_structures/sqrt_decomposition.html#mos-algorithm)

Given a list of *q* range queries about a sequence of length *n*, produce an iterator of instructions [`MoStep::`](MoStep)[`Add`](MoStep::Add), [`Remove`](MoStep::Remove), [`Answer`](MoStep::Answer) that answers all queries in *O*((*n*+*q*) √*n*) steps

# Examples
```
# use rithmic::mo_algorithm;
# use rithmic::MoStep;
let seq = [10000, 2000, 300, 40, 5];
let queries = [2..5, 0..3, 1..2];
let mut answers = [0; 3];
let mut current = 0;

for step in mo_algorithm(seq.len(), queries) {
match step {
    MoStep::Add(i)    => current += seq[i],
    MoStep::Remove(i) => current -= seq[i],
    MoStep::Answer(q) => answers[q] = current
}}

assert_eq!(answers, [345, 12300, 2000]);
```
*/
pub fn mo_algorithm(n: usize, queries: impl IntoIterator<Item=impl Rangelike<usize>>) -> impl Iterator<Item=MoStep>
{
    let k = n.sqrt();

    let queries: Vec<_> = queries.into_iter()
        .map(|r| r.clamp(0..n).unwrap())
        .enumerate()
        .sorted_unstable_by( |
            &(_i, (li, ri)),
            &(_j, (lj, rj))
        | {
            let bi = li / k;
            let bj = lj / k;
            if bi != bj { bi.cmp(&bj) }
            else {
                if bi & 1 == 0 { ri.cmp(&rj) }
                else { rj.cmp(&ri) }
            }
        })
        .collect();

    if let Some(&(_, (l, _r))) = queries.last() {
        MoIter { queries, l, r: l }
    } else {
        MoIter::default()
    }
}

#[derive(Default)]
struct MoIter {
    queries: Vec<(usize, (usize, usize))>,
    l: usize,
    r: usize,
}

impl Iterator for MoIter {
    type Item = MoStep;

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        let &(qi, (ql, qr)) = self.queries.last()?;

        Some(
            if ql < self.l {
                self.l -= 1;
                MoStep::Add(self.l)
            }
            else if self.r < qr {
                self.r += 1;
                MoStep::Add(self.r - 1)
            }
            else if self.l < ql {
                self.l += 1;
                MoStep::Remove(self.l - 1)
            }
            else if qr < self.r {
                self.r -= 1;
                MoStep::Remove(self.r)
            }
            else {
                self.queries.pop();
                MoStep::Answer(qi)
            }
        )
    }
}
