![Ferris the crab playing the bongos](https://barryfam.io/rithmic/rithmic.png)

[![build](https://img.shields.io/github/workflow/status/barryfam/rithmic/CI/master)](https://github.com/barryfam/rithmic/actions/workflows/ci.yml)
[![coverage](https://img.shields.io/coveralls/github/barryfam/rithmic)](https://coveralls.io/github/barryfam/rithmic)
[![docs](https://img.shields.io/badge/docs-partial-yellow)](https://www.barryfam.io/rithmic/doc/rithmic/)

`#![forbid(unsafe_code)]`

# rithmic
Odds & ends: Algorithms, data structures, helpers

### Utility
- [Print methods](https://www.barryfam.io/rithmic/doc/rithmic/trait.PrintMethods.html)
- Incremental [min](https://www.barryfam.io/rithmic/doc/rithmic/macro.imin.html)/[max](https://www.barryfam.io/rithmic/doc/rithmic/macro.imax.html)
- [Ordered pair](https://www.barryfam.io/rithmic/doc/rithmic/trait.OrdPair.html)
- [Option merge](https://www.barryfam.io/rithmic/doc/rithmic/trait.OptionMerge.html)
- [Result unwrap-any](https://www.barryfam.io/rithmic/doc/rithmic/trait.UnwrapAny.html)
- [Slice simultaneous mutable borrow](https://www.barryfam.io/rithmic/doc/rithmic/trait.PairMut.html)
- [`Rangelike`](https://www.barryfam.io/rithmic/doc/rithmic/trait.Rangelike.html), a trait accepting integers and integer ranges *(no doc yet)*
- [Unsigned integers as bitsets](https://www.barryfam.io/rithmic/doc/rithmic/trait.IntBitOps.html) *(no doc yet)*
- [Mixed-radix counter](https://www.barryfam.io/rithmic/doc/rithmic/type.OdometerLE.html)

### Algorithms
- [Binary search](https://www.barryfam.io/rithmic/doc/rithmic/fn.binary_search.html)
- [Ternary search](https://www.barryfam.io/rithmic/doc/rithmic/fn.ternary_search.html)
- [Index compression](https://www.barryfam.io/rithmic/doc/rithmic/trait.IndexCompress.html)
- [Prefix sums](https://www.barryfam.io/rithmic/doc/rithmic/trait.PrefixSums.html)
- [Graph](https://www.barryfam.io/rithmic/doc/rithmic/graph/prelude/struct.Graph.html) *(no doc yet)*
    - Dijkstra, 0-1 Dijkstra, BFS
    - Floyd-Warshall
    - DFS
    - Tarjan SCC
    - Rooted subtrees
- [Mo's algorithm](https://www.barryfam.io/rithmic/doc/rithmic/fn.mo_algorithm.html)
- [Inversion counting](https://www.barryfam.io/rithmic/doc/rithmic/trait.CountInversions.html) *(no doc yet)*

### Data Structures
- [Tensor](https://www.barryfam.io/rithmic/doc/rithmic/struct.NdVec.html) ("*n*-dimensional vector") *(no doc yet)*
- [Boolean/bit vector](https://www.barryfam.io/rithmic/doc/rithmic/struct.BVec.html)
- [Meldable heap](https://www.barryfam.io/rithmic/doc/rithmic/struct.MeldHeap.html)
- [Generic augmented treap](https://www.barryfam.io/rithmic/doc/rithmic/aug_treap/struct.AugTreap.html) *(no doc yet)*
    - [Generic order-statistic treap](https://www.barryfam.io/rithmic/doc/rithmic/aug_treap/order_treap/struct.OrderTreap.html) *(no doc yet)*
        - [`List`](https://www.barryfam.io/rithmic/doc/rithmic/aug_treap/order_treap/struct.List.html), a sequence with *O*(log *n*) insert/delete *(no doc yet)*
        - [`SortedList`](https://www.barryfam.io/rithmic/doc/rithmic/aug_treap/order_treap/struct.SortedList.html), with rank- and order-statistics *(no doc yet)*
- [Generic sparse table](https://www.barryfam.io/rithmic/doc/rithmic/struct.SparseTable.html) *(no doc yet)*
- [Generic segment tree](https://www.barryfam.io/rithmic/doc/rithmic/monoid_ds/struct.SegTree.html) supports lazy updates and "segtree beats" *(no doc yet)*
- [Union-find](https://www.barryfam.io/rithmic/doc/rithmic/struct.UnionFind.html)
- [XOR basis](https://www.barryfam.io/rithmic/doc/rithmic/xor_basis/index.html)

### Math
- [Triangular numbers](https://www.barryfam.io/rithmic/doc/rithmic/triangular_n/index.html)
- [All divisors of an integer](https://www.barryfam.io/rithmic/doc/rithmic/trait.Divisors.html#tymethod.divisors) using [`::primal::Sieve`](https://docs.rs/primal/latest/primal/struct.Sieve.html)
- [Factorials mod *n*](https://www.barryfam.io/rithmic/doc/rithmic/struct.FactorialTable.html) using [`::ac_library_rs::modint`](https://www.barryfam.io/rithmic/doc/ac_library_rs/modint/index.html) *(no doc yet)*
- [Polynomial arithmetic](https://www.barryfam.io/rithmic/doc/rithmic/polynomial/index.html) *(no doc yet)*
    - [Lagrange interpolation](https://www.barryfam.io/rithmic/doc/rithmic/polynomial/fn.lagrange_interpolation.html) *(no doc yet)*
- [A few extra methods](https://www.barryfam.io/rithmic/doc/rithmic/trait.Vector2DMore.html) for [`::euclid::Vector2D`](https://docs.rs/euclid/latest/euclid/struct.Vector2D.html) *(no doc yet)*

### Misc
- [`Gameboard`](https://www.barryfam.io/rithmic/doc/rithmic/gameboard/struct.Gameboard.html)
