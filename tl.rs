#![feature(prelude_import, fmt_helpers_for_derive, no_coverage, proc_macro_internals, core_intrinsics, structural_match, rustc_attrs)]
#![doc(html_root_url = "https://docs.rs/tracing-attributes/0.1.23")]
#![doc(html_logo_url =
"https://raw.githubusercontent.com/tokio-rs/tracing/master/assets/logo-type.png",
issue_tracker_base_url = "https://github.com/tokio-rs/tracing/issues/")]
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms,
unreachable_pub, bad_style, dead_code, improper_ctypes,
non_shorthand_field_patterns, no_mangle_generic_items, overflowing_literals,
path_statements, patterns_in_fns_without_body, private_in_public,
unconditional_recursion, unused_allocation, unused_comparisons, unused_parens,
while_true)]#![allow(unused)]#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
extern crate proc_macro;extern crate alloc;use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, ItemFn, Signature, Visibility};mod attr {    use std::collections::HashSet;
    use syn::{
        punctuated::Punctuated, Expr, Ident, LitInt, LitStr, Path, Token,
    };
    use proc_macro2::TokenStream;
    use quote::{quote, quote_spanned, ToTokens};
    use syn::ext::IdentExt as _;
    use syn::parse::{Parse, ParseStream};
    pub(crate) struct InstrumentArgs {
        level: Option<Level>,
        pub(crate) name: Option<LitStr>,
        target: Option<LitStr>,
        pub(crate) parent: Option<Expr>,
        pub(crate) follows_from: Option<Expr>,
        pub(crate) skips: HashSet<Ident>,
        pub(crate) skip_all: bool,
        pub(crate) fields: Option<Fields>,
        pub(crate) err_mode: Option<FormatMode>,
        pub(crate) ret_mode: Option<FormatMode>,        parse_warnings: Vec<syn::Error>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for InstrumentArgs {
        #[inline]
        fn clone(&self) -> InstrumentArgs { loop { } }
    }
    #[automatically_derived]
    impl ::core::default::Default for InstrumentArgs {
        #[inline]
        fn default() -> InstrumentArgs { loop { } }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InstrumentArgs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { loop { } }
    }
    impl InstrumentArgs {
        pub(crate) fn level(&self) -> impl ToTokens {
            fn is_level(lit: &LitInt, expected: u64) -> bool {
                match lit.base10_parse::<u64>() {
                    Ok(value) => value == expected,
                    Err(_) => false,
                }
            }
            match &self.level {
                Some(Level::Str(ref lit)) if
                    lit.value().eq_ignore_ascii_case("trace") => {
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "tracing");
                        ::quote::__private::push_colon2(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "Level");
                        ::quote::__private::push_colon2(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "TRACE");
                        _s
                    }
                }
                Some(Level::Str(ref lit)) if
                    lit.value().eq_ignore_ascii_case("debug") => {
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "tracing");
                        ::quote::__private::push_colon2(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "Level");
                        ::quote::__private::push_colon2(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "DEBUG");
                        _s
                    }
                }
                Some(Level::Str(ref lit)) if
                    lit.value().eq_ignore_ascii_case("info") => {
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "tracing");
                        ::quote::__private::push_colon2(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "Level");
                        ::quote::__private::push_colon2(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "INFO");
                        _s
                    }
                }
                Some(Level::Str(ref lit)) if
                    lit.value().eq_ignore_ascii_case("warn") => {
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "tracing");
                        ::quote::__private::push_colon2(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "Level");
                        ::quote::__private::push_colon2(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "WARN");
                        _s
                    }
                }
                Some(Level::Str(ref lit)) if
                    lit.value().eq_ignore_ascii_case("error") => {
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "tracing");
                        ::quote::__private::push_colon2(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "Level");
                        ::quote::__private::push_colon2(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "ERROR");
                        _s
                    }
                }
                Some(Level::Int(ref lit)) if is_level(lit, 1) => {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "tracing");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Level");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "TRACE");
                    _s
                }
                Some(Level::Int(ref lit)) if is_level(lit, 2) => {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "tracing");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Level");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "DEBUG");
                    _s
                }
                Some(Level::Int(ref lit)) if is_level(lit, 3) => {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "tracing");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Level");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "INFO");
                    _s
                }
                Some(Level::Int(ref lit)) if is_level(lit, 4) => {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "tracing");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Level");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "WARN");
                    _s
                }
                Some(Level::Int(ref lit)) if is_level(lit, 5) => {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "tracing");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Level");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "ERROR");
                    _s
                }
                Some(Level::Path(ref pat)) => {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::ToTokens::to_tokens(&pat, &mut _s);
                    _s
                }
                Some(_) => {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "compile_error");
                    ::quote::__private::push_bang(&mut _s);
                    ::quote::__private::push_group(&mut _s,
                        ::quote::__private::Delimiter::Parenthesis,
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::parse(&mut _s,
                                "\"unknown verbosity level, expected one of \\\"trace\\\", \\\n                     \\\"debug\\\", \\\"info\\\", \\\"warn\\\", or \\\"error\\\", or a number 1-5\"");
                            _s
                        });
                    _s
                }
                None => {
                    let mut _s = ::quote::__private::TokenStream::new();
                    ::quote::__private::push_ident(&mut _s, "tracing");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "Level");
                    ::quote::__private::push_colon2(&mut _s);
                    ::quote::__private::push_ident(&mut _s, "INFO");
                    _s
                }
            }
        }
        pub(crate) fn target(&self) -> impl ToTokens {
            if let Some(ref target) = self.target {
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::ToTokens::to_tokens(&target, &mut _s);
                        _s
                    }
                } else {
                   {
                       let mut _s = ::quote::__private::TokenStream::new();
                       ::quote::__private::push_ident(&mut _s, "module_path");
                       ::quote::__private::push_bang(&mut _s);
                       ::quote::__private::push_group(&mut _s,
                           ::quote::__private::Delimiter::Parenthesis,
                           ::quote::__private::TokenStream::new());
                       _s
                   }
               }
        }
        pub(crate) fn warnings(&self) -> impl ToTokens {
            let warnings =
                self.parse_warnings.iter().map(|err|
                        {
                            let msg =
                                {
                                    let res =
                                        ::alloc::fmt::format(format_args!("found unrecognized input, {0}",
                                                err));
                                    res
                                };
                            let msg = LitStr::new(&msg, err.span());
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = err.span();
                                ::quote::__private::push_pound_spanned(&mut _s, _span);
                                ::quote::__private::push_group_spanned(&mut _s, _span,
                                    ::quote::__private::Delimiter::Bracket,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        let _span: ::quote::__private::Span = _span;
                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                            "warn");
                                        ::quote::__private::push_group_spanned(&mut _s, _span,
                                            ::quote::__private::Delimiter::Parenthesis,
                                            {
                                                let mut _s = ::quote::__private::TokenStream::new();
                                                let _span: ::quote::__private::Span = _span;
                                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                    "deprecated");
                                                _s
                                            });
                                        _s
                                    });
                                ::quote::__private::push_group_spanned(&mut _s, _span,
                                    ::quote::__private::Delimiter::Brace,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        let _span: ::quote::__private::Span = _span;
                                        ::quote::__private::push_pound_spanned(&mut _s, _span);
                                        ::quote::__private::push_group_spanned(&mut _s, _span,
                                            ::quote::__private::Delimiter::Bracket,
                                            {
                                                let mut _s = ::quote::__private::TokenStream::new();
                                                let _span: ::quote::__private::Span = _span;
                                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                    "deprecated");
                                                ::quote::__private::push_group_spanned(&mut _s, _span,
                                                    ::quote::__private::Delimiter::Parenthesis,
                                                    {
                                                        let mut _s = ::quote::__private::TokenStream::new();
                                                        let _span: ::quote::__private::Span = _span;
                                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                            "since");
                                                        ::quote::__private::push_eq_spanned(&mut _s, _span);
                                                        ::quote::__private::parse_spanned(&mut _s, _span,
                                                            "\"not actually deprecated\"");
                                                        ::quote::__private::push_comma_spanned(&mut _s, _span);
                                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                            "note");
                                                        ::quote::__private::push_eq_spanned(&mut _s, _span);
                                                        ::quote::ToTokens::to_tokens(&msg, &mut _s);
                                                        _s
                                                    });
                                                _s
                                            });
                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                            "const");
                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                            "TRACING_INSTRUMENT_WARNING");
                                        ::quote::__private::push_colon_spanned(&mut _s, _span);
                                        ::quote::__private::push_group_spanned(&mut _s, _span,
                                            ::quote::__private::Delimiter::Parenthesis,
                                            {
                                                let _: ::quote::__private::Span = _span;
                                                ::quote::__private::TokenStream::new()
                                            });
                                        ::quote::__private::push_eq_spanned(&mut _s, _span);
                                        ::quote::__private::push_group_spanned(&mut _s, _span,
                                            ::quote::__private::Delimiter::Parenthesis,
                                            {
                                                let _: ::quote::__private::Span = _span;
                                                ::quote::__private::TokenStream::new()
                                            });
                                        ::quote::__private::push_semi_spanned(&mut _s, _span);
                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                            "let");
                                        ::quote::__private::push_underscore_spanned(&mut _s, _span);
                                        ::quote::__private::push_eq_spanned(&mut _s, _span);
                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                            "TRACING_INSTRUMENT_WARNING");
                                        ::quote::__private::push_semi_spanned(&mut _s, _span);
                                        _s
                                    });
                                _s
                            }
                        });
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_group(&mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let has_iter =
                                ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut warnings, i) = warnings.quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let warnings =
                                    match warnings.next() {
                                        Some(_x) => ::quote::__private::RepInterp(_x),
                                        None => break,
                                    };
                                ::quote::ToTokens::to_tokens(&warnings, &mut _s);
                            }
                        }
                        _s
                    });
                _s
            }
        }
    }
    impl Parse for InstrumentArgs {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> { loop { } }
    }
    struct StrArg<T> {
        value: LitStr,
        _p: std::marker::PhantomData<T>,
    }
    impl<T: Parse> Parse for StrArg<T> {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> { loop { } }
    }
    struct ExprArg<T> {
        value: Expr,
        _p: std::marker::PhantomData<T>,
    }
    impl<T: Parse> Parse for ExprArg<T> {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> { loop { } }
    }
    struct Skips(HashSet<Ident>);
    impl Parse for Skips {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> { loop { } }
    }
    pub(crate) enum FormatMode { Default, Display, Debug, }
    #[automatically_derived]
    impl ::core::clone::Clone for FormatMode {
        #[inline]
        fn clone(&self) -> FormatMode { loop { } }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FormatMode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { loop { } }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for FormatMode {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () { loop { } }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FormatMode { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FormatMode {
        #[inline]
        fn eq(&self, other: &FormatMode) -> bool { loop { } }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for FormatMode { }
    #[automatically_derived]
    impl ::core::cmp::Eq for FormatMode {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () { loop { } }
    }
    impl Default for FormatMode {
        fn default() -> Self { loop { } }
    }
    impl Parse for FormatMode {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> { loop { } }
    }
    pub(crate) struct Fields(pub(crate) Punctuated<Field,
        ::syn::token::Comma>);
    #[automatically_derived]
    impl ::core::clone::Clone for Fields {
        #[inline]
        fn clone(&self) -> Fields { loop { } }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Fields {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { loop { } }
    }
    pub(crate) struct Field {
        pub(crate) name: Punctuated<Ident, ::syn::token::Dot>,
        pub(crate) value: Option<Expr>,
        pub(crate) kind: FieldKind,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Field {
        #[inline]
        fn clone(&self) -> Field { loop { } }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Field {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { loop { } }
    }
    pub(crate) enum FieldKind { Debug, Display, Value, }
    #[automatically_derived]
    impl ::core::clone::Clone for FieldKind {
        #[inline]
        fn clone(&self) -> FieldKind { loop { } }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FieldKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { loop { } }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for FieldKind { }
    #[automatically_derived]
    impl ::core::cmp::Eq for FieldKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () { loop { } }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FieldKind { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FieldKind {
        #[inline]
        fn eq(&self, other: &FieldKind) -> bool { loop { } }
    }
    impl Parse for Fields {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> { loop { } }
    }
    impl ToTokens for Fields {
        fn to_tokens(&self, tokens: &mut TokenStream) { loop { } }
    }
    impl Parse for Field {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> { loop { } }
    }
    impl ToTokens for Field {
        fn to_tokens(&self, tokens: &mut TokenStream) { loop { } }
    }
    impl ToTokens for FieldKind {
        fn to_tokens(&self, tokens: &mut TokenStream) { loop { } }
    }
    enum Level { Str(LitStr), Int(LitInt), Path(Path), }
    #[automatically_derived]
    impl ::core::clone::Clone for Level {
        #[inline]
        fn clone(&self) -> Level { loop { } }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Level {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { loop { } }
    }
    impl Parse for Level {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> { loop { } }
    }
    mod kw {
        #[allow(non_camel_case_types)]
        pub struct fields {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn fields<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span: __S) -> fields { loop { } }
        impl ::syn::__private::Default for fields {
            fn default() -> Self { loop { } }
        }
        impl ::syn::token::CustomToken for fields {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool { loop { } }
            fn display() -> &'static ::syn::__private::str { loop { } }
        }
        impl ::syn::parse::Parse for fields {
            fn parse(input: ::syn::parse::ParseStream)
                     -> ::syn::parse::Result<fields> { loop { } }
        }
        impl ::syn::__private::ToTokens for fields {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) { loop { } }
        }
        impl ::syn::__private::Copy for fields {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for fields {
            fn clone(&self) -> Self { loop { } }
        }
        impl ::syn::__private::Debug for fields {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                   -> ::syn::__private::fmt::Result { loop { } }
        }
        impl ::syn::__private::Eq for fields {}
        impl ::syn::__private::PartialEq for fields {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { loop { } }
        }
        impl ::syn::__private::Hash for fields {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) { loop { } }
        }
        #[allow(non_camel_case_types)]
        pub struct skip {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn skip<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span: __S) -> skip { loop { } }
        impl ::syn::__private::Default for skip {
            fn default() -> Self { loop { } }
        }
        impl ::syn::token::CustomToken for skip {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool { loop { } }
            fn display() -> &'static ::syn::__private::str { loop { } }
        }
        impl ::syn::parse::Parse for skip {
            fn parse(input: ::syn::parse::ParseStream)
                     -> ::syn::parse::Result<skip> { loop { } }
        }
        impl ::syn::__private::ToTokens for skip {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) { loop { } }
        }
        impl ::syn::__private::Copy for skip {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for skip {
            fn clone(&self) -> Self { loop { } }
        }
        impl ::syn::__private::Debug for skip {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                   -> ::syn::__private::fmt::Result { loop { } }
        }
        impl ::syn::__private::Eq for skip {}
        impl ::syn::__private::PartialEq for skip {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { loop { } }
        }
        impl ::syn::__private::Hash for skip {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) { loop { } }
        }
        #[allow(non_camel_case_types)]
        pub struct skip_all {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn skip_all<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span: __S) -> skip_all { loop { } }
        impl ::syn::__private::Default for skip_all {
            fn default() -> Self { loop { } }
        }
        impl ::syn::token::CustomToken for skip_all {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool { loop { } }
            fn display() -> &'static ::syn::__private::str { loop { } }
        }
        impl ::syn::parse::Parse for skip_all {
            fn parse(input: ::syn::parse::ParseStream)
                     -> ::syn::parse::Result<skip_all> { loop { } }
        }
        impl ::syn::__private::ToTokens for skip_all {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) { loop { } }
        }
        impl ::syn::__private::Copy for skip_all {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for skip_all {
            fn clone(&self) -> Self { loop { } }
        }
        impl ::syn::__private::Debug for skip_all {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                   -> ::syn::__private::fmt::Result { loop { } }
        }
        impl ::syn::__private::Eq for skip_all {}
        impl ::syn::__private::PartialEq for skip_all {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { loop { } }
        }
        impl ::syn::__private::Hash for skip_all {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) { loop { } }
        }
        #[allow(non_camel_case_types)]
        pub struct level {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn level<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span: __S) -> level { loop { } }
        impl ::syn::__private::Default for level {
            fn default() -> Self { loop { } }
        }
        impl ::syn::token::CustomToken for level {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool { loop { } }
            fn display() -> &'static ::syn::__private::str { loop { } }
        }
        impl ::syn::parse::Parse for level {
            fn parse(input: ::syn::parse::ParseStream)
                     -> ::syn::parse::Result<level> { loop { } }
        }
        impl ::syn::__private::ToTokens for level {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) { loop { } }
        }
        impl ::syn::__private::Copy for level {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for level {
            fn clone(&self) -> Self { loop { } }
        }
        impl ::syn::__private::Debug for level {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                   -> ::syn::__private::fmt::Result { loop { } }
        }
        impl ::syn::__private::Eq for level {}
        impl ::syn::__private::PartialEq for level {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { loop { } }
        }
        impl ::syn::__private::Hash for level {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) { loop { } }
        }
        #[allow(non_camel_case_types)]
        pub struct target {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn target<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span: __S) -> target { loop { } }
        impl ::syn::__private::Default for target {
            fn default() -> Self { loop { } }
        }
        impl ::syn::token::CustomToken for target {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool { loop { } }
            fn display() -> &'static ::syn::__private::str { loop { } }
        }
        impl ::syn::parse::Parse for target {
            fn parse(input: ::syn::parse::ParseStream)
                     -> ::syn::parse::Result<target> { loop { } }
        }
        impl ::syn::__private::ToTokens for target {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) { loop { } }
        }
        impl ::syn::__private::Copy for target {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for target {
            fn clone(&self) -> Self { loop { } }
        }
        impl ::syn::__private::Debug for target {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                   -> ::syn::__private::fmt::Result { loop { } }
        }
        impl ::syn::__private::Eq for target {}
        impl ::syn::__private::PartialEq for target {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { loop { } }
        }
        impl ::syn::__private::Hash for target {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) { loop { } }
        }
        #[allow(non_camel_case_types)]
        pub struct parent {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn parent<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span: __S) -> parent { loop { } }
        impl ::syn::__private::Default for parent {
            fn default() -> Self { loop { } }
        }
        impl ::syn::token::CustomToken for parent {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool { loop { } }
            fn display() -> &'static ::syn::__private::str { "`parent`" }
        }
        impl ::syn::parse::Parse for parent {
            fn parse(input: ::syn::parse::ParseStream)
                     -> ::syn::parse::Result<parent> { loop { } }
        }
        impl ::syn::__private::ToTokens for parent {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) { loop { } }
        }
        impl ::syn::__private::Copy for parent {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for parent {
            fn clone(&self) -> Self { loop { } }
        }
        impl ::syn::__private::Debug for parent {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                   -> ::syn::__private::fmt::Result { loop { } }
        }
        impl ::syn::__private::Eq for parent {}
        impl ::syn::__private::PartialEq for parent {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { loop { } }
        }
        impl ::syn::__private::Hash for parent {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) { loop { } }
        }
        #[allow(non_camel_case_types)]
        pub struct follows_from {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn follows_from<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span: __S) -> follows_from { loop { } }
        impl ::syn::__private::Default for follows_from {
            fn default() -> Self { loop { } }
        }
        impl ::syn::token::CustomToken for follows_from {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool { loop { } }
            fn display() -> &'static ::syn::__private::str { loop { } }
        }
        impl ::syn::parse::Parse for follows_from {
            fn parse(input: ::syn::parse::ParseStream)
                     -> ::syn::parse::Result<follows_from> { loop { } }
        }
        impl ::syn::__private::ToTokens for follows_from {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) { loop { } }
        }
        impl ::syn::__private::Copy for follows_from {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for follows_from {
            fn clone(&self) -> Self { loop { } }
        }
        impl ::syn::__private::Debug for follows_from {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                   -> ::syn::__private::fmt::Result { loop { } }
        }
        impl ::syn::__private::Eq for follows_from {}
        impl ::syn::__private::PartialEq for follows_from {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { loop { } }
        }
        impl ::syn::__private::Hash for follows_from {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) { loop { } }
        }
        #[allow(non_camel_case_types)]
        pub struct name {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn name<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span: __S) -> name { loop { } }
        impl ::syn::__private::Default for name {
            fn default() -> Self { loop { } }
        }
        impl ::syn::token::CustomToken for name {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool { loop { } }
            fn display() -> &'static ::syn::__private::str { loop { } }
        }
        impl ::syn::parse::Parse for name {
            fn parse(input: ::syn::parse::ParseStream)
                     -> ::syn::parse::Result<name> { loop { } }
        }
        impl ::syn::__private::ToTokens for name {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) { loop { } }
        }
        impl ::syn::__private::Copy for name {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for name {
            fn clone(&self) -> Self { loop { } }
        }
        impl ::syn::__private::Debug for name {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                   -> ::syn::__private::fmt::Result { loop { } }
        }
        impl ::syn::__private::Eq for name {}
        impl ::syn::__private::PartialEq for name {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { loop { } }
        }
        impl ::syn::__private::Hash for name {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) { loop { } }
        }
        #[allow(non_camel_case_types)]
        pub struct err {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn err<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span: __S) -> err { loop { } }
        impl ::syn::__private::Default for err {
            fn default() -> Self { loop { } }
        }
        impl ::syn::token::CustomToken for err {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool { loop { } }
            fn display() -> &'static ::syn::__private::str { loop { } }
        }
        impl ::syn::parse::Parse for err {
            fn parse(input: ::syn::parse::ParseStream)
                     -> ::syn::parse::Result<err> { loop { } }
        }
        impl ::syn::__private::ToTokens for err {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) { loop { } }
        }
        impl ::syn::__private::Copy for err {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for err {
            fn clone(&self) -> Self { loop { } }
        }
        impl ::syn::__private::Debug for err {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                   -> ::syn::__private::fmt::Result { loop { } }
        }
        impl ::syn::__private::Eq for err {}
        impl ::syn::__private::PartialEq for err {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { loop { } }
        }
        impl ::syn::__private::Hash for err {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) { loop { } }
        }
        #[allow(non_camel_case_types)]
        pub struct ret {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn ret<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span: __S) -> ret { loop { } }
        impl ::syn::__private::Default for ret {
            fn default() -> Self { loop { } }
        }
        impl ::syn::token::CustomToken for ret {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool { loop { } }
            fn display() -> &'static ::syn::__private::str { loop { } }
        }
        impl ::syn::parse::Parse for ret {
            fn parse(input: ::syn::parse::ParseStream)
                     -> ::syn::parse::Result<ret> { loop { } }
        }
        impl ::syn::__private::ToTokens for ret {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) { loop { } }
        }
        impl ::syn::__private::Copy for ret {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for ret {
            fn clone(&self) -> Self { loop { } }
        }
        impl ::syn::__private::Debug for ret {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                   -> ::syn::__private::fmt::Result { loop { } }
        }
        impl ::syn::__private::Eq for ret {}
        impl ::syn::__private::PartialEq for ret {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { loop { } }
        }
        impl ::syn::__private::Hash for ret {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) { loop { } }
        }
    }
}
mod expand {
    use std::iter;
    use proc_macro2::TokenStream;
    use quote::{quote, quote_spanned, ToTokens};
    use syn::visit_mut::VisitMut;
    use syn::{
        punctuated::Punctuated, spanned::Spanned, Block, Expr, ExprAsync,
        ExprCall, FieldPat, FnArg, Ident, Item, ItemFn, Pat, PatIdent,
        PatReference, PatStruct, PatTuple, PatTupleStruct, PatType, Path,
        ReturnType, Signature, Stmt, Token, Type, TypePath,
    };
    use crate::{
        attr::{Field, Fields, FormatMode, InstrumentArgs},
        MaybeItemFn, MaybeItemFnRef,
    };    pub(crate) fn gen_function<'a, B: ToTokens +
        'a>(input: MaybeItemFnRef<'a, B>, args: InstrumentArgs,
        instrumented_function_name: &str, self_type: Option<&TypePath>)
        -> proc_macro2::TokenStream {
        let MaybeItemFnRef { outer_attrs, inner_attrs, vis, sig, block } =
            input;
        let Signature {
                output,
                inputs: params,
                unsafety,
                asyncness,
                constness,
                abi,
                ident,
                generics: syn::Generics {
                    params: gen_params, where_clause,
                    ..
                }, .. } = sig;
        loop { }
    }
    fn gen_block<B: ToTokens>(block: &B,
        params: &Punctuated<FnArg, ::syn::token::Comma>, async_context: bool,
        mut args: InstrumentArgs, instrumented_function_name: &str, self_type: Option<&TypePath>) -> proc_macro2::TokenStream { loop { } }    enum RecordType {
        Value,
        Debug,
    }
    impl RecordType {        const TYPES_FOR_VALUE: &'static [&'static str] =
            &["bool", "str", "u8", "i8", "u16", "i16", "u32", "i32", "u64",
                        "i64", "f32", "f64", "usize", "isize", "NonZeroU8",
                        "NonZeroI8", "NonZeroU16", "NonZeroI16", "NonZeroU32",
                        "NonZeroI32", "NonZeroU64", "NonZeroI64", "NonZeroUsize",
                        "NonZeroIsize", "Wrapping"];
                             fn parse_from_ty(ty: &Type) -> Self { loop { } }
    }
    fn param_names(pat: Pat, record_type: RecordType) -> Box<dyn Iterator<Item = (Ident, RecordType)>> { loop { } }
    enum AsyncKind<'a> {        Function(&'a ItemFn),
        Async {
            async_expr: &'a ExprAsync,
            pinned_box: bool,
        },
    }
    pub(crate) struct AsyncInfo<'block> {
        source_stmt: &'block Stmt,
        kind: AsyncKind<'block>,
        self_type: Option<TypePath>,
        input: &'block ItemFn,
    }
    impl<'block> AsyncInfo<'block> {        pub(crate) fn from_fn(input: &'block ItemFn) -> Option<Self> { loop { } }
        pub(crate) fn gen_async(self, args: InstrumentArgs,
            instrumented_function_name: &str)
                                -> Result<proc_macro::TokenStream, syn::Error> { loop { } }
    }
    fn path_to_string(path: &Path) -> String { loop { } }
    struct IdentAndTypesRenamer<'a> {
        types: Vec<(&'a str, TypePath)>,
        idents: Vec<(Ident, Ident)>,
    }
    impl<'a> VisitMut for IdentAndTypesRenamer<'a> {
        #[allow(clippy :: cmp_owned)]
        fn visit_ident_mut(&mut self, id: &mut Ident) { loop { } }
        fn visit_type_mut(&mut self, ty: &mut Type) { loop { } }
    }
    struct AsyncTraitBlockReplacer<'a> {
        block: &'a Block,
        patched_block: Block,
    }
    impl<'a> VisitMut for AsyncTraitBlockReplacer<'a> {
        fn visit_block_mut(&mut self, i: &mut Block) { loop { } }
    }
    struct ImplTraitEraser;
    impl VisitMut for ImplTraitEraser {
        fn visit_type_mut(&mut self, t: &mut Type) { loop { } }
    }
    fn erase_impl_trait(ty: &Type) -> Type { loop { } }
}
#[proc_macro_attribute]
pub fn instrument(args: proc_macro::TokenStream,
    item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args =
        match ::syn::parse_macro_input::parse::<attr::InstrumentArgs>(args) {
            ::syn::__private::Ok(data) => data,
            ::syn::__private::Err(err) => {
                return ::syn::__private::TokenStream::from(err.to_compile_error());
            }
        };
    instrument_precise(args.clone(),
            item.clone()).unwrap_or_else(|_err|
            instrument_speculative(args, item))
}

