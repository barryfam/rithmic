![Ferris the crab playing the bongos](https://barryfam.io/rithmic/rithmic.png)

[![build](https://img.shields.io/github/actions/workflow/status/barryfam/rithmic/ci.yml?branch=master)](https://github.com/barryfam/rithmic/actions/workflows/ci.yml)
[![coverage](https://img.shields.io/coveralls/github/barryfam/rithmic)](https://coveralls.io/github/barryfam/rithmic)
[![docs](https://img.shields.io/badge/docs-partial-yellow)](https://barryfam.io/rithmic/doc/rithmic/)

Optional `#![forbid(unsafe_code)]`

# rithmic

### Utility
- [Print methods](https://barryfam.io/rithmic/doc/rithmic/trait.PrintMethods.html)
- Incremental [min](https://barryfam.io/rithmic/doc/rithmic/macro.imin.html) / [max](https://barryfam.io/rithmic/doc/rithmic/macro.imax.html)
- [Ordered pair](https://barryfam.io/rithmic/doc/rithmic/trait.OrdPair.html)
- [Option merge](https://barryfam.io/rithmic/doc/rithmic/trait.OptionMerge.html)
- [Result unwrap-any](https://barryfam.io/rithmic/doc/rithmic/trait.UnwrapAny.html)
- [`Rangelike`](https://barryfam.io/rithmic/doc/rithmic/trait.Rangelike.html), a trait accepting integers and integer ranges
- [Unsigned integers as bitsets](https://barryfam.io/rithmic/doc/rithmic/trait.IntBitOps.html)
- [Mixed-radix counter](https://barryfam.io/rithmic/doc/rithmic/type.OdometerLE.html)
- [Child process spawn and pipe](https://www.barryfam.io/rithmic/doc/rithmic/struct.Ipc.html)

### Procedural macros
- `#[autofill]` [Pseudo-capture function arguments](https://www.barryfam.io/rithmic/doc/rithmic/attr.autofill.html)
- `#[memoize]` [Function memoization (caching)](https://www.barryfam.io/rithmic/doc/rithmic/attr.memoize.html)
- `#[derive(CmpByKey)]` [Order structs by a subset of fields](https://www.barryfam.io/rithmic/doc/rithmic/derive.CmpByKey.html)
- `struct_input!` [Use `::proconio` to read data into a struct and set up helper macros](https://www.barryfam.io/rithmic/doc/rithmic/macro.struct_input.html)
- `#[transparent]`, `#[semitransparent]`, `#[opaque]` [Modify declarative macro hygiene](https://www.barryfam.io/rithmic/doc/rithmic/attr.transparent.html)

### Algorithms
- [Binary search](https://barryfam.io/rithmic/doc/rithmic/fn.binary_search.html)
- [Ternary search](https://barryfam.io/rithmic/doc/rithmic/fn.ternary_search.html)
- [Index compression](https://barryfam.io/rithmic/doc/rithmic/trait.IndexCompress.html)
- [Prefix sums](https://barryfam.io/rithmic/doc/rithmic/trait.PrefixSums.html)
- [Graph](https://barryfam.io/rithmic/doc/rithmic/graph/struct.Graph.html)
    - Dijkstra, 0-1 Dijkstra, BFS ⚙️
    - Floyd-Warshall ⚙️
    - DFS ⚙️
    - Tarjan SCC ⚙️
    - [Rooted subtree function](https://barryfam.io/rithmic/doc/rithmic/graph/struct.Graph.html#method.rooted_subtree_fn) "rerooting"
- [Mo's algorithm](https://barryfam.io/rithmic/doc/rithmic/fn.mo_algorithm.html)
- [Inversion counting](https://barryfam.io/rithmic/doc/rithmic/trait.CountInversions.html) ⚙️

### Data Structures
- [Tensor](https://barryfam.io/rithmic/doc/rithmic/struct.NdVec.html) ("*n*-dimensional vector") ⚙️
- [Boolean/bit vector](https://barryfam.io/rithmic/doc/rithmic/struct.BVec.html)
- [Meldable heap](https://barryfam.io/rithmic/doc/rithmic/meld_heap/type.MeldHeap.html)
- [Generic augmented treap](https://barryfam.io/rithmic/doc/rithmic/aug_treap/struct.AugTreap.html) ⚙️
    - [Generic order-statistic treap](https://barryfam.io/rithmic/doc/rithmic/aug_treap/order_treap/struct.OrderTreap.html) ⚙️
        - [`List`](https://barryfam.io/rithmic/doc/rithmic/aug_treap/order_treap/struct.List.html), a sequence with *O*(log *n*) insert/delete ⚙️
        - [`SortedList`](https://barryfam.io/rithmic/doc/rithmic/aug_treap/order_treap/struct.SortedList.html), with rank- and order-statistics ⚙️
- [Generic sparse table](https://barryfam.io/rithmic/doc/rithmic/struct.SparseTable.html) ⚙️
- [Generic segment tree](https://barryfam.io/rithmic/doc/rithmic/monoid_ds/struct.SegTree.html) supports lazy updates and "segtree beats" ⚙️
- [*n*-dimensional Fenwick tree](https://barryfam.io/rithmic/doc/rithmic/monoid_ds/struct.NdFenwick.html) (binary-indexed tree) ⚙️
- [Union-find](https://barryfam.io/rithmic/doc/rithmic/struct.UnionFind.html)
- [XOR basis](https://barryfam.io/rithmic/doc/rithmic/xor_basis/index.html)

### Math
- [Triangular numbers](https://barryfam.io/rithmic/doc/rithmic/triangular_n/index.html)
- [All divisors of an integer](https://barryfam.io/rithmic/doc/rithmic/trait.Divisors.html#tymethod.divisors) using [`::primal::Sieve`](https://docs.rs/primal/latest/primal/struct.Sieve.html)
- [Factorials mod *n*](https://barryfam.io/rithmic/doc/rithmic/struct.FactorialTable.html) using [`::ac_library_rs::modint`](https://barryfam.io/rithmic/doc/ac_library_rs/modint/index.html) ⚙️
- [Polynomial arithmetic](https://barryfam.io/rithmic/doc/rithmic/polynomial/index.html) ⚙️
    - [Lagrange interpolation](https://barryfam.io/rithmic/doc/rithmic/polynomial/fn.lagrange_interpolation.html) ⚙️
- [A few extra methods](https://barryfam.io/rithmic/doc/rithmic/trait.Vector2DMore.html) for [`::euclid::Vector2D`](https://docs.rs/euclid/latest/euclid/struct.Vector2D.html) ⚙️

### Misc
- [`Gameboard`](https://barryfam.io/rithmic/doc/rithmic/gameboard/struct.Gameboard.html)

*Items marked* ⚙️ *are working and tested but missing documentation - email or open an issue and I will add it*

# Features
`unsafe`
* By default, this crate uses `#![forbid(unsafe_code)]` to guarantee no undefined behavior (UB). The `unsafe` feature drops this attribute to enable some performance gains
* The library is still written such that UB should not be possible; however this becomes no longer verifiable by the compiler
* Currently, this affects: [`NdVec`](https://barryfam.io/rithmic/doc/rithmic/struct.NdVec.html), [`BVec`](https://barryfam.io/rithmic/doc/rithmic/struct.BVec.html), [`NdFenwick`](https://barryfam.io/rithmic/doc/rithmic/monoid_ds/struct.NdFenwick.html)
