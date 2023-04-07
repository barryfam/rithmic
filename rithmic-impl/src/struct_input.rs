use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::token::{Bracket, Paren};
use syn::{Ident, Token, Type, bracketed, parse_macro_input, braced, parenthesized, Expr};

mod kw {
    use syn::custom_keyword;

    custom_keyword!(Usize1);
    custom_keyword!(Isize1);
    custom_keyword!(Bytes);
    custom_keyword!(Chars);
}

struct Item {
    name: Ident,
    fields: Punctuated<InputField, Token![,]>,
}

struct InputField {
    ident: Ident,
    spec: InputSpec,
}

enum InputSpec {
    Type(Box<Type>),
    Tuple(Vec<InputSpec>),
    Vec(Box<InputSpec>),
    Usize1,
    Isize1,
    Bytes,
    Chars,
}

impl Parse for Item {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self>
    {
        let name = input.parse()?;

        let fields;
        braced!(fields in input);
        let fields = fields.parse_terminated(InputField::parse, Token![,])?;

        Ok(Self {
            name,
            fields
        })
    }
}

impl Parse for InputField {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: InputSpec = input.parse()?;
        Ok(Self { ident, spec: ty })
    }
}

impl Parse for InputSpec {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self>
    {
        if input.peek(Bracket) {
            let vec_spec;
            bracketed!(vec_spec in input);
            let vec_type: InputSpec = vec_spec.parse()?;
            if vec_spec.peek(Token![;]) {
                vec_spec.parse::<Token![;]>()?;
                vec_spec.parse::<Expr>()?;
            }
            Ok(InputSpec::Vec(Box::new(vec_type)))
        }
        else if input.peek(Paren) {
            let tuple_spec;
            parenthesized!(tuple_spec in input);
            let tuple_types = tuple_spec.parse_terminated(InputSpec::parse, Token![,])?;
            Ok(InputSpec::Tuple(tuple_types.into_iter().collect()))
        }
        else if input.peek(kw::Usize1) {
            input.parse::<kw::Usize1>()?;
            Ok(InputSpec::Usize1)
        }
        else if input.peek(kw::Isize1) {
            input.parse::<kw::Isize1>()?;
            Ok(InputSpec::Isize1)
        }
        else if input.peek(kw::Bytes) {
            input.parse::<kw::Bytes>()?;
            Ok(InputSpec::Bytes)
        }
        else if input.peek(kw::Chars) {
            input.parse::<kw::Chars>()?;
            Ok(InputSpec::Chars)
        }
        else {
            let ty: Type = input.parse()?;
            Ok(InputSpec::Type(Box::new(ty)))
        }
    }
}

impl InputSpec {
    fn to_type(&self) -> TokenStream2 {
        match self {
            InputSpec::Type(ty) => quote!{ #ty },
            InputSpec::Tuple(inners) => {
                let inners = inners.iter().map(Self::to_type);
                quote!{ ( #( #inners ),* ) }
            }
            InputSpec::Usize1 => quote!{ usize },
            InputSpec::Isize1 => quote!{ isize },
            InputSpec::Bytes => quote!{ Vec<u8> },
            InputSpec::Chars => quote!{ Vec<char> },
            InputSpec::Vec(inner) => {
                let inner = inner.to_type();
                quote!{ Vec< #inner > }
            }
        }
    }
}

pub fn struct_input(item: TokenStream) -> TokenStream
{
    let orig: TokenStream2 = item.clone().into();
    let item = parse_macro_input!(item as Item);
    let orig: TokenStream2 = orig.into_iter().skip(1).collect();  // skip struct name

    let name = item.name;
    let mut f_names = vec![];
    let mut f_types = vec![];
    for f in item.fields {
        f_names.push(f.ident);
        f_types.push(f.spec.to_type());
    }

    let input_macro_name = format!("{}_input", name).to_lowercase();
    let destruct_macro_name = format!("{}_destruct", name).to_lowercase();

    let input_macro_name = Ident::new(&input_macro_name, name.span());
    let destruct_macro_name = Ident::new(&destruct_macro_name, name.span());

    let q = quote! {
        struct #name {
            #( #f_names: #f_types ),*
        }

        #[allow(unused_macros)]
        #[rustc_macro_transparency = "transparent"]
        macro #input_macro_name() {
            #[allow(unused_imports)]
            use ::proconio::marker::*;
            ::proconio::input! #orig
        }

        #[allow(unused_macros)]
        #[rustc_macro_transparency = "transparent"]
        macro #destruct_macro_name() {
            #name {
                #( #f_names ),*
            }
        }
    };

    // eprintln!("{}", q);
    q.into()
}
