use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields, FieldsNamed, FieldsUnnamed, Field, parse_quote, Type};

pub fn derive(item: TokenStream) -> TokenStream
{
    let DeriveInput{ident, generics, data, ..} = parse_macro_input!(item);

    fn has_key_attr(field: &Field) -> bool {
        field.attrs.iter().any( |a| {
            *a.path() == parse_quote! { key }
        })
    }

    let mut keys = Vec::<TokenStream2>::new();
    let mut key_types = Vec::<Type>::new();

    let Data::Struct(DataStruct{fields, ..}) = data else {
        panic!("non-structs are not supported");
    };
    match fields {
        Fields::Named(FieldsNamed{named, ..}) => {
            for f in &named {
                if has_key_attr(f) {
                    let f_ident = &f.ident;
                    keys.push(quote! { #f_ident } );
                    key_types.push(f.ty.clone());
                }
            }
        }
        Fields::Unnamed(FieldsUnnamed{unnamed, ..}) => {
            for (i, f) in unnamed.iter().enumerate() {
                if has_key_attr(f) {
                    keys.push(syn::Index::from(i).into_token_stream());
                    key_types.push(f.ty.clone());
                }
            }
        }
        Fields::Unit => panic!("struct must not be empty"),
    }
    assert!(!keys.is_empty(), "at least one #[key] tag is required");

    let q = quote! {
        impl #generics ::core::cmp::PartialEq for #ident #generics
        where #(
            #key_types : ::core::cmp::PartialEq
        ),*
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                #(
                    if self.#keys.ne(&other.#keys) { return false; }
                )*
                return true
            }
        }

        impl #generics ::core::cmp::Eq for #ident #generics
        where #(
            #key_types : ::core::cmp::Eq
        ),*
        {}

        impl #generics ::core::cmp::PartialOrd for #ident #generics
        where #(
            #key_types : ::core::cmp::PartialOrd
        ),*
        {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                #(
                    match self.#keys.partial_cmp(&other.#keys) {
                        None => return None,
                        Some(::core::cmp::Ordering::Equal) => {}
                        ord => return ord,
                    }
                )*
                Some(::core::cmp::Ordering::Equal)
            }
        }

        impl #generics ::core::cmp::Ord for #ident #generics
        where #(
            #key_types : ::core::cmp::Ord
        ),*
        {
            #[inline]
            fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                self.partial_cmp(other).unwrap()
            }
        }
    };

    // eprintln!("{}", q);
    q.into()
}
