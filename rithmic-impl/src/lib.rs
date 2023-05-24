#![feature(
    box_patterns,
    decl_macro,
    extend_one,
    iter_collect_into,
)]

#![allow(
    clippy::module_inception,
)]

#![warn(
    clippy::dbg_macro,
)]

extern crate proc_macro;

mod autofill;
mod cmp_by_key;
mod cmp_from_ord;
mod struct_input;
mod transparency;

use proc_macro::TokenStream;



/**
Derive [`PartialEq`], [`Eq`], [`PartialOrd`] and [`Ord`] for a struct, but only based on a subset of its fields, marked by `#[key]`

This is particularly useful on structs where some of the other fields do not implement these traits.

# Examples
```
use rithmic::CmpByKey;

#[derive(CmpByKey)]
struct City {
    loc: (f64, f64),
    #[key] pop: u64
}

let a = [
    City { loc: (35.689722, 139.692222), pop: 13_988_129 },
    City { loc: (53.763056, -122.745278), pop: 74_003 },
    City { loc: (51.507222, -0.12750000), pop: 9_002_488 },
];

assert!(a[0] > a[2]);
assert!(a[1] < a[2]);
```
*/
#[proc_macro_derive(CmpByKey, attributes(key))]
pub fn derive_cmp_by_key(item: TokenStream) -> TokenStream {
    cmp_by_key::derive(item)
}



#[proc_macro_derive(CmpFromOrd)]
pub fn derive_cmp_from_ord(item: TokenStream) -> TokenStream {
    cmp_from_ord::derive(item)
}



/**
Create a wrapper macro with the same name as a given function, which automatically fills some of the later arguments with variables of the same name in the calling scope. The result is similar to a closure's external variable capture but without the disadvantages of closures (e.g. lack of recursion, borrow conflicts)

**Note:** `#![feature(rustc_attrs)]` **is required**

# Examples
```
#![feature(rustc_attrs)]

use rithmic::autofill;

let mut u = [3, 5, 7, 11, 13];
let mut v = vec![];

#[autofill(2..)]
fn duplicate(i: usize, count: usize, u: &[i32], v: &mut Vec<i32>)
{
    if count != 0 {
        v.push(u[i]);
        duplicate!(i, count - 1);  // note the !
    }
}

duplicate!(2, 4);

assert_eq!(v, vec![7, 7, 7, 7]);
```
*/
#[proc_macro_attribute]
pub fn autofill(attr: TokenStream, item: TokenStream) -> TokenStream {
    autofill::autofill(attr, item)
}



/**
[Memoize](https://en.wikipedia.org/wiki/Memoization) a function based on some or all of its arguments

Two types of storage are supported:

- **Static:** `#[memoize[5, 7]]` uses an [`NdVec`](https://barryfam.io/rithmic/doc/rithmic/struct.NdVec.html) of size `5` x `7` to memoize based on the first 2 function arguments, which must be castable to [`usize`]
- **Dynamic:** `#[memoize(..3)]` uses an [`AHashMap`](https://docs.rs/ahash/latest/ahash/struct.AHashMap.html) to memoize based on the first `3` function arguments, which must be [hashable](std::hash::Hash)

A wrapper macro with the same name as the function is created, and this macro must be used instead of calling the function directly. Arguments other than the memoized parameters are auto-filled with names from the calling scope; see [`macro@autofill`]

**Note:** `#![feature(rustc_attrs)]` **is required**

# Examples
Static
```
#![feature(rustc_attrs)]

use rithmic::memoize;

#[memoize[10]]
fn fibonacci(n: usize) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci!(n-1) + fibonacci!(n-2)
    }
}

assert_eq!(fibonacci!(9), 34);
```
Dynamic
```
#![feature(rustc_attrs)]

use rithmic::memoize;

#[memoize(..1)]
fn fibonacci(n: usize) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci!(n-1) + fibonacci!(n-2)
    }
}

assert_eq!(fibonacci!(20), 6765);
```
*/
#[proc_macro_attribute]
pub fn memoize(attr: TokenStream, item: TokenStream) -> TokenStream {
    autofill::memoize(attr, item)
}



/**
Change the hygiene of a declarative macro

- **Opaque:** Definition-site hygiene. Default for `macro`
- **Semitransparent:** Transparent only for local variables, labels and `$crate`. Default for `macro_rules!`
- **Transparent:** Call-site hygiene (aka "copy-paste" or "unhygienic"). Default for procedural macros

`#[transparent]` is shorthand for `#[rustc_macro_transparency = "transparent"]`; similar for the others

**Note:** `#![feature(rustc_attrs)]` **is required**

# Examples
```
#![feature(decl_macro, rustc_attrs)]

use rithmic::transparent;

let y = 100;

// opaque by default
macro f($x:expr) {
    $x + y
}

#[transparent]
macro g($x:expr) {
    $x + y
}

let y = 5;

assert_eq!(f!(3), 103);
assert_eq!(g!(3), 8);
```
*/
#[proc_macro_attribute]
pub fn transparent(attr: TokenStream, item: TokenStream) -> TokenStream {
    transparency::transparent(attr, item)
}

/// See [`macro@transparent`]
#[proc_macro_attribute]
pub fn semitransparent(attr: TokenStream, item: TokenStream) -> TokenStream {
    transparency::semitransparent(attr, item)
}

/// See [`macro@transparent`]
#[proc_macro_attribute]
pub fn opaque(attr: TokenStream, item: TokenStream) -> TokenStream {
    transparency::opaque(attr, item)
}



/**
Given a [`::proconio`](https://docs.rs/proconio/latest/proconio/) input specification and a `Name`, define:

- A `struct` definition called `Name`
- A macro called `name_input!` that calls proconio's `input!` with the given specification
- A macro called `name_destruct!` that expands to a destructure of the `Name` `struct` and can be used to either construct from free variables, or destructure into free variables (see example below)

**Note:** `#![feature(rustc_attrs)]` **and** `#![feature(decl_macro)]` **are required**

# Examples
```
#![feature(decl_macro, rustc_attrs)]

use rithmic::struct_input;

struct_input! {
    Student {
        name: String,
        exams_taken: usize,
        scores: [u8; exams_taken],
    }
}

// We now have access to:
// Student
// student_input!()
// student_destruct!() which expands to:
//      Student { name, exams_taken, scores }

// In a real program, this would read `name`, `exams_taken` and `scores` from stdin:
// student_input!();
# let name = String::from("Melissa");
# let exams_taken = 3;
# let scores = vec![85, 99, 71];

assert_eq!(name, "Melissa");
assert_eq!(exams_taken, 3);
assert_eq!(scores, vec![85, 99, 71]);

// use _destruct! to construct from free variables
let x: Student = student_destruct!();

assert_eq!(x.name, "Melissa");
assert_eq!(x.exams_taken, 3);
assert_eq!(x.scores, vec![85, 99, 71]);

// use _destruct! to destruct to free variables
let student_destruct!() = x;

assert_eq!(name, "Melissa");
assert_eq!(exams_taken, 3);
assert_eq!(scores, vec![85, 99, 71]);
```
*/
#[proc_macro]
pub fn struct_input(item: TokenStream) -> TokenStream {
    struct_input::struct_input(item)
}
