![Ferris the crab playing the bongos](https://barryfam.io/rithmic/rithmic.png)

[![build](https://img.shields.io/github/workflow/status/barryfam/rithmic/CI/master)](https://github.com/barryfam/rithmic/actions/workflows/ci.yml)
[![coverage](https://img.shields.io/coveralls/github/barryfam/rithmic)](https://coveralls.io/github/barryfam/rithmic)
[![docs](https://img.shields.io/badge/docs-partial-yellow)](https://barryfam.io/rithmic/doc/rithmic/)

Optional `#![forbid(unsafe_code)]`

# rithmic
Algorithms, data structures, ease-of-use

### Utility
- [Print methods](https://barryfam.io/rithmic/doc/rithmic/trait.PrintMethods.html)
- Incremental [min](https://barryfam.io/rithmic/doc/rithmic/macro.imin.html)/[max](https://barryfam.io/rithmic/doc/rithmic/macro.imax.html)
- [Ordered pair](https://barryfam.io/rithmic/doc/rithmic/trait.OrdPair.html)
- [Option merge](https://barryfam.io/rithmic/doc/rithmic/trait.OptionMerge.html)
- [Result unwrap-any](https://barryfam.io/rithmic/doc/rithmic/trait.UnwrapAny.html)
- [Slice simultaneous mutable borrow](https://barryfam.io/rithmic/doc/rithmic/trait.PairMut.html)
- [`Rangelike`](https://barryfam.io/rithmic/doc/rithmic/trait.Rangelike.html), a trait accepting integers and integer ranges *(no doc yet)*
- [Unsigned integers as bitsets](https://barryfam.io/rithmic/doc/rithmic/trait.IntBitOps.html) *(no doc yet)*
- [Mixed-radix counter](https://barryfam.io/rithmic/doc/rithmic/type.OdometerLE.html)

### Algorithms
- [Binary search](https://barryfam.io/rithmic/doc/rithmic/fn.binary_search.html)
- [Ternary search](https://barryfam.io/rithmic/doc/rithmic/fn.ternary_search.html)
- [Index compression](https://barryfam.io/rithmic/doc/rithmic/trait.IndexCompress.html)
- [Prefix sums](https://barryfam.io/rithmic/doc/rithmic/trait.PrefixSums.html)
- [Graph](https://barryfam.io/rithmic/doc/rithmic/graph/struct.Graph.html)
    - Dijkstra, 0-1 Dijkstra, BFS *(no doc yet)*
    - Floyd-Warshall *(no doc yet)*l
    - DFS *(no doc yet)*S
    - Tarjan SCC *(no doc yet)*C
    - [Rooted subtree function](https://barryfam.io/rithmic/doc/rithmic/graph/struct.Graph.html#method.rooted_subtree_fn) "rerooting"
- [Mo's algorithm](https://barryfam.io/rithmic/doc/rithmic/fn.mo_algorithm.html)
- [Inversion counting](https://barryfam.io/rithmic/doc/rithmic/trait.CountInversions.html) *(no doc yet)*

### Data Structures
- [Tensor](https://barryfam.io/rithmic/doc/rithmic/struct.NdVec.html) ("*n*-dimensional vector") *(no doc yet)*
- [Boolean/bit vector](https://barryfam.io/rithmic/doc/rithmic/struct.BVec.html)
- [Meldable heap](https://barryfam.io/rithmic/doc/rithmic/struct.MeldHeap.html)
- [Generic augmented treap](https://barryfam.io/rithmic/doc/rithmic/aug_treap/struct.AugTreap.html) *(no doc yet)*
    - [Generic order-statistic treap](https://barryfam.io/rithmic/doc/rithmic/aug_treap/order_treap/struct.OrderTreap.html) *(no doc yet)*
        - [`List`](https://barryfam.io/rithmic/doc/rithmic/aug_treap/order_treap/struct.List.html), a sequence with *O*(log *n*) insert/delete *(no doc yet)*
        - [`SortedList`](https://barryfam.io/rithmic/doc/rithmic/aug_treap/order_treap/struct.SortedList.html), with rank- and order-statistics *(no doc yet)*
- [Generic sparse table](https://barryfam.io/rithmic/doc/rithmic/struct.SparseTable.html) *(no doc yet)*
- [Generic segment tree](https://barryfam.io/rithmic/doc/rithmic/monoid_ds/struct.SegTree.html) supports lazy updates and "segtree beats" *(no doc yet)*
- [*n*-dimensional Fenwick tree](https://barryfam.io/rithmic/doc/rithmic/monoid_ds/struct.NdFenwick.html) (binary-indexed tree) *(no doc yet)*
- [Union-find](https://barryfam.io/rithmic/doc/rithmic/struct.UnionFind.html)
- [XOR basis](https://barryfam.io/rithmic/doc/rithmic/xor_basis/index.html)

### Math
- [Triangular numbers](https://barryfam.io/rithmic/doc/rithmic/triangular_n/index.html)
- [All divisors of an integer](https://barryfam.io/rithmic/doc/rithmic/trait.Divisors.html#tymethod.divisors) using [`::primal::Sieve`](https://docs.rs/primal/latest/primal/struct.Sieve.html)
- [Factorials mod *n*](https://barryfam.io/rithmic/doc/rithmic/struct.FactorialTable.html) using [`::ac_library_rs::modint`](https://barryfam.io/rithmic/doc/ac_library_rs/modint/index.html) *(no doc yet)*
- [Polynomial arithmetic](https://barryfam.io/rithmic/doc/rithmic/polynomial/index.html) *(no doc yet)*
    - [Lagrange interpolation](https://barryfam.io/rithmic/doc/rithmic/polynomial/fn.lagrange_interpolation.html) *(no doc yet)*
- [A few extra methods](https://barryfam.io/rithmic/doc/rithmic/trait.Vector2DMore.html) for [`::euclid::Vector2D`](https://docs.rs/euclid/latest/euclid/struct.Vector2D.html) *(no doc yet)*

### Misc
- [`Gameboard`](https://barryfam.io/rithmic/doc/rithmic/gameboard/struct.Gameboard.html)
