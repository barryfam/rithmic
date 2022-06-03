pub trait OptionMerge {
    type T;

    /// Merges two [`Option`]s by the following logic:
    ///
    /// |`x`|`y`|`x.merge(y, f)`|
    /// |---|---|---|
    /// |`None`|`None`|`None`|
    /// |`Some(a)`|`None`|`Some(a)`|
    /// |`None`|`Some(b)`|`Some(b)`|
    /// |`Some(a)`|`Some(b)`|`Some(f(a, b))`|
    ///
    /// # Examples
    /// ```
    /// use rithmic::OptionMerge;
    ///
    /// assert_eq!(Some(3).merge(Some(5), |a, b| a+b), Some(8));
    /// assert_eq!(Some(3).merge(None, |a, b| a+b), Some(3));
    /// assert_eq!(None.merge(Some(5), |a, b| a+b), Some(5));
    /// ```
    ///
    /// # Notes
    /// If `T` has an [identity element](https://en.wikipedia.org/wiki/Identity_element) under `f()`, also consider [`Option::unwrap_or`]:
    /// ```
    /// # let (x, y) = (None, None);
    /// x.unwrap_or(0) + y.unwrap_or(0)
    /// # ;
    /// ```
    ///
    fn merge<F>(self, optb: Self, f: F) -> Self
    where F: FnOnce(Self::T, Self::T) -> Self::T;

    /// Merges an [`Option::Some`] with a 2nd value, or if [`None`](Option::None) simply returns the 2nd value:
    ///
    /// |`x`|`b`|`x.merge_or(b, f)`|
    /// |---|---|---|
    /// |`None`|`b`|`b`|
    /// |`Some(a)`|`b`|`f(a, b)`|
    ///
    /// # Notes
    /// [`x.map_or(b, |a| f(a, b))`](Option::map_or) is equivalent for `Copy` types
    ///
    fn merge_or<F>(self, b: Self::T, f: F) -> Self::T
    where F: FnOnce(Self::T, Self::T) -> Self::T;
}

impl<T> OptionMerge for Option<T> {
    type T = T;

    fn merge<F: FnOnce(T, T) -> T>(self, optb: Option<T>, f: F) -> Option<T> {
        match (self, optb) {
            (a, None) => a,
            (None, b) => b,
            (Some(a), Some(b)) => Some(f(a, b)),
        }
    }

    fn merge_or<F: FnOnce(T, T) -> T>(self, b: T, f: F) -> T {
        if let Some(a) = self {
            f(a, b)
        } else {
            b
        }
    }
}