fn instrument_speculative(args: attr::InstrumentArgs,
    item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input =
        match ::syn::parse_macro_input::parse::<MaybeItemFn>(item) {
            ::syn::__private::Ok(data) => data,
            ::syn::__private::Err(err) => {
                return ::syn::__private::TokenStream::from(err.to_compile_error());
            }
        };
    let instrumented_function_name = input.sig.ident.to_string();
    expand::gen_function(input.as_ref(), args,
            instrumented_function_name.as_str(), None).into()
}
fn instrument_precise(args: attr::InstrumentArgs,
    item: proc_macro::TokenStream)
    -> Result<proc_macro::TokenStream, syn::Error> {
    let input = syn::parse::<ItemFn>(item)?;
    let instrumented_function_name = input.sig.ident.to_string();
    if let Some(async_like) = expand::AsyncInfo::from_fn(&input) {
            return async_like.gen_async(args,
                    instrumented_function_name.as_str());
        }
    let input = MaybeItemFn::from(input);
    Ok(expand::gen_function(input.as_ref(), args,
                instrumented_function_name.as_str(), None).into())
}
struct MaybeItemFn {
    outer_attrs: Vec<Attribute>,
    inner_attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
    block: TokenStream,
}
#[automatically_derived]
impl ::core::fmt::Debug for MaybeItemFn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { loop { } }
}
#[automatically_derived]
impl ::core::clone::Clone for MaybeItemFn {
    #[inline]
    fn clone(&self) -> MaybeItemFn { loop { } }
}
impl MaybeItemFn {
    fn as_ref(&self) -> MaybeItemFnRef<'_, TokenStream> { loop { } }
}
impl Parse for MaybeItemFn {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> { loop { } }
}
impl From<ItemFn> for MaybeItemFn {
    fn from(f: ItemFn) -> Self { loop { } }
}

struct MaybeItemFnRef<'a, B: ToTokens> {
    outer_attrs: &'a Vec<Attribute>,
    inner_attrs: &'a Vec<Attribute>,
    vis: &'a Visibility,
    sig: &'a Signature,
    block: &'a B,
}
#[automatically_derived]
impl<'a, B: ::core::fmt::Debug + ToTokens> ::core::fmt::Debug for
    MaybeItemFnRef<'a, B> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result { loop { } }
}
#[automatically_derived]
impl<'a, B: ::core::clone::Clone + ToTokens> ::core::clone::Clone for
    MaybeItemFnRef<'a, B> {
    #[inline]
        fn clone(&self) -> MaybeItemFnRef<'a, B> { loop { } }
}
