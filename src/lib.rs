//! Macros that expand the power and usefulness of the `sigma-types` crate.

#![expect(
    clippy::panic,
    reason = "TODO: how to return an error from a `TokenStream` function"
)]

use {
    core::iter::{empty, once},
    proc_macro::TokenStream,
    quote::{format_ident, quote},
    syn::{
        AttrStyle, Attribute, Block, Expr, ExprAssign, ExprCall, ExprPath, FnArg, GenericParam,
        Generics, Item, ItemFn, MacroDelimiter, Meta, MetaList, Pat, PatIdent, PatType, Path,
        PathArguments, PathSegment, ReturnType, Signature, Stmt, Type, TypePath, TypeReference,
        Visibility, parse_macro_input, punctuated::Punctuated,
    },
};

#[cfg(test)]
use {quickcheck as _, quickcheck_macros as _, sigma_types as _};

/// Add a test that ensures that
/// this type signature holds for all possible inputs.
/// # Panics
/// If not applied to a standalone (i.e. no `self`),
/// standard (i.e. non-`async`, non-`unsafe`, etc.) function.
#[proc_macro_attribute]
#[expect(
    clippy::too_many_lines,
    reason = "fuck off (or take issue with `syn`)--the logic is actually fairly simple"
)]
pub fn forall(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.clone().into(); // TODO: must be a better way
    let item = parse_macro_input!(input as Item);
    let Item::Fn(fn_item) = item else {
        panic!("`{input2}` is not a function");
    };

    let signature = fn_item.sig;
    let quickcheck_inputs: Punctuated<_, _> = signature
        .inputs
        .into_iter()
        .map(|arg_or_self| {
            let FnArg::Typed(arg_with_type) = arg_or_self else {
                panic!("Functions with `self` won't work");
            };
            let PatType {
                attrs,
                pat,
                colon_token,
                ty,
            } = arg_with_type;
            let (quickcheck_pattern, quickcheck_type) =
                if let Type::Reference(TypeReference { elem, .. }) = *ty {
                    (
                        Box::new(Pat::Ident(PatIdent {
                            attrs: attrs.clone(),
                            by_ref: Some(Default::default()),
                            mutability: None,
                            ident: {
                                #[expect(
                                    clippy::wildcard_enum_match_arm,
                                    reason = "huge number of cases"
                                )]
                                match *pat {
                                    Pat::Ident(PatIdent { ident, .. }) => ident,
                                    #[expect(clippy::todo, reason = "edge case, not world-ending")]
                                    _ => todo!("make up an ident"),
                                }
                            },
                            subpat: None,
                        })),
                        elem,
                    )
                } else {
                    (pat, ty)
                };
            FnArg::Typed(PatType {
                attrs,
                pat: quickcheck_pattern,
                colon_token,
                ty: quickcheck_type,
            })
        })
        .collect();
    let quickcheck_args = quickcheck_inputs
        .iter()
        .map(|fn_arg| {
            let FnArg::Typed(PatType {
                ref attrs,
                ref pat,
                colon_token: _,
                ty: _,
            }) = *fn_arg
            else {
                panic!("INTERNAL ERROR")
            };
            Expr::Path(ExprPath {
                attrs: attrs.clone(),
                qself: None,
                path: pat_to_path(pat),
            })
        })
        .collect();
    let quickcheck_signature = Signature {
        constness: None,
        abi: None,
        asyncness: None,
        fn_token: signature.fn_token,
        generics: Generics {
            lt_token: None,
            params: empty::<GenericParam>().collect(),
            gt_token: None,
            where_clause: None,
        },
        ident: format_ident!("forall_{}", signature.ident),
        inputs: quickcheck_inputs,
        output: ReturnType::Type(
            Default::default(),
            Box::new(Type::Path(TypePath {
                qself: None,
                path: Path {
                    leading_colon: Some(Default::default()),
                    segments: [
                        PathSegment {
                            arguments: PathArguments::None,
                            ident: format_ident!("quickcheck"),
                        },
                        PathSegment {
                            arguments: PathArguments::None,
                            ident: format_ident!("TestResult"),
                        },
                    ]
                    .into_iter()
                    .collect(),
                },
            })),
        ),
        paren_token: Default::default(),
        unsafety: None,
        variadic: None,
    };

    let quickcheck_fn = ItemFn {
        attrs: once(Attribute {
            pound_token: Default::default(),
            style: AttrStyle::Outer,
            bracket_token: Default::default(),
            meta: Meta::List(MetaList {
                path: Path {
                    leading_colon: None,
                    segments: once(PathSegment {
                        arguments: PathArguments::None,
                        ident: format_ident!("cfg"),
                    })
                    .collect(),
                },
                delimiter: MacroDelimiter::Paren(Default::default()),
                tokens: quote! { test },
            }),
        })
        .chain(
            fn_item
                .attrs
                .into_iter()
                .filter(|attr| matches!(attr.style, AttrStyle::Outer)),
        )
        .chain(once(Attribute {
            pound_token: Default::default(),
            style: AttrStyle::Outer,
            bracket_token: Default::default(),
            meta: Meta::Path(Path {
                leading_colon: Some(Default::default()),
                segments: [
                    PathSegment {
                        arguments: PathArguments::None,
                        ident: format_ident!("quickcheck_macros"),
                    },
                    PathSegment {
                        arguments: PathArguments::None,
                        ident: format_ident!("quickcheck"),
                    },
                ]
                .into_iter()
                .collect(),
            }),
        }))
        .collect(),
        vis: Visibility::Inherited,
        sig: quickcheck_signature,
        block: Box::new(Block {
            brace_token: Default::default(),
            stmts: vec![
                Stmt::Expr(
                    Expr::Assign(ExprAssign {
                        attrs: vec![],
                        left: Box::new(Expr::Path(ExprPath {
                            attrs: vec![],
                            qself: None,
                            path: Path {
                                leading_colon: None,
                                segments: once(PathSegment {
                                    arguments: PathArguments::None,
                                    ident: format_ident!("_"),
                                })
                                .collect(),
                            },
                        })),
                        eq_token: Default::default(),
                        right: Box::new(Expr::Call(ExprCall {
                            attrs: vec![],
                            func: Box::new(Expr::Path(ExprPath {
                                attrs: vec![],
                                qself: None,
                                path: Path {
                                    leading_colon: None,
                                    segments: once(PathSegment {
                                        arguments: PathArguments::None,
                                        ident: signature.ident,
                                    })
                                    .collect(),
                                },
                            })),
                            paren_token: Default::default(),
                            args: quickcheck_args,
                        })),
                    }),
                    Some(Default::default()),
                ),
                Stmt::Expr(
                    Expr::Call(ExprCall {
                        attrs: vec![],
                        func: Box::new(Expr::Path(ExprPath {
                            attrs: vec![],
                            qself: None,
                            path: Path {
                                leading_colon: Some(Default::default()),
                                segments: [
                                    PathSegment {
                                        arguments: PathArguments::None,
                                        ident: format_ident!("quickcheck"),
                                    },
                                    PathSegment {
                                        arguments: PathArguments::None,
                                        ident: format_ident!("TestResult"),
                                    },
                                    PathSegment {
                                        arguments: PathArguments::None,
                                        ident: format_ident!("passed"),
                                    },
                                ]
                                .into_iter()
                                .collect(),
                            },
                        })),
                        paren_token: Default::default(),
                        args: empty::<Expr>().collect(),
                    }),
                    None,
                ),
            ],
        }),
    };

    quote! {
        #input2 // retain the original input

        #quickcheck_fn
    }
    .into()
}

/// Extract an ident from a compatible pattern.
#[inline]
#[expect(clippy::single_call_fn, reason = "may be recursive")]
fn pat_to_path(pat: &Pat) -> Path {
    #[expect(clippy::wildcard_enum_match_arm, reason = "huge number of cases")]
    match *pat {
        Pat::Ident(PatIdent { ref ident, .. }) => Path {
            leading_colon: None,
            segments: once(PathSegment {
                ident: ident.clone(),
                arguments: PathArguments::None,
            })
            .collect(),
        },
        _ => panic!("INTERNAL ERROR"),
    }
}
