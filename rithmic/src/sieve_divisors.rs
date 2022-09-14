use std::iter;

use itertools::Itertools;
use primal::Sieve;

/// Add a [`divisors`](Divisors::divisors) method to [`::primal::Sieve`](https://docs.rs/primal/latest/primal/struct.Sieve.html)
pub trait Divisors {
    /// Find all divisors of `n`
    ///
    /// # Examples
    /// ```
    /// use primal::Sieve;
    /// use rithmic::Divisors;
    ///
    /// let sieve = Sieve::new(100);
    /// assert_eq!(sieve.divisors(525), vec![1, 3, 5, 7, 15, 21, 25, 35, 75, 105, 175, 525]);
    /// ```
    fn divisors(&self, n: usize) -> Vec<usize>;
}

impl Divisors for Sieve {
    /// See [`Divisors::divisors`]
    fn divisors(&self, n: usize) -> Vec<usize> {
        if n == 0 { return vec![]; }
        if n == 1 { return vec![1]; }

        self.factor(n).expect("factorization failed; sieve limit must be at least sqrt(n)").into_iter()
            .map(|(f, p)| {
                let mut x = 1;
                iter::once(1).chain(iter::repeat_with(move || {
                    x *= f;
                    x
                }))
                .take(p+1)
            })
            .multi_cartesian_product()
            .map(|u| u.iter().product::<usize>())
            .sorted_unstable().collect()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factor_power_overflow() {
        let sieve = Sieve::new(1e6 as usize);
        let n = ((1<<36) - 5) * 2;
        assert_eq!(sieve.divisors(n), vec![1, 2, 68719476731, 137438953462]);
    }

    #[test]
    fn test_edge_cases() {
        let sieve = Sieve::new(100);
        assert_eq!(sieve.divisors(0), vec![]);
        assert_eq!(sieve.divisors(1), vec![1]);
        assert_eq!(sieve.divisors(2), vec![1, 2]);
    }
}
