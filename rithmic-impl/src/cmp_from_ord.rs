use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive(item: TokenStream) -> TokenStream
{
    let DeriveInput{ident, generics, ..} = parse_macro_input!(item);

    let q = quote! {
        impl #generics ::core::cmp::Eq for #ident #generics {}

        impl #generics ::core::cmp::PartialEq for #ident #generics
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.cmp(other) == ::core::cmp::Ordering::Equal
            }
        }

        impl #generics ::core::cmp::PartialOrd for #ident #generics
        {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
    };

    // eprintln!("{}", q);
    q.into()
}
