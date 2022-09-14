use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn transparent(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);
    (quote! {
        #[rustc_macro_transparency = "transparent"]
        #item
    }).into()
}

pub fn semitransparent(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);
    (quote! {
        #[rustc_macro_transparency = "semitransparent"]
        #item
    }).into()
}

pub fn opaque(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = TokenStream2::from(item);
    (quote! {
        #[rustc_macro_transparency = "opaque"]
        #item
    }).into()
}
