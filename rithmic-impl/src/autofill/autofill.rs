use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::token::Mut;
use syn::{FnArg, ItemFn, Lit, Pat, Type, parse_macro_input, parse_quote, ExprRange, Expr, RangeLimits, ExprLit, Ident, PatType, PatIdent};

struct AutofillAttr {
    n_skip: usize
}
impl Parse for AutofillAttr {
    fn parse(input: ParseStream) -> syn::Result<Self>
    {
        let c = input.fork();
        macro syntax_err() { return Err(c.error("invalid syntax. example: #[autofill(2..)]")) }

        let ExprRange {
            start,
            limits: RangeLimits::HalfOpen(_),
            end: None,
            ..
        } = input.parse()? else { syntax_err!() };

        let n_skip = match start {
            None => 0,
            Some(box Expr::Lit(ExprLit{lit: Lit::Int(i), ..})) => i.base10_parse()?,
            _ => syntax_err!(),
        };

        Ok(AutofillAttr { n_skip })
    }
}

pub fn autofill(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let AutofillAttr { n_skip } = parse_macro_input!(attr);

    let mut item = parse_macro_input!(item as ItemFn);
    let fn_name = &item.sig.ident;
    let fn_args = &mut item.sig.inputs;

    assert!(n_skip < fn_args.len(),
        "range start ({}) greater than number of function arguments ({})", n_skip, fn_args.len());

    let mut macro_pattern = Vec::<TokenStream2>::new();
    let mut macro_fn_args = Vec::<TokenStream2>::new();
    make_macro_args(fn_args.iter_mut(), n_skip, &mut vec![], &mut vec![], &mut macro_pattern, &mut macro_fn_args);

    // Using `macro_rules!` until `macro` is stabilized to avoid forcing user to `#![feature(decl_macro)]`
    let q = quote!
    {
        #[rustc_macro_transparency = "transparent"]
        macro_rules! #fn_name {
            ( #(#macro_pattern),* ) => {
                #fn_name( #(#macro_fn_args),* )
            };
        }
        #item
    };

    // eprintln!("{}", q);
    q.into()
}

pub fn make_macro_args<'a>(
    mut fn_args: impl Iterator<Item=&'a mut FnArg>,
    n_passthrough: usize,
    passed_names: &mut Vec<Ident>,
    passed_types: &mut Vec<Type>,
    macro_pattern: &mut Vec<TokenStream2>,
    macro_fn_args: &mut Vec<TokenStream2>,
) {
    macro non_ident() { panic!("only single-identifier patterns like `x: type` are supported") }

    // macro_pattern = ($x0:expr, $x1:expr, ...)
    // macro_fn_args = ($x0, $x1, ..., <autofilled>)
    for i in 0..n_passthrough {
        let ident = format_ident!("x{}", i);
        macro_pattern.push(quote! { $#ident:expr } );
        macro_fn_args.push(quote! { $#ident } );
    }

    for fn_arg in fn_args.by_ref().take(n_passthrough) {
        let FnArg::Typed(PatType {
            pat: box Pat::Ident(PatIdent{ ident, .. }),
            ty,
            ..
        }) = fn_arg
            else { non_ident!() };

        passed_names.push(ident.clone());
        passed_types.push(*ty.clone());
    }

    for fn_arg in fn_args {
    match fn_arg {
        FnArg::Typed(pat_type) => {
            // `pattern: type`

            // If `f(x: &i32, y: &mut i32)`, `f!()` will call `f(&x, &mut y)` so that it works when `x` and `y` are not refs. We then rely on rust's auto-deref to fix `f(&&x, &mut &mut y)` calls
            //
            // This means we must also modify `f()`'s signature to `mut y`:
            // `f(x: &i32, mut y: &mut i32)`
            // so that `&mut y` (with type `&mut &mut`) does not cause an error

            let mut macro_fn_arg = TokenStream2::new();
            let mut force_mut = false;

            let mut ty0 = &pat_type.ty;
            while let box Type::Reference(ty1) = &ty0 {
                let ty_mut = ty1.mutability.is_some();
                macro_fn_arg.extend_one(if ty_mut {quote!{&mut}} else {quote!{&}});
                force_mut |= ty_mut;
                ty0 = &ty1.elem;
            }

            let box Pat::Ident(pat_ident) = &mut pat_type.pat else { non_ident!() };
            let ident = &pat_ident.ident;

            macro_fn_arg.extend_one(quote!{#ident});

            if force_mut && pat_ident.mutability.is_none() {
                pat_ident.attrs.push(parse_quote! { #[allow(unused_mut)] } );
                pat_ident.mutability = Some(Mut::default());
            }

            macro_fn_args.push(macro_fn_arg);
        }
        FnArg::Receiver(_) =>
            panic!("methods cannot be autofilled until Rust supports macros in field position")
    }}
}
