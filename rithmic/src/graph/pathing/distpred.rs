use crate::graph::NONE;

#[derive(Default, Clone, Debug)]
pub struct DistPred<E> {
    dp: Vec<(E, usize)>,
    t0: usize,
}

impl<E> DistPred<E>
where E: Copy
{
    pub(super) fn new(dp: Vec<(E, usize)>, t0: usize) -> Self {
        Self { dp, t0 }
    }

    #[inline]
    pub fn get(&self, u: usize) -> Option<(E, usize)> {
        let (d, p) = self.dp[u];
        (p != NONE).then_some((d, p))
    }

    #[inline]
    pub fn dist_to(&self, u: usize) -> Option<E> {
        Some(self.get(u)?.0)
    }

    #[inline]
    pub fn dist(&self) -> Option<E> {
        debug_assert!(self.t0 != NONE, "More than one destination was specified; use .dist_to() instead");
        self.dist_to(self.t0)
    }

    #[inline]
    pub fn pred(&self, u: usize) -> Option<usize> {
        Some(self.get(u)?.1)
    }
}

impl<E> DistPred<E>
where E: Copy + Default + PartialEq
{
    pub fn path_to(&self, t: usize) -> Option<Vec<usize>>
    {
        let mut path = vec![];

        let mut v = t;
        while self.dist_to(v)? != E::default() {
            path.push(v);
            v = self.pred(v)?;
        }
        path.push(v);
        path.reverse();
        Some(path)
    }

    #[inline]
    pub fn path(&self) -> Option<Vec<usize>> {
        debug_assert!(self.t0 != NONE, "More than one destination was specified; use .path_to() instead");
        self.path_to(self.t0)
    }
}
