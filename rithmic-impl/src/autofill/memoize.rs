use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{ExprRange, RangeLimits, bracketed, Expr, Token, ExprLit, Lit, parse_macro_input, ItemFn, Ident, ReturnType, FnArg, PatType, token, Type, Visibility};
use syn::parse::{ParseStream, Parse};

enum MemoizeAttr {
    Dynamic { n_args: Option<usize> },
    Static { dim: Vec<Expr> },
}
impl Parse for MemoizeAttr {
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        let c = input.fork();
        macro syntax_err() { return Err(c.error("invalid syntax. example: #[memoize(..2)]")) }

        if let Ok(ExprRange {
            start: None,
            limits,
            end,
            ..
        }) = input.fork().parse()
        {
            let _: ExprRange = input.parse()?;

            let mut n_args = match end {
                None => None,
                Some(box Expr::Lit(ExprLit{lit: Lit::Int(i), ..})) => Some(i.base10_parse()?),
                _ => syntax_err!(),
            };

            if matches!(limits, RangeLimits::Closed(_)) {
                n_args = Some(n_args.expect("an inclusive range must have an end bound") + 1);
            }

            Ok(Self::Dynamic { n_args })
        }
        else
        {
            let dim_bracketed;
            let dim = if input.peek(token::Bracket) {
                bracketed!(dim_bracketed in input);
                &dim_bracketed
            } else {
                input
            };
            let dim = dim.parse_terminated(Expr::parse, Token![,])?;
            let dim = dim.into_iter().collect();

            Ok(Self::Static { dim })
        }
    }
}
impl MemoizeAttr {
    fn n_args(&self) -> Option<usize> {
        match self {
            MemoizeAttr::Dynamic { n_args } => *n_args,
            MemoizeAttr::Static { dim } => Some(dim.len()),
        }
    }
}

pub fn memoize(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let attr = parse_macro_input!(attr as MemoizeAttr);

    let ItemFn {
        attrs: fn_attrs,
        vis: Visibility::Inherited,
        sig: mut fn_sig,
        block: fn_block,
    } = parse_macro_input!(item as ItemFn)
        else { panic!("memoize not supported on public functions") };

    let fn_name = fn_sig.ident.clone();
    let cache_name = Ident::new(&format!("__{}_cache", fn_name), fn_name.span());
    let inner_name = Ident::new(&format!("__{}_inner", fn_name), fn_name.span());

    fn_sig.ident = inner_name.clone();
    let fn_args = &mut fn_sig.inputs;

    let ReturnType::Type(_, box ret_type) = &fn_sig.output
        else { panic!("function must have a return type") };

    let n_args = attr.n_args().unwrap_or_else(|| fn_args.len());

    let cache_type = match &attr {
        MemoizeAttr::Dynamic { .. } => {
            let memo_arg_types = fn_args.iter().take(n_args).map(|arg| {
                let FnArg::Typed(PatType { box ty, .. }) = arg else { panic!() };
                ty
            });
            quote! { ::ahash::AHashMap<( #(#memo_arg_types),* ), #ret_type> }
        }
        MemoizeAttr::Static { .. } => {
            quote! { ::rithmic::NdVec<#n_args, Option<#ret_type>> }
        }
    };

    fn_args.push(syn::parse_quote! { mut #cache_name: &mut #cache_type });

    let mut memo_arg_names = Vec::<Ident>::new();
    let mut memo_arg_types = Vec::<Type>::new();
    let mut macro_pattern = Vec::<TokenStream2>::new();
    let mut macro_fn_args = Vec::<TokenStream2>::new();
    super::autofill::make_macro_args(
        fn_args.iter_mut(),
        n_args,
        &mut memo_arg_names,
        &mut memo_arg_types,
        &mut macro_pattern,
        &mut macro_fn_args,
    );

    let q = match attr {
        MemoizeAttr::Dynamic { .. } =>
        { quote!
        {
            let mut #cache_name = ::ahash::AHashMap::<( #(#memo_arg_types),* ), #ret_type>::new();

            #[rustc_macro_transparency = "transparent"]
            macro_rules! #fn_name {
                ( #(#macro_pattern),* ) => {
                    #inner_name( #(#macro_fn_args),* )
                };
            }

            #(#fn_attrs)*
            #fn_sig
            {
                let __key = ( #(#memo_arg_names.clone()),* );
                if let Some(__ret) = #cache_name.get(&__key) { return __ret.clone() }

                let __ret = (|| #fn_block )();

                #cache_name.insert(__key, __ret.clone());
                __ret
            }
        }}
        MemoizeAttr::Static { dim } =>
        { quote!
        {
            let mut #cache_name = ::rithmic::NdVec::<#n_args, Option<#ret_type>>::new([ #(#dim),* ]);

            #[rustc_macro_transparency = "transparent"]
            macro_rules! #fn_name {
                ( #(#macro_pattern),* ) => {
                    #inner_name( #(#macro_fn_args),* )
                };
            }

            #(#fn_attrs)*
            #fn_sig
            {
                let __key = [ #(#memo_arg_names as usize),* ];
                if let Some(ref __ret) = #cache_name[__key] { return __ret.clone() }

                let __ret = (|| #fn_block )();

                #cache_name[__key] = Some(__ret.clone());
                __ret
            }
        }}
    };

    // eprintln!("{}", q);
    q.into()
}
