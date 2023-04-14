#![doc = include_str!("../../README.md")]

#![cfg_attr(not(feature = "unsafe"), forbid(unsafe_code))]

#![feature(
    array_zip,
    btree_cursors,
    decl_macro,
    int_roundings,
    is_some_and,
    is_sorted,
    iter_collect_into,
    let_chains,
    rustc_attrs,
    specialization,
)]

#![allow(
    incomplete_features,
    clippy::collapsible_else_if,
    clippy::collapsible_if,
    clippy::missing_safety_doc,
    clippy::module_inception,
    clippy::needless_range_loop,
    clippy::nonminimal_bool,
    clippy::only_used_in_recursion,
    clippy::option_map_unit_fn,
    clippy::partialeq_to_none,
    clippy::reversed_empty_ranges,
    clippy::type_complexity,
    clippy::unnecessary_lazy_evaluations,
)]

#![warn(
    clippy::dbg_macro,
    clippy::imprecise_flops,
)]

pub use rithmic_impl::*;

    mod aho_corasick;
pub mod aug_treap;
pub mod binary_search;
    mod bvec;
    mod debug_or;
    mod disjoint_intervals;
pub mod gameboard;
    mod geometry;
pub mod graph;
    mod imin_imax;
    mod index_compress;
    mod insort;
    mod int_bitops;
    mod inversions;
    mod ipc;
    mod longest_transitive_subseq;
pub mod meld_heap;
    mod mo_algorithm;
    mod mod_factorial_table;
pub mod monoid_ds;
    mod ndvec;
    mod odometer;
    mod option_merge;
    mod ord_pair;
pub mod polynomial;
    mod prefix_sums;
    mod print_methods;
    mod rangelike;
    mod sieve_divisors;
    mod slice_get_sub;
    mod sparse_table;
    mod ternary_search;
pub mod triangular_n;
    mod union_find;
    mod unwrap_any;
    mod waterline;
pub mod xor_basis;

pub use aho_corasick::AhoCorasick;
pub use aug_treap::convex_hull_trick::{ConvexHullMax, ConvexHullMin};
pub use aug_treap::order_treap::{List, SortedList};
pub use binary_search::binary_search;
pub use bvec::BVec;
pub use debug_or::DebugOr;
pub use disjoint_intervals::DisjointIntervals;
pub use gameboard::Gameboard;
pub use geometry::Vector2DMore;
pub use imin_imax::{imax, imin};
pub use index_compress::{IndexCompress, IndexCompressed};
pub use insort::Insort;
pub use int_bitops::IntBitOps;
pub use inversions::CountInversions;
pub use ipc::Ipc;
pub use longest_transitive_subseq::LongestTransitiveSubseq;
pub use meld_heap::{MeldHeap, MeldMinHeap};
pub use mo_algorithm::{mo_algorithm, MoStep};
pub use mod_factorial_table::FactorialTable;
pub use monoid_ds::{monoid_ops::monoid_ops, NdFenwick, SegTree};
pub use ndvec::NdVec;
pub use odometer::{OdometerBE, OdometerLE};
pub use option_merge::OptionMerge;
pub use ord_pair::OrdPair;
pub use polynomial::{polynomial_add, polynomial_mul, polynomial_div, polynomial_div_exact, lagrange_interpolation};
pub use prefix_sums::PrefixSums;
pub use print_methods::PrintMethods;
pub use rangelike::Rangelike;
pub use sieve_divisors::Divisors;
pub use slice_get_sub::GetSub;
pub use sparse_table::SparseTable;
pub use ternary_search::ternary_search;
pub use triangular_n::{triangular_n, triangular_slice, triangular_steps};
pub use union_find::UnionFind;
pub use unwrap_any::UnwrapAny;
pub use waterline::Waterline;
pub use xor_basis::{XorBasis128, XorBasis64, XorBasis32, XorBasis16, XorBasis8};

pub mod prelude
{
    pub use rithmic_impl::*;

    pub use crate::{AhoCorasick, ConvexHullMax, ConvexHullMin, List, SortedList, binary_search, BVec, DebugOr, DisjointIntervals, Gameboard, Vector2DMore, imax, imin, IndexCompress, IndexCompressed, Insort, IntBitOps, CountInversions, LongestTransitiveSubseq, Ipc, MeldHeap, MeldMinHeap, mo_algorithm, MoStep, FactorialTable, monoid_ops, NdFenwick, SegTree, NdVec, OdometerBE, OdometerLE, OptionMerge, OrdPair, polynomial_add, polynomial_mul, polynomial_div, polynomial_div_exact, lagrange_interpolation, PrefixSums, PrintMethods, Rangelike, Divisors, GetSub, SparseTable, ternary_search, triangular_n, triangular_slice, triangular_steps, UnionFind, UnwrapAny, Waterline, XorBasis128, XorBasis64, XorBasis32, XorBasis16, XorBasis8};
}
