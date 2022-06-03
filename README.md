![Ferris the crab playing the bongos](https://barryfam.io/rithmic.png)

`#![forbid(unsafe_code)]`

# rithmic
Odds & ends: Algorithms, data structures, helpers

### Utility
- [Print methods](PrintMethods)
- Incremental [min](imin)/[max](imax)
- [Ordered pair](OrdPair)
- [Option merge](OptionMerge)
- [Result unwrap-any](UnwrapAny)
- [Slice simultaneous mutable borrow](PairMut)
- [`Rangelike`], a trait accepting integers and integer ranges *(no doc yet)*
- [Unsigned integers as bitsets](IntBitOps) *(no doc yet)*
- [Mixed-radix counter](OdometerLE)

### Algorithms
- [Binary search](binary_search())
- [Ternary search](ternary_search())
- [Index compression](IndexCompress)
- [Prefix sums](PrefixSums)
- [Graph](graph::Graph) *(no doc yet)*
    - Dijkstra, 0-1 Dijkstra, BFS
    - Floyd-Warshall
    - DFS
    - Tarjan SCC
    - Rooted subtrees
- [Mo's algorithm](mo_algorithm())
- [Inversion counting](CountInversions) *(no doc yet)*

### Data Structures
- [Tensor](NdVec) ("*n*-dimensional vector") *(no doc yet)*
- [Boolean/bit vector](BVec)
- [Meldable heap](MeldHeap)
- [Generic augmented treap](aug_treap::AugTreap) *(no doc yet)*
    - [Generic order-statistic treap](aug_treap::order_treap::OrderTreap) *(no doc yet)*
        - [`List`], a sequence with *O*(log *n*) insert/delete *(no doc yet)*
        - [`SortedList`], with rank- and order-statistics *(no doc yet)*
- [Generic sparse table](SparseTable) *(no doc yet)*
- [Generic segment tree](SegTree) supports lazy updates and "segtree beats" *(no doc yet)*
- [Union-find](UnionFind)
- [XOR basis](xor_basis)

### Math
- [Triangular numbers](mod@triangular_n)
- [All divisors of an integer](Divisors::divisors) using [`::primal::Sieve`](https://docs.rs/primal/latest/primal/struct.Sieve.html)
- [Factorials mod *n*](FactorialTable) using [`::ac_library_rs::modint`] *(no doc yet)*
- [Polynomial arithmetic](polynomial) *(no doc yet)*
    - [Lagrange interpolation](lagrange_interpolation) *(no doc yet)*
- [A few extra methods](Vector2DMore) for [`::euclid::Vector2D`](https://docs.rs/euclid/latest/euclid/struct.Vector2D.html) *(no doc yet)*

### Misc
- [`Gameboard`]
