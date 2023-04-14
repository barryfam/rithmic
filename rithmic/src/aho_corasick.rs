use std::borrow::Borrow;
use std::collections::{BTreeMap, VecDeque};
use std::mem;

const NONE: usize = !0;

#[derive(Clone, Debug)]
struct Node<T> {
    children: BTreeMap<T, usize>,
    suffix: usize,
    leaf_id: usize,
    suffix_leaf: usize,
}

impl<T> Default for Node<T> {
    fn default() -> Self {
        Self {
            children: BTreeMap::default(),
            suffix: 0,
            leaf_id: NONE,
            suffix_leaf: NONE,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct AhoCorasick<T> {
    pub patterns: Vec<Vec<T>>,
    tomato: Vec<Node<T>>,
}

impl<T> AhoCorasick<T>
where T: Ord + Clone
{
    pub fn new(patterns: impl IntoIterator<Item=impl IntoIterator<Item=T>>) -> Self {
        let mut _patterns = vec![];
        let root = Node::default();
        let mut tomato = vec![root];

        for (i, pattern) in patterns.into_iter().enumerate() {
            let mut _pattern = vec![];

            let mut u = 0;
            for c in pattern {
                let n = tomato.len();
                let v = *tomato[u].children.entry(c.clone()).or_insert(n);
                if v == n {
                    tomato.push(Node::default());
                }
                u = v;

                _pattern.push(c);
            }
            tomato[u].leaf_id = i;

            _patterns.push(_pattern);
        }

        let mut q = VecDeque::from([0]);
        while let Some(u) = q.pop_front()
        {
            // CORRECTNESS: must not read/write this field during this loop
            let children = mem::take(&mut tomato[u].children);

            for (c, &v) in &children {
                let mut w = u;
                while w > 0 {
                    w = tomato[w].suffix;

                    debug_assert_ne!(u, w);
                    if let Some(&x) = tomato[w].children.get(c)
                    {
                        tomato[v].suffix = x;
                        tomato[v].suffix_leaf =
                            if tomato[x].leaf_id != NONE {x} else {tomato[x].suffix_leaf};

                        break
                    }
                }
                q.push_back(v);
            }

            tomato[u].children = children;
        }

        Self { tomato, patterns: _patterns }
    }

    #[inline]
    pub fn step(&self, state: usize, c: impl Borrow<T>) -> usize {
        let a = &self.tomato;
        let mut u = state;
        loop {
            if let Some(&v) = a[u].children.get(c.borrow()) {
                return v
            }
            else if u == 0 {
                return 0
            }
            u = a[u].suffix;
        }
    }

    #[inline]
    pub fn matches(&self, state: usize) -> Vec<usize> {
        let a = &self.tomato;
        let mut u = state;
        let mut ret = vec![];

        if a[u].leaf_id != NONE {
            ret.push(a[u].leaf_id);
        }
        while let v = a[u].suffix_leaf && v != NONE {
            u = v;
            ret.push(a[u].leaf_id);
        }
        ret
    }

    pub fn run(&self, haystack: impl IntoIterator<Item=impl Borrow<T>>)
        -> Vec<((usize, usize), (usize, &[T]))>
    {
        let mut u = 0;
        let mut ret = vec![];

        for (c, i) in haystack.into_iter().zip(1..) {
            u = self.step(u, c.borrow());

            ret.extend(self.matches(u).into_iter().map( |id| (
                (i - self.patterns[id].len(), i),
                (id, &self.patterns[id][..])
            )));
        }
        ret
    }
}

impl<T> std::fmt::Display for AhoCorasick<T>
where T: std::fmt::Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let a = &self.tomato;

        writeln!(f, "flowchart TD")?;
        for u in 0..self.tomato.len()
        {
            let (l, r) = if a[u].leaf_id == NONE {("((", "))")} else {("(((", ")))")};
            writeln!(f, "    {u}{l}{u}{r}")?;

            for (c, &v) in &a[u].children {
                writeln!(f, "    {u} -->|{c}| {v}")?;
            }
            if a[u].suffix != NONE {
                writeln!(f, "    {u} -.-> {}", a[u].suffix)?;
            }
            if a[u].suffix_leaf != NONE {
                writeln!(f, "    {u} ==x {}", a[u].suffix_leaf)?;
            }
        }
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use indoc::indoc;
    use super::*;

    #[test]
    fn test_basic() {
        let ac = AhoCorasick::new([
                "Apple",
                "Store",
                "AppleAppStore",
            ]
            .map(str::chars));

        assert_eq!(format!("{}", &ac), indoc!("
            flowchart TD
                0((0))
                0 -->|A| 1
                0 -->|S| 6
                0 -.-> 0
                1((1))
                1 -->|p| 2
                1 -.-> 0
                2((2))
                2 -->|p| 3
                2 -.-> 0
                3((3))
                3 -->|l| 4
                3 -.-> 0
                4((4))
                4 -->|e| 5
                4 -.-> 0
                5(((5)))
                5 -->|A| 11
                5 -.-> 0
                6((6))
                6 -->|t| 7
                6 -.-> 0
                7((7))
                7 -->|o| 8
                7 -.-> 0
                8((8))
                8 -->|r| 9
                8 -.-> 0
                9((9))
                9 -->|e| 10
                9 -.-> 0
                10(((10)))
                10 -.-> 0
                11((11))
                11 -->|p| 12
                11 -.-> 1
                12((12))
                12 -->|p| 13
                12 -.-> 2
                13((13))
                13 -->|S| 14
                13 -.-> 3
                14((14))
                14 -->|t| 15
                14 -.-> 6
                15((15))
                15 -->|o| 16
                15 -.-> 7
                16((16))
                16 -->|r| 17
                16 -.-> 8
                17((17))
                17 -->|e| 18
                17 -.-> 9
                18(((18)))
                18 -.-> 10
                18 ==x 10
        "));

        let matches = ac.run("AppleBananaAppleAppleAppStoreWidgetStore".chars())
            .into_iter().collect::<Vec<_>>();

        assert!(matches!(matches[..], [
                ((0, 5), (0, _)),
                ((11, 16), (0, _)),
                ((16, 21), (0, _)),
                ((16, 29), (2, _)),
                ((24, 29), (1, _)),
                ((35, 40), (1, _)),
            ]),
            "assert match failed: {:?}", matches
        );
    }
}
