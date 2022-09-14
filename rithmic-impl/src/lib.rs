#![feature(
    box_patterns,
    box_syntax,
    decl_macro,
    extend_one,
    iter_collect_into,
    let_else,
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
mod struct_input;
mod transparency;

use proc_macro::TokenStream;

#[proc_macro_derive(CmpByKey, attributes(key))]
pub fn derive_cmp_by_key(item: TokenStream) -> TokenStream {
    cmp_by_key::derive(item)
}

#[proc_macro_attribute]
pub fn autofill(attr: TokenStream, item: TokenStream) -> TokenStream {
    autofill::autofill(attr, item)
}
#[proc_macro_attribute]
pub fn memoize(attr: TokenStream, item: TokenStream) -> TokenStream {
    autofill::memoize(attr, item)
}

#[proc_macro_attribute]
pub fn transparent(attr: TokenStream, item: TokenStream) -> TokenStream {
    transparency::transparent(attr, item)
}
#[proc_macro_attribute]
pub fn semitransparent(attr: TokenStream, item: TokenStream) -> TokenStream {
    transparency::semitransparent(attr, item)
}
#[proc_macro_attribute]
pub fn opaque(attr: TokenStream, item: TokenStream) -> TokenStream {
    transparency::opaque(attr, item)
}

#[proc_macro]
pub fn struct_input(item: TokenStream) -> TokenStream {
    struct_input::struct_input(item)
}
