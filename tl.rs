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
        fn clone(&self) -> InstrumentArgs {
            InstrumentArgs {
                level: ::core::clone::Clone::clone(&self.level),
                name: ::core::clone::Clone::clone(&self.name),
                target: ::core::clone::Clone::clone(&self.target),
                parent: ::core::clone::Clone::clone(&self.parent),
                follows_from: ::core::clone::Clone::clone(&self.follows_from),
                skips: ::core::clone::Clone::clone(&self.skips),
                skip_all: ::core::clone::Clone::clone(&self.skip_all),
                fields: ::core::clone::Clone::clone(&self.fields),
                err_mode: ::core::clone::Clone::clone(&self.err_mode),
                ret_mode: ::core::clone::Clone::clone(&self.ret_mode),
                parse_warnings: ::core::clone::Clone::clone(&self.parse_warnings),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for InstrumentArgs {
        #[inline]
        fn default() -> InstrumentArgs {
            InstrumentArgs {
                level: ::core::default::Default::default(),
                name: ::core::default::Default::default(),
                target: ::core::default::Default::default(),
                parent: ::core::default::Default::default(),
                follows_from: ::core::default::Default::default(),
                skips: ::core::default::Default::default(),
                skip_all: ::core::default::Default::default(),
                fields: ::core::default::Default::default(),
                err_mode: ::core::default::Default::default(),
                ret_mode: ::core::default::Default::default(),
                parse_warnings: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for InstrumentArgs {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ =
                &["level", "name", "target", "parent", "follows_from",
                            "skips", "skip_all", "fields", "err_mode", "ret_mode",
                            "parse_warnings"];
            let values: &[&dyn ::core::fmt::Debug] =
                &[&self.level, &self.name, &self.target, &self.parent,
                            &self.follows_from, &self.skips, &self.skip_all,
                            &self.fields, &self.err_mode, &self.ret_mode,
                            &&self.parse_warnings];
            ::core::fmt::Formatter::debug_struct_fields_finish(f,
                "InstrumentArgs", names, values)
        }
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
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            let mut args = Self::default();
            while !input.is_empty() {
                let lookahead = input.lookahead1();
                if lookahead.peek(kw::name) {
                        if args.name.is_some() {
                                return Err(input.error("expected only a single `name` argument"));
                            }
                        let name = input.parse::<StrArg<kw::name>>()?.value;
                        args.name = Some(name);
                    } else if lookahead.peek(LitStr) {
                       if args.name.is_some() {
                               return Err(input.error("expected only a single `name` argument"));
                           }
                       args.name = Some(input.parse()?);
                   } else if lookahead.peek(kw::target) {
                       if args.target.is_some() {
                               return Err(input.error("expected only a single `target` argument"));
                           }
                       let target = input.parse::<StrArg<kw::target>>()?.value;
                       args.target = Some(target);
                   } else if lookahead.peek(kw::parent) {
                       if args.target.is_some() {
                               return Err(input.error("expected only a single `parent` argument"));
                           }
                       let parent = input.parse::<ExprArg<kw::parent>>()?;
                       args.parent = Some(parent.value);
                   } else if lookahead.peek(kw::follows_from) {
                       if args.target.is_some() {
                               return Err(input.error("expected only a single `follows_from` argument"));
                           }
                       let follows_from =
                           input.parse::<ExprArg<kw::follows_from>>()?;
                       args.follows_from = Some(follows_from.value);
                   } else if lookahead.peek(kw::level) {
                       if args.level.is_some() {
                               return Err(input.error("expected only a single `level` argument"));
                           }
                       args.level = Some(input.parse()?);
                   } else if lookahead.peek(kw::skip) {
                       if !args.skips.is_empty() {
                               return Err(input.error("expected only a single `skip` argument"));
                           }
                       if args.skip_all {
                               return Err(input.error("expected either `skip` or `skip_all` argument"));
                           }
                       let Skips(skips) = input.parse()?;
                       args.skips = skips;
                   } else if lookahead.peek(kw::skip_all) {
                       if args.skip_all {
                               return Err(input.error("expected only a single `skip_all` argument"));
                           }
                       if !args.skips.is_empty() {
                               return Err(input.error("expected either `skip` or `skip_all` argument"));
                           }
                       let _ = input.parse::<kw::skip_all>()?;
                       args.skip_all = true;
                   } else if lookahead.peek(kw::fields) {
                       if args.fields.is_some() {
                               return Err(input.error("expected only a single `fields` argument"));
                           }
                       args.fields = Some(input.parse()?);
                   } else if lookahead.peek(kw::err) {
                       let _ = input.parse::<kw::err>();
                       let mode = FormatMode::parse(input)?;
                       args.err_mode = Some(mode);
                   } else if lookahead.peek(kw::ret) {
                       let _ = input.parse::<kw::ret>()?;
                       let mode = FormatMode::parse(input)?;
                       args.ret_mode = Some(mode);
                   } else if lookahead.peek(::syn::token::Comma) {
                       let _ = input.parse::<::syn::token::Comma>()?;
                   } else {
                       args.parse_warnings.push(lookahead.error());
                       let _ = input.parse::<proc_macro2::TokenTree>();
                   }
            }
            Ok(args)
        }
    }
    struct StrArg<T> {
        value: LitStr,
        _p: std::marker::PhantomData<T>,
    }
    impl<T: Parse> Parse for StrArg<T> {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            let _ = input.parse::<T>()?;
            let _ = input.parse::<::syn::token::Eq>()?;
            let value = input.parse()?;
            Ok(Self { value, _p: std::marker::PhantomData })
        }
    }
    struct ExprArg<T> {
        value: Expr,
        _p: std::marker::PhantomData<T>,
    }
    impl<T: Parse> Parse for ExprArg<T> {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            let _ = input.parse::<T>()?;
            let _ = input.parse::<::syn::token::Eq>()?;
            let value = input.parse()?;
            Ok(Self { value, _p: std::marker::PhantomData })
        }
    }
    struct Skips(HashSet<Ident>);
    impl Parse for Skips {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            let _ = input.parse::<kw::skip>();
            let content;
            let _ =
                match ::syn::__private::parse_parens(&input) {
                    ::syn::__private::Ok(parens) => {
                        content = parens.content;
                        parens.token
                    }
                    ::syn::__private::Err(error) => {
                        return ::syn::__private::Err(error);
                    }
                };
            let names: Punctuated<Ident, ::syn::token::Comma> =
                content.parse_terminated(Ident::parse_any)?;
            let mut skips = HashSet::new();
            for name in names {
                if skips.contains(&name) {
                        return Err(syn::Error::new(name.span(),
                                    "tried to skip the same field twice"));
                    } else { skips.insert(name); }
            }
            Ok(Self(skips))
        }
    }
    pub(crate) enum FormatMode { Default, Display, Debug, }
    #[automatically_derived]
    impl ::core::clone::Clone for FormatMode {
        #[inline]
        fn clone(&self) -> FormatMode {
            match self {
                FormatMode::Default => FormatMode::Default,
                FormatMode::Display => FormatMode::Display,
                FormatMode::Debug => FormatMode::Debug,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FormatMode {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f,
                match self {
                    FormatMode::Default => "Default",
                    FormatMode::Display => "Display",
                    FormatMode::Debug => "Debug",
                })
        }
    }
    #[automatically_derived]
    impl ::core::hash::Hash for FormatMode {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_tag, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FormatMode { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FormatMode {
        #[inline]
        fn eq(&self, other: &FormatMode) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for FormatMode { }
    #[automatically_derived]
    impl ::core::cmp::Eq for FormatMode {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    impl Default for FormatMode {
        fn default() -> Self { FormatMode::Default }
    }
    impl Parse for FormatMode {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            if !input.peek(syn::token::Paren) {
                    return Ok(FormatMode::default());
                }
            let content;
            let _ =
                match ::syn::__private::parse_parens(&input) {
                    ::syn::__private::Ok(parens) => {
                        content = parens.content;
                        parens.token
                    }
                    ::syn::__private::Err(error) => {
                        return ::syn::__private::Err(error);
                    }
                };
            let maybe_mode: Option<Ident> = content.parse()?;
            maybe_mode.map_or(Ok(FormatMode::default()),
                |ident|
                    {
                        match ident.to_string().as_str() {
                            "Debug" => Ok(FormatMode::Debug),
                            "Display" => Ok(FormatMode::Display),
                            _ =>
                                Err(syn::Error::new(ident.span(),
                                        "unknown error mode, must be Debug or Display")),
                        }
                    })
        }
    }
    pub(crate) struct Fields(pub(crate) Punctuated<Field,
        ::syn::token::Comma>);
    #[automatically_derived]
    impl ::core::clone::Clone for Fields {
        #[inline]
        fn clone(&self) -> Fields {
            Fields(::core::clone::Clone::clone(&self.0))
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Fields {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Fields",
                &&self.0)
        }
    }
    pub(crate) struct Field {
        pub(crate) name: Punctuated<Ident, ::syn::token::Dot>,
        pub(crate) value: Option<Expr>,
        pub(crate) kind: FieldKind,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Field {
        #[inline]
        fn clone(&self) -> Field {
            Field {
                name: ::core::clone::Clone::clone(&self.name),
                value: ::core::clone::Clone::clone(&self.value),
                kind: ::core::clone::Clone::clone(&self.kind),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Field {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(f, "Field",
                "name", &self.name, "value", &self.value, "kind", &&self.kind)
        }
    }
    pub(crate) enum FieldKind { Debug, Display, Value, }
    #[automatically_derived]
    impl ::core::clone::Clone for FieldKind {
        #[inline]
        fn clone(&self) -> FieldKind {
            match self {
                FieldKind::Debug => FieldKind::Debug,
                FieldKind::Display => FieldKind::Display,
                FieldKind::Value => FieldKind::Value,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for FieldKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f,
                match self {
                    FieldKind::Debug => "Debug",
                    FieldKind::Display => "Display",
                    FieldKind::Value => "Value",
                })
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for FieldKind { }
    #[automatically_derived]
    impl ::core::cmp::Eq for FieldKind {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for FieldKind { }
    #[automatically_derived]
    impl ::core::cmp::PartialEq for FieldKind {
        #[inline]
        fn eq(&self, other: &FieldKind) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
        }
    }
    impl Parse for Fields {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            let _ = input.parse::<kw::fields>();
            let content;
            let _ =
                match ::syn::__private::parse_parens(&input) {
                    ::syn::__private::Ok(parens) => {
                        content = parens.content;
                        parens.token
                    }
                    ::syn::__private::Err(error) => {
                        return ::syn::__private::Err(error);
                    }
                };
            let fields: Punctuated<_, ::syn::token::Comma> =
                content.parse_terminated(Field::parse)?;
            Ok(Self(fields))
        }
    }
    impl ToTokens for Fields {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            self.0.to_tokens(tokens)
        }
    }
    impl Parse for Field {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            let mut kind = FieldKind::Value;
            if input.peek(::syn::token::Rem) {
                    input.parse::<::syn::token::Rem>()?;
                    kind = FieldKind::Display;
                } else if input.peek(::syn::token::Question) {
                   input.parse::<::syn::token::Question>()?;
                   kind = FieldKind::Debug;
               };
            let name =
                Punctuated::parse_separated_nonempty_with(input,
                        Ident::parse_any)?;
            let value =
                if input.peek(::syn::token::Eq) {
                        input.parse::<::syn::token::Eq>()?;
                        if input.peek(::syn::token::Rem) {
                                input.parse::<::syn::token::Rem>()?;
                                kind = FieldKind::Display;
                            } else if input.peek(::syn::token::Question) {
                               input.parse::<::syn::token::Question>()?;
                               kind = FieldKind::Debug;
                           };
                        Some(input.parse()?)
                    } else { None };
            Ok(Self { name, value, kind })
        }
    }
    impl ToTokens for Field {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            if let Some(ref value) = self.value {
                    let name = &self.name;
                    let kind = &self.kind;
                    tokens.extend({
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::ToTokens::to_tokens(&name, &mut _s);
                            ::quote::__private::push_eq(&mut _s);
                            ::quote::ToTokens::to_tokens(&kind, &mut _s);
                            ::quote::ToTokens::to_tokens(&value, &mut _s);
                            _s
                        })
                } else if self.kind == FieldKind::Value {
                   let name = &self.name;
                   tokens.extend({
                           let mut _s = ::quote::__private::TokenStream::new();
                           ::quote::ToTokens::to_tokens(&name, &mut _s);
                           ::quote::__private::push_eq(&mut _s);
                           ::quote::__private::push_ident(&mut _s, "tracing");
                           ::quote::__private::push_colon2(&mut _s);
                           ::quote::__private::push_ident(&mut _s, "field");
                           ::quote::__private::push_colon2(&mut _s);
                           ::quote::__private::push_ident(&mut _s, "Empty");
                           _s
                       })
               } else {
                   self.kind.to_tokens(tokens);
                   self.name.to_tokens(tokens);
               }
        }
    }
    impl ToTokens for FieldKind {
        fn to_tokens(&self, tokens: &mut TokenStream) {
            match self {
                FieldKind::Debug =>
                    tokens.extend({
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_question(&mut _s);
                            _s
                        }),
                FieldKind::Display =>
                    tokens.extend({
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_rem(&mut _s);
                            _s
                        }),
                _ => {}
            }
        }
    }
    enum Level { Str(LitStr), Int(LitInt), Path(Path), }
    #[automatically_derived]
    impl ::core::clone::Clone for Level {
        #[inline]
        fn clone(&self) -> Level {
            match self {
                Level::Str(__self_0) =>
                    Level::Str(::core::clone::Clone::clone(__self_0)),
                Level::Int(__self_0) =>
                    Level::Int(::core::clone::Clone::clone(__self_0)),
                Level::Path(__self_0) =>
                    Level::Path(::core::clone::Clone::clone(__self_0)),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Level {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Level::Str(__self_0) =>
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Str",
                        &__self_0),
                Level::Int(__self_0) =>
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Int",
                        &__self_0),
                Level::Path(__self_0) =>
                    ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Path",
                        &__self_0),
            }
        }
    }
    impl Parse for Level {
        fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
            let _ = input.parse::<kw::level>()?;
            let _ = input.parse::<::syn::token::Eq>()?;
            let lookahead = input.lookahead1();
            if lookahead.peek(LitStr) {
                    Ok(Self::Str(input.parse()?))
                } else if lookahead.peek(LitInt) {
                   Ok(Self::Int(input.parse()?))
               } else if lookahead.peek(Ident) {
                   Ok(Self::Path(input.parse()?))
               } else { Err(lookahead.error()) }
        }
    }
    mod kw {
        #[allow(non_camel_case_types)]
        pub struct fields {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn fields<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span:
                __S) -> fields {
            fields { span: ::syn::__private::IntoSpans::into_spans(span)[0] }
        }
        impl ::syn::__private::Default for fields {
            fn default() -> Self {
                fields { span: ::syn::__private::Span::call_site() }
            }
        }
        impl ::syn::token::CustomToken for fields {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool {
                if let ::syn::__private::Some((ident, _rest)) = cursor.ident()
                        {
                        ident == "fields"
                    } else { false }
            }
            fn display() -> &'static ::syn::__private::str { "`fields`" }
        }
        impl ::syn::parse::Parse for fields {
            fn parse(input: ::syn::parse::ParseStream)
                -> ::syn::parse::Result<fields> {
                input.step(|cursor|
                        {
                            if let ::syn::__private::Some((ident, rest)) =
                                        cursor.ident() {
                                    if ident == "fields" {
                                            return ::syn::__private::Ok((fields { span: ident.span() },
                                                        rest));
                                        }
                                }
                            ::syn::__private::Err(cursor.error("expected `fields`"))
                        })
            }
        }
        impl ::syn::__private::ToTokens for fields {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                let ident = ::syn::Ident::new("fields", self.span);
                ::syn::__private::TokenStreamExt::append(tokens, ident);
            }
        }
        impl ::syn::__private::Copy for fields {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for fields {
            fn clone(&self) -> Self { *self }
        }
        impl ::syn::__private::Debug for fields {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f, "Keyword [fields]")
            }
        }
        impl ::syn::__private::Eq for fields {}
        impl ::syn::__private::PartialEq for fields {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { true }
        }
        impl ::syn::__private::Hash for fields {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
        }
        #[allow(non_camel_case_types)]
        pub struct skip {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn skip<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span:
                __S) -> skip {
            skip { span: ::syn::__private::IntoSpans::into_spans(span)[0] }
        }
        impl ::syn::__private::Default for skip {
            fn default() -> Self {
                skip { span: ::syn::__private::Span::call_site() }
            }
        }
        impl ::syn::token::CustomToken for skip {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool {
                if let ::syn::__private::Some((ident, _rest)) = cursor.ident()
                        {
                        ident == "skip"
                    } else { false }
            }
            fn display() -> &'static ::syn::__private::str { "`skip`" }
        }
        impl ::syn::parse::Parse for skip {
            fn parse(input: ::syn::parse::ParseStream)
                -> ::syn::parse::Result<skip> {
                input.step(|cursor|
                        {
                            if let ::syn::__private::Some((ident, rest)) =
                                        cursor.ident() {
                                    if ident == "skip" {
                                            return ::syn::__private::Ok((skip { span: ident.span() },
                                                        rest));
                                        }
                                }
                            ::syn::__private::Err(cursor.error("expected `skip`"))
                        })
            }
        }
        impl ::syn::__private::ToTokens for skip {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                let ident = ::syn::Ident::new("skip", self.span);
                ::syn::__private::TokenStreamExt::append(tokens, ident);
            }
        }
        impl ::syn::__private::Copy for skip {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for skip {
            fn clone(&self) -> Self { *self }
        }
        impl ::syn::__private::Debug for skip {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f, "Keyword [skip]")
            }
        }
        impl ::syn::__private::Eq for skip {}
        impl ::syn::__private::PartialEq for skip {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { true }
        }
        impl ::syn::__private::Hash for skip {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
        }
        #[allow(non_camel_case_types)]
        pub struct skip_all {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn skip_all<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span:
                __S) -> skip_all {
            skip_all {
                span: ::syn::__private::IntoSpans::into_spans(span)[0],
            }
        }
        impl ::syn::__private::Default for skip_all {
            fn default() -> Self {
                skip_all { span: ::syn::__private::Span::call_site() }
            }
        }
        impl ::syn::token::CustomToken for skip_all {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool {
                if let ::syn::__private::Some((ident, _rest)) = cursor.ident()
                        {
                        ident == "skip_all"
                    } else { false }
            }
            fn display() -> &'static ::syn::__private::str { "`skip_all`" }
        }
        impl ::syn::parse::Parse for skip_all {
            fn parse(input: ::syn::parse::ParseStream)
                -> ::syn::parse::Result<skip_all> {
                input.step(|cursor|
                        {
                            if let ::syn::__private::Some((ident, rest)) =
                                        cursor.ident() {
                                    if ident == "skip_all" {
                                            return ::syn::__private::Ok((skip_all {
                                                            span: ident.span(),
                                                        }, rest));
                                        }
                                }
                            ::syn::__private::Err(cursor.error("expected `skip_all`"))
                        })
            }
        }
        impl ::syn::__private::ToTokens for skip_all {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                let ident = ::syn::Ident::new("skip_all", self.span);
                ::syn::__private::TokenStreamExt::append(tokens, ident);
            }
        }
        impl ::syn::__private::Copy for skip_all {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for skip_all {
            fn clone(&self) -> Self { *self }
        }
        impl ::syn::__private::Debug for skip_all {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f,
                    "Keyword [skip_all]")
            }
        }
        impl ::syn::__private::Eq for skip_all {}
        impl ::syn::__private::PartialEq for skip_all {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { true }
        }
        impl ::syn::__private::Hash for skip_all {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
        }
        #[allow(non_camel_case_types)]
        pub struct level {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn level<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span:
                __S) -> level {
            level { span: ::syn::__private::IntoSpans::into_spans(span)[0] }
        }
        impl ::syn::__private::Default for level {
            fn default() -> Self {
                level { span: ::syn::__private::Span::call_site() }
            }
        }
        impl ::syn::token::CustomToken for level {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool {
                if let ::syn::__private::Some((ident, _rest)) = cursor.ident()
                        {
                        ident == "level"
                    } else { false }
            }
            fn display() -> &'static ::syn::__private::str { "`level`" }
        }
        impl ::syn::parse::Parse for level {
            fn parse(input: ::syn::parse::ParseStream)
                -> ::syn::parse::Result<level> {
                input.step(|cursor|
                        {
                            if let ::syn::__private::Some((ident, rest)) =
                                        cursor.ident() {
                                    if ident == "level" {
                                            return ::syn::__private::Ok((level { span: ident.span() },
                                                        rest));
                                        }
                                }
                            ::syn::__private::Err(cursor.error("expected `level`"))
                        })
            }
        }
        impl ::syn::__private::ToTokens for level {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                let ident = ::syn::Ident::new("level", self.span);
                ::syn::__private::TokenStreamExt::append(tokens, ident);
            }
        }
        impl ::syn::__private::Copy for level {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for level {
            fn clone(&self) -> Self { *self }
        }
        impl ::syn::__private::Debug for level {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f, "Keyword [level]")
            }
        }
        impl ::syn::__private::Eq for level {}
        impl ::syn::__private::PartialEq for level {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { true }
        }
        impl ::syn::__private::Hash for level {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
        }
        #[allow(non_camel_case_types)]
        pub struct target {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn target<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span:
                __S) -> target {
            target { span: ::syn::__private::IntoSpans::into_spans(span)[0] }
        }
        impl ::syn::__private::Default for target {
            fn default() -> Self {
                target { span: ::syn::__private::Span::call_site() }
            }
        }
        impl ::syn::token::CustomToken for target {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool {
                if let ::syn::__private::Some((ident, _rest)) = cursor.ident()
                        {
                        ident == "target"
                    } else { false }
            }
            fn display() -> &'static ::syn::__private::str { "`target`" }
        }
        impl ::syn::parse::Parse for target {
            fn parse(input: ::syn::parse::ParseStream)
                -> ::syn::parse::Result<target> {
                input.step(|cursor|
                        {
                            if let ::syn::__private::Some((ident, rest)) =
                                        cursor.ident() {
                                    if ident == "target" {
                                            return ::syn::__private::Ok((target { span: ident.span() },
                                                        rest));
                                        }
                                }
                            ::syn::__private::Err(cursor.error("expected `target`"))
                        })
            }
        }
        impl ::syn::__private::ToTokens for target {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                let ident = ::syn::Ident::new("target", self.span);
                ::syn::__private::TokenStreamExt::append(tokens, ident);
            }
        }
        impl ::syn::__private::Copy for target {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for target {
            fn clone(&self) -> Self { *self }
        }
        impl ::syn::__private::Debug for target {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f, "Keyword [target]")
            }
        }
        impl ::syn::__private::Eq for target {}
        impl ::syn::__private::PartialEq for target {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { true }
        }
        impl ::syn::__private::Hash for target {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
        }
        #[allow(non_camel_case_types)]
        pub struct parent {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn parent<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span:
                __S) -> parent {
            parent { span: ::syn::__private::IntoSpans::into_spans(span)[0] }
        }
        impl ::syn::__private::Default for parent {
            fn default() -> Self {
                parent { span: ::syn::__private::Span::call_site() }
            }
        }
        impl ::syn::token::CustomToken for parent {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool {
                if let ::syn::__private::Some((ident, _rest)) = cursor.ident()
                        {
                        ident == "parent"
                    } else { false }
            }
            fn display() -> &'static ::syn::__private::str { "`parent`" }
        }
        impl ::syn::parse::Parse for parent {
            fn parse(input: ::syn::parse::ParseStream)
                -> ::syn::parse::Result<parent> {
                input.step(|cursor|
                        {
                            if let ::syn::__private::Some((ident, rest)) =
                                        cursor.ident() {
                                    if ident == "parent" {
                                            return ::syn::__private::Ok((parent { span: ident.span() },
                                                        rest));
                                        }
                                }
                            ::syn::__private::Err(cursor.error("expected `parent`"))
                        })
            }
        }
        impl ::syn::__private::ToTokens for parent {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                let ident = ::syn::Ident::new("parent", self.span);
                ::syn::__private::TokenStreamExt::append(tokens, ident);
            }
        }
        impl ::syn::__private::Copy for parent {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for parent {
            fn clone(&self) -> Self { *self }
        }
        impl ::syn::__private::Debug for parent {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f, "Keyword [parent]")
            }
        }
        impl ::syn::__private::Eq for parent {}
        impl ::syn::__private::PartialEq for parent {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { true }
        }
        impl ::syn::__private::Hash for parent {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
        }
        #[allow(non_camel_case_types)]
        pub struct follows_from {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn follows_from<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span:
                __S) -> follows_from {
            follows_from {
                span: ::syn::__private::IntoSpans::into_spans(span)[0],
            }
        }
        impl ::syn::__private::Default for follows_from {
            fn default() -> Self {
                follows_from { span: ::syn::__private::Span::call_site() }
            }
        }
        impl ::syn::token::CustomToken for follows_from {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool {
                if let ::syn::__private::Some((ident, _rest)) = cursor.ident()
                        {
                        ident == "follows_from"
                    } else { false }
            }
            fn display() -> &'static ::syn::__private::str {
                "`follows_from`"
            }
        }
        impl ::syn::parse::Parse for follows_from {
            fn parse(input: ::syn::parse::ParseStream)
                -> ::syn::parse::Result<follows_from> {
                input.step(|cursor|
                        {
                            if let ::syn::__private::Some((ident, rest)) =
                                        cursor.ident() {
                                    if ident == "follows_from" {
                                            return ::syn::__private::Ok((follows_from {
                                                            span: ident.span(),
                                                        }, rest));
                                        }
                                }
                            ::syn::__private::Err(cursor.error("expected `follows_from`"))
                        })
            }
        }
        impl ::syn::__private::ToTokens for follows_from {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                let ident = ::syn::Ident::new("follows_from", self.span);
                ::syn::__private::TokenStreamExt::append(tokens, ident);
            }
        }
        impl ::syn::__private::Copy for follows_from {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for follows_from {
            fn clone(&self) -> Self { *self }
        }
        impl ::syn::__private::Debug for follows_from {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f,
                    "Keyword [follows_from]")
            }
        }
        impl ::syn::__private::Eq for follows_from {}
        impl ::syn::__private::PartialEq for follows_from {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { true }
        }
        impl ::syn::__private::Hash for follows_from {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
        }
        #[allow(non_camel_case_types)]
        pub struct name {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn name<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span:
                __S) -> name {
            name { span: ::syn::__private::IntoSpans::into_spans(span)[0] }
        }
        impl ::syn::__private::Default for name {
            fn default() -> Self {
                name { span: ::syn::__private::Span::call_site() }
            }
        }
        impl ::syn::token::CustomToken for name {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool {
                if let ::syn::__private::Some((ident, _rest)) = cursor.ident()
                        {
                        ident == "name"
                    } else { false }
            }
            fn display() -> &'static ::syn::__private::str { "`name`" }
        }
        impl ::syn::parse::Parse for name {
            fn parse(input: ::syn::parse::ParseStream)
                -> ::syn::parse::Result<name> {
                input.step(|cursor|
                        {
                            if let ::syn::__private::Some((ident, rest)) =
                                        cursor.ident() {
                                    if ident == "name" {
                                            return ::syn::__private::Ok((name { span: ident.span() },
                                                        rest));
                                        }
                                }
                            ::syn::__private::Err(cursor.error("expected `name`"))
                        })
            }
        }
        impl ::syn::__private::ToTokens for name {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                let ident = ::syn::Ident::new("name", self.span);
                ::syn::__private::TokenStreamExt::append(tokens, ident);
            }
        }
        impl ::syn::__private::Copy for name {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for name {
            fn clone(&self) -> Self { *self }
        }
        impl ::syn::__private::Debug for name {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f, "Keyword [name]")
            }
        }
        impl ::syn::__private::Eq for name {}
        impl ::syn::__private::PartialEq for name {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { true }
        }
        impl ::syn::__private::Hash for name {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
        }
        #[allow(non_camel_case_types)]
        pub struct err {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn err<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span:
                __S) -> err {
            err { span: ::syn::__private::IntoSpans::into_spans(span)[0] }
        }
        impl ::syn::__private::Default for err {
            fn default() -> Self {
                err { span: ::syn::__private::Span::call_site() }
            }
        }
        impl ::syn::token::CustomToken for err {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool {
                if let ::syn::__private::Some((ident, _rest)) = cursor.ident()
                        {
                        ident == "err"
                    } else { false }
            }
            fn display() -> &'static ::syn::__private::str { "`err`" }
        }
        impl ::syn::parse::Parse for err {
            fn parse(input: ::syn::parse::ParseStream)
                -> ::syn::parse::Result<err> {
                input.step(|cursor|
                        {
                            if let ::syn::__private::Some((ident, rest)) =
                                        cursor.ident() {
                                    if ident == "err" {
                                            return ::syn::__private::Ok((err { span: ident.span() },
                                                        rest));
                                        }
                                }
                            ::syn::__private::Err(cursor.error("expected `err`"))
                        })
            }
        }
        impl ::syn::__private::ToTokens for err {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                let ident = ::syn::Ident::new("err", self.span);
                ::syn::__private::TokenStreamExt::append(tokens, ident);
            }
        }
        impl ::syn::__private::Copy for err {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for err {
            fn clone(&self) -> Self { *self }
        }
        impl ::syn::__private::Debug for err {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f, "Keyword [err]")
            }
        }
        impl ::syn::__private::Eq for err {}
        impl ::syn::__private::PartialEq for err {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { true }
        }
        impl ::syn::__private::Hash for err {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
        }
        #[allow(non_camel_case_types)]
        pub struct ret {
            pub span: ::syn::__private::Span,
        }
        #[doc(hidden)]
        #[allow(dead_code, non_snake_case)]
        pub fn ret<__S: ::syn::__private::IntoSpans<[::syn::__private::Span; 1]>>(span:
                __S) -> ret {
            ret { span: ::syn::__private::IntoSpans::into_spans(span)[0] }
        }
        impl ::syn::__private::Default for ret {
            fn default() -> Self {
                ret { span: ::syn::__private::Span::call_site() }
            }
        }
        impl ::syn::token::CustomToken for ret {
            fn peek(cursor: ::syn::buffer::Cursor) -> ::syn::__private::bool {
                if let ::syn::__private::Some((ident, _rest)) = cursor.ident()
                        {
                        ident == "ret"
                    } else { false }
            }
            fn display() -> &'static ::syn::__private::str { "`ret`" }
        }
        impl ::syn::parse::Parse for ret {
            fn parse(input: ::syn::parse::ParseStream)
                -> ::syn::parse::Result<ret> {
                input.step(|cursor|
                        {
                            if let ::syn::__private::Some((ident, rest)) =
                                        cursor.ident() {
                                    if ident == "ret" {
                                            return ::syn::__private::Ok((ret { span: ident.span() },
                                                        rest));
                                        }
                                }
                            ::syn::__private::Err(cursor.error("expected `ret`"))
                        })
            }
        }
        impl ::syn::__private::ToTokens for ret {
            fn to_tokens(&self, tokens: &mut ::syn::__private::TokenStream2) {
                let ident = ::syn::Ident::new("ret", self.span);
                ::syn::__private::TokenStreamExt::append(tokens, ident);
            }
        }
        impl ::syn::__private::Copy for ret {}
        #[allow(clippy :: expl_impl_clone_on_copy)]
        impl ::syn::__private::Clone for ret {
            fn clone(&self) -> Self { *self }
        }
        impl ::syn::__private::Debug for ret {
            fn fmt(&self, f: &mut ::syn::__private::Formatter)
                -> ::syn::__private::fmt::Result {
                ::syn::__private::Formatter::write_str(f, "Keyword [ret]")
            }
        }
        impl ::syn::__private::Eq for ret {}
        impl ::syn::__private::PartialEq for ret {
            fn eq(&self, _other: &Self) -> ::syn::__private::bool { true }
        }
        impl ::syn::__private::Hash for ret {
            fn hash<__H: ::syn::__private::Hasher>(&self, _state: &mut __H) {}
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
        loop { }    }    fn gen_block<B: ToTokens>(block: &B,
        params: &Punctuated<FnArg, ::syn::token::Comma>, async_context: bool,
        mut args: InstrumentArgs, instrumented_function_name: &str,
        self_type: Option<&TypePath>) -> proc_macro2::TokenStream {
        let span_name =
            args.name.as_ref().map(|name|
                        {
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::ToTokens::to_tokens(&name, &mut _s);
                            _s
                        }).unwrap_or_else(||
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::ToTokens::to_tokens(&instrumented_function_name,
                            &mut _s);
                        _s
                    });
        let level = args.level();
        let follows_from = args.follows_from.iter();
        let follows_from =
            {
                let mut _s = ::quote::__private::TokenStream::new();
                {
                    use ::quote::__private::ext::*;
                    let has_iter =
                        ::quote::__private::ThereIsNoIteratorInRepetition;
                    #[allow(unused_mut)]
                    let (mut follows_from, i) = follows_from.quote_into_iter();
                    let has_iter = has_iter | i;
                    let _: ::quote::__private::HasIterator = has_iter;
                    while true {
                        let follows_from =
                            match follows_from.next() {
                                Some(_x) => ::quote::__private::RepInterp(_x),
                                None => break,
                            };
                        ::quote::__private::push_ident(&mut _s, "for");
                        ::quote::__private::push_ident(&mut _s, "cause");
                        ::quote::__private::push_ident(&mut _s, "in");
                        ::quote::ToTokens::to_tokens(&follows_from, &mut _s);
                        ::quote::__private::push_group(&mut _s,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::push_ident(&mut _s,
                                    "__tracing_attr_span");
                                ::quote::__private::push_dot(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "follows_from");
                                ::quote::__private::push_group(&mut _s,
                                    ::quote::__private::Delimiter::Parenthesis,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        ::quote::__private::push_ident(&mut _s, "cause");
                                        _s
                                    });
                                ::quote::__private::push_semi(&mut _s);
                                _s
                            });
                    }
                }
                _s
            };
        let span =
            (||
                        {
                            let param_names: Vec<(Ident, (Ident, RecordType))> =
                                params.clone().into_iter().flat_map(|param|
                                                match param {
                                                    FnArg::Typed(PatType { pat, ty, .. }) => {
                                                        param_names(*pat, RecordType::parse_from_ty(&*ty))
                                                    }
                                                    FnArg::Receiver(_) =>
                                                        Box::new(iter::once((Ident::new("self", param.span()),
                                                                    RecordType::Debug))),
                                                }).map(|(x, record_type)|
                                            {
                                                if self_type.is_some() && x == "_self" {
                                                        (Ident::new("self", x.span()), (x, record_type))
                                                    } else { (x.clone(), (x, record_type)) }
                                            }).collect();
                            for skip in &args.skips {
                                if !param_names.iter().map(|(user, _)|
                                                        user).any(|y| y == skip) {
                                        return {
                                                let mut _s = ::quote::__private::TokenStream::new();
                                                let _span: ::quote::__private::Span = skip.span();
                                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                    "compile_error");
                                                ::quote::__private::push_bang_spanned(&mut _s, _span);
                                                ::quote::__private::push_group_spanned(&mut _s, _span,
                                                    ::quote::__private::Delimiter::Parenthesis,
                                                    {
                                                        let mut _s = ::quote::__private::TokenStream::new();
                                                        let _span: ::quote::__private::Span = _span;
                                                        ::quote::__private::parse_spanned(&mut _s, _span,
                                                            "\"attempting to skip non-existent parameter\"");
                                                        _s
                                                    });
                                                _s
                                            };
                                    }
                            }
                            let target = args.target();
                            let parent = args.parent.iter();
                            let quoted_fields: Vec<_> =
                                param_names.iter().filter(|(param, _)|
                                                {
                                                    if args.skip_all || args.skips.contains(param) {
                                                            return false;
                                                        }
                                                    if let Some(ref fields) = args.fields {
                                                            fields.0.iter().all(|Field { ref name, .. }|
                                                                    {
                                                                        let first = name.first();
                                                                        first != name.last() ||
                                                                            !first.iter().any(|name| name == &param)
                                                                    })
                                                        } else { true }
                                                }).map(|(user_name, (real_name, record_type))|
                                            match record_type {
                                                RecordType::Value => {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    ::quote::ToTokens::to_tokens(&user_name, &mut _s);
                                                    ::quote::__private::push_eq(&mut _s);
                                                    ::quote::ToTokens::to_tokens(&real_name, &mut _s);
                                                    _s
                                                }
                                                RecordType::Debug => {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    ::quote::ToTokens::to_tokens(&user_name, &mut _s);
                                                    ::quote::__private::push_eq(&mut _s);
                                                    ::quote::__private::push_ident(&mut _s, "tracing");
                                                    ::quote::__private::push_colon2(&mut _s);
                                                    ::quote::__private::push_ident(&mut _s, "field");
                                                    ::quote::__private::push_colon2(&mut _s);
                                                    ::quote::__private::push_ident(&mut _s, "debug");
                                                    ::quote::__private::push_group(&mut _s,
                                                        ::quote::__private::Delimiter::Parenthesis,
                                                        {
                                                            let mut _s = ::quote::__private::TokenStream::new();
                                                            ::quote::__private::push_and(&mut _s);
                                                            ::quote::ToTokens::to_tokens(&real_name, &mut _s);
                                                            _s
                                                        });
                                                    _s
                                                }
                                            }).collect();
                            if let Some(Fields(ref mut fields)) = args.fields {
                                    let mut replacer =
                                        IdentAndTypesRenamer {
                                            idents: param_names.into_iter().map(|(a, (b, _))|
                                                        (a, b)).collect(),
                                            types: Vec::new(),
                                        };
                                    if let Some(self_type) = self_type {
                                            replacer.types.push(("Self", self_type.clone()));
                                        }
                                    for e in fields.iter_mut().filter_map(|f| f.value.as_mut())
                                        {
                                        syn::visit_mut::visit_expr_mut(&mut replacer, e);
                                    }
                                }
                            let custom_fields = &args.fields;
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::push_ident(&mut _s, "tracing");
                                ::quote::__private::push_colon2(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "span");
                                ::quote::__private::push_bang(&mut _s);
                                ::quote::__private::push_group(&mut _s,
                                    ::quote::__private::Delimiter::Parenthesis,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        ::quote::__private::push_ident(&mut _s, "target");
                                        ::quote::__private::push_colon(&mut _s);
                                        ::quote::ToTokens::to_tokens(&target, &mut _s);
                                        ::quote::__private::push_comma(&mut _s);
                                        {
                                            use ::quote::__private::ext::*;
                                            let has_iter =
                                                ::quote::__private::ThereIsNoIteratorInRepetition;
                                            #[allow(unused_mut)]
                                            let (mut parent, i) = parent.quote_into_iter();
                                            let has_iter = has_iter | i;
                                            let _: ::quote::__private::HasIterator = has_iter;
                                            while true {
                                                let parent =
                                                    match parent.next() {
                                                        Some(_x) => ::quote::__private::RepInterp(_x),
                                                        None => break,
                                                    };
                                                ::quote::__private::push_ident(&mut _s, "parent");
                                                ::quote::__private::push_colon(&mut _s);
                                                ::quote::ToTokens::to_tokens(&parent, &mut _s);
                                                ::quote::__private::push_comma(&mut _s);
                                            }
                                        }
                                        ::quote::ToTokens::to_tokens(&level, &mut _s);
                                        ::quote::__private::push_comma(&mut _s);
                                        ::quote::ToTokens::to_tokens(&span_name, &mut _s);
                                        ::quote::__private::push_comma(&mut _s);
                                        {
                                            use ::quote::__private::ext::*;
                                            let has_iter =
                                                ::quote::__private::ThereIsNoIteratorInRepetition;
                                            #[allow(unused_mut)]
                                            let (mut quoted_fields, i) =
                                                quoted_fields.quote_into_iter();
                                            let has_iter = has_iter | i;
                                            let _: ::quote::__private::HasIterator = has_iter;
                                            while true {
                                                let quoted_fields =
                                                    match quoted_fields.next() {
                                                        Some(_x) => ::quote::__private::RepInterp(_x),
                                                        None => break,
                                                    };
                                                ::quote::ToTokens::to_tokens(&quoted_fields, &mut _s);
                                                ::quote::__private::push_comma(&mut _s);
                                            }
                                        }
                                        ::quote::ToTokens::to_tokens(&custom_fields, &mut _s);
                                        _s
                                    });
                                _s
                            }
                        })();
        let target = args.target();
        let err_event =
            match args.err_mode {
                Some(FormatMode::Default) | Some(FormatMode::Display) => {
                    Some({
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "tracing");
                            ::quote::__private::push_colon2(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "error");
                            ::quote::__private::push_bang(&mut _s);
                            ::quote::__private::push_group(&mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "target");
                                    ::quote::__private::push_colon(&mut _s);
                                    ::quote::ToTokens::to_tokens(&target, &mut _s);
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "error");
                                    ::quote::__private::push_eq(&mut _s);
                                    ::quote::__private::push_rem(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "e");
                                    _s
                                });
                            _s
                        })
                }
                Some(FormatMode::Debug) =>
                    Some({
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "tracing");
                            ::quote::__private::push_colon2(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "error");
                            ::quote::__private::push_bang(&mut _s);
                            ::quote::__private::push_group(&mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "target");
                                    ::quote::__private::push_colon(&mut _s);
                                    ::quote::ToTokens::to_tokens(&target, &mut _s);
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "error");
                                    ::quote::__private::push_eq(&mut _s);
                                    ::quote::__private::push_question(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "e");
                                    _s
                                });
                            _s
                        }),
                _ => None,
            };
        let ret_event =
            match args.ret_mode {
                Some(FormatMode::Display) =>
                    Some({
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "tracing");
                            ::quote::__private::push_colon2(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "event");
                            ::quote::__private::push_bang(&mut _s);
                            ::quote::__private::push_group(&mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "target");
                                    ::quote::__private::push_colon(&mut _s);
                                    ::quote::ToTokens::to_tokens(&target, &mut _s);
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::ToTokens::to_tokens(&level, &mut _s);
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "return");
                                    ::quote::__private::push_eq(&mut _s);
                                    ::quote::__private::push_rem(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "x");
                                    _s
                                });
                            _s
                        }),
                Some(FormatMode::Default) | Some(FormatMode::Debug) =>
                    Some({
                            let mut _s = ::quote::__private::TokenStream::new();
                            ::quote::__private::push_ident(&mut _s, "tracing");
                            ::quote::__private::push_colon2(&mut _s);
                            ::quote::__private::push_ident(&mut _s, "event");
                            ::quote::__private::push_bang(&mut _s);
                            ::quote::__private::push_group(&mut _s,
                                ::quote::__private::Delimiter::Parenthesis,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    ::quote::__private::push_ident(&mut _s, "target");
                                    ::quote::__private::push_colon(&mut _s);
                                    ::quote::ToTokens::to_tokens(&target, &mut _s);
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::ToTokens::to_tokens(&level, &mut _s);
                                    ::quote::__private::push_comma(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "return");
                                    ::quote::__private::push_eq(&mut _s);
                                    ::quote::__private::push_question(&mut _s);
                                    ::quote::__private::push_ident(&mut _s, "x");
                                    _s
                                });
                            _s
                        }),
                _ => None,
            };
        if async_context {
                let mk_fut =
                    match (err_event, ret_event) {
                        (Some(err_event), Some(ret_event)) => {
                            let mut _s = ::quote::__private::TokenStream::new();
                            let _span: ::quote::__private::Span = block.span();
                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                "async");
                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                "move");
                            ::quote::__private::push_group_spanned(&mut _s, _span,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    let _span: ::quote::__private::Span = _span;
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "match");
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "async");
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "move");
                                    ::quote::ToTokens::to_tokens(&block, &mut _s);
                                    ::quote::__private::push_dot_spanned(&mut _s, _span);
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "await");
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
                                                        "allow");
                                                    ::quote::__private::push_group_spanned(&mut _s, _span,
                                                        ::quote::__private::Delimiter::Parenthesis,
                                                        {
                                                            let mut _s = ::quote::__private::TokenStream::new();
                                                            let _span: ::quote::__private::Span = _span;
                                                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                                "clippy");
                                                            ::quote::__private::push_colon2_spanned(&mut _s, _span);
                                                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                                "unit_arg");
                                                            _s
                                                        });
                                                    _s
                                                });
                                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                "Ok");
                                            ::quote::__private::push_group_spanned(&mut _s, _span,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    let _span: ::quote::__private::Span = _span;
                                                    ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                                                    _s
                                                });
                                            ::quote::__private::push_fat_arrow_spanned(&mut _s, _span);
                                            ::quote::__private::push_group_spanned(&mut _s, _span,
                                                ::quote::__private::Delimiter::Brace,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    let _span: ::quote::__private::Span = _span;
                                                    ::quote::ToTokens::to_tokens(&ret_event, &mut _s);
                                                    ::quote::__private::push_semi_spanned(&mut _s, _span);
                                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                        "Ok");
                                                    ::quote::__private::push_group_spanned(&mut _s, _span,
                                                        ::quote::__private::Delimiter::Parenthesis,
                                                        {
                                                            let mut _s = ::quote::__private::TokenStream::new();
                                                            let _span: ::quote::__private::Span = _span;
                                                            ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                                                            _s
                                                        });
                                                    _s
                                                });
                                            ::quote::__private::push_comma_spanned(&mut _s, _span);
                                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                "Err");
                                            ::quote::__private::push_group_spanned(&mut _s, _span,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    let _span: ::quote::__private::Span = _span;
                                                    ::quote::__private::push_ident_spanned(&mut _s, _span, "e");
                                                    _s
                                                });
                                            ::quote::__private::push_fat_arrow_spanned(&mut _s, _span);
                                            ::quote::__private::push_group_spanned(&mut _s, _span,
                                                ::quote::__private::Delimiter::Brace,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    let _span: ::quote::__private::Span = _span;
                                                    ::quote::ToTokens::to_tokens(&err_event, &mut _s);
                                                    ::quote::__private::push_semi_spanned(&mut _s, _span);
                                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                        "Err");
                                                    ::quote::__private::push_group_spanned(&mut _s, _span,
                                                        ::quote::__private::Delimiter::Parenthesis,
                                                        {
                                                            let mut _s = ::quote::__private::TokenStream::new();
                                                            let _span: ::quote::__private::Span = _span;
                                                            ::quote::__private::push_ident_spanned(&mut _s, _span, "e");
                                                            _s
                                                        });
                                                    _s
                                                });
                                            _s
                                        });
                                    _s
                                });
                            _s
                        }
                        (Some(err_event), None) => {
                            let mut _s = ::quote::__private::TokenStream::new();
                            let _span: ::quote::__private::Span = block.span();
                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                "async");
                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                "move");
                            ::quote::__private::push_group_spanned(&mut _s, _span,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    let _span: ::quote::__private::Span = _span;
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "match");
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "async");
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "move");
                                    ::quote::ToTokens::to_tokens(&block, &mut _s);
                                    ::quote::__private::push_dot_spanned(&mut _s, _span);
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "await");
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
                                                        "allow");
                                                    ::quote::__private::push_group_spanned(&mut _s, _span,
                                                        ::quote::__private::Delimiter::Parenthesis,
                                                        {
                                                            let mut _s = ::quote::__private::TokenStream::new();
                                                            let _span: ::quote::__private::Span = _span;
                                                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                                "clippy");
                                                            ::quote::__private::push_colon2_spanned(&mut _s, _span);
                                                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                                "unit_arg");
                                                            _s
                                                        });
                                                    _s
                                                });
                                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                "Ok");
                                            ::quote::__private::push_group_spanned(&mut _s, _span,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    let _span: ::quote::__private::Span = _span;
                                                    ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                                                    _s
                                                });
                                            ::quote::__private::push_fat_arrow_spanned(&mut _s, _span);
                                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                "Ok");
                                            ::quote::__private::push_group_spanned(&mut _s, _span,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    let _span: ::quote::__private::Span = _span;
                                                    ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                                                    _s
                                                });
                                            ::quote::__private::push_comma_spanned(&mut _s, _span);
                                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                "Err");
                                            ::quote::__private::push_group_spanned(&mut _s, _span,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    let _span: ::quote::__private::Span = _span;
                                                    ::quote::__private::push_ident_spanned(&mut _s, _span, "e");
                                                    _s
                                                });
                                            ::quote::__private::push_fat_arrow_spanned(&mut _s, _span);
                                            ::quote::__private::push_group_spanned(&mut _s, _span,
                                                ::quote::__private::Delimiter::Brace,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    let _span: ::quote::__private::Span = _span;
                                                    ::quote::ToTokens::to_tokens(&err_event, &mut _s);
                                                    ::quote::__private::push_semi_spanned(&mut _s, _span);
                                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                                        "Err");
                                                    ::quote::__private::push_group_spanned(&mut _s, _span,
                                                        ::quote::__private::Delimiter::Parenthesis,
                                                        {
                                                            let mut _s = ::quote::__private::TokenStream::new();
                                                            let _span: ::quote::__private::Span = _span;
                                                            ::quote::__private::push_ident_spanned(&mut _s, _span, "e");
                                                            _s
                                                        });
                                                    _s
                                                });
                                            _s
                                        });
                                    _s
                                });
                            _s
                        }
                        (None, Some(ret_event)) => {
                            let mut _s = ::quote::__private::TokenStream::new();
                            let _span: ::quote::__private::Span = block.span();
                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                "async");
                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                "move");
                            ::quote::__private::push_group_spanned(&mut _s, _span,
                                ::quote::__private::Delimiter::Brace,
                                {
                                    let mut _s = ::quote::__private::TokenStream::new();
                                    let _span: ::quote::__private::Span = _span;
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "let");
                                    ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                                    ::quote::__private::push_eq_spanned(&mut _s, _span);
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "async");
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "move");
                                    ::quote::ToTokens::to_tokens(&block, &mut _s);
                                    ::quote::__private::push_dot_spanned(&mut _s, _span);
                                    ::quote::__private::push_ident_spanned(&mut _s, _span,
                                        "await");
                                    ::quote::__private::push_semi_spanned(&mut _s, _span);
                                    ::quote::ToTokens::to_tokens(&ret_event, &mut _s);
                                    ::quote::__private::push_semi_spanned(&mut _s, _span);
                                    ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                                    _s
                                });
                            _s
                        }
                        (None, None) => {
                            let mut _s = ::quote::__private::TokenStream::new();
                            let _span: ::quote::__private::Span = block.span();
                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                "async");
                            ::quote::__private::push_ident_spanned(&mut _s, _span,
                                "move");
                            ::quote::ToTokens::to_tokens(&block, &mut _s);
                            _s
                        }
                    };
                return {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s, "let");
                        ::quote::__private::push_ident(&mut _s,
                            "__tracing_attr_span");
                        ::quote::__private::push_eq(&mut _s);
                        ::quote::ToTokens::to_tokens(&span, &mut _s);
                        ::quote::__private::push_semi(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "let");
                        ::quote::__private::push_ident(&mut _s,
                            "__tracing_instrument_future");
                        ::quote::__private::push_eq(&mut _s);
                        ::quote::ToTokens::to_tokens(&mk_fut, &mut _s);
                        ::quote::__private::push_semi(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "if");
                        ::quote::__private::push_bang(&mut _s);
                        ::quote::__private::push_ident(&mut _s,
                            "__tracing_attr_span");
                        ::quote::__private::push_dot(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "is_disabled");
                        ::quote::__private::push_group(&mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            ::quote::__private::TokenStream::new());
                        ::quote::__private::push_group(&mut _s,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::ToTokens::to_tokens(&follows_from, &mut _s);
                                ::quote::__private::push_ident(&mut _s, "tracing");
                                ::quote::__private::push_colon2(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "Instrument");
                                ::quote::__private::push_colon2(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "instrument");
                                ::quote::__private::push_group(&mut _s,
                                    ::quote::__private::Delimiter::Parenthesis,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        ::quote::__private::push_ident(&mut _s,
                                            "__tracing_instrument_future");
                                        ::quote::__private::push_comma(&mut _s);
                                        ::quote::__private::push_ident(&mut _s,
                                            "__tracing_attr_span");
                                        _s
                                    });
                                ::quote::__private::push_dot(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "await");
                                _s
                            });
                        ::quote::__private::push_ident(&mut _s, "else");
                        ::quote::__private::push_group(&mut _s,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                ::quote::__private::push_ident(&mut _s,
                                    "__tracing_instrument_future");
                                ::quote::__private::push_dot(&mut _s);
                                ::quote::__private::push_ident(&mut _s, "await");
                                _s
                            });
                        _s
                    };
            }
        let span =
            {
                let mut _s = ::quote::__private::TokenStream::new();
                ::quote::__private::push_ident(&mut _s, "let");
                ::quote::__private::push_ident(&mut _s,
                    "__tracing_attr_span");
                ::quote::__private::push_semi(&mut _s);
                ::quote::__private::push_ident(&mut _s, "let");
                ::quote::__private::push_ident(&mut _s,
                    "__tracing_attr_guard");
                ::quote::__private::push_semi(&mut _s);
                ::quote::__private::push_ident(&mut _s, "if");
                ::quote::__private::push_ident(&mut _s, "tracing");
                ::quote::__private::push_colon2(&mut _s);
                ::quote::__private::push_ident(&mut _s, "level_enabled");
                ::quote::__private::push_bang(&mut _s);
                ::quote::__private::push_group(&mut _s,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::ToTokens::to_tokens(&level, &mut _s);
                        _s
                    });
                ::quote::__private::push_group(&mut _s,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        ::quote::__private::push_ident(&mut _s,
                            "__tracing_attr_span");
                        ::quote::__private::push_eq(&mut _s);
                        ::quote::ToTokens::to_tokens(&span, &mut _s);
                        ::quote::__private::push_semi(&mut _s);
                        ::quote::ToTokens::to_tokens(&follows_from, &mut _s);
                        ::quote::__private::push_ident(&mut _s,
                            "__tracing_attr_guard");
                        ::quote::__private::push_eq(&mut _s);
                        ::quote::__private::push_ident(&mut _s,
                            "__tracing_attr_span");
                        ::quote::__private::push_dot(&mut _s);
                        ::quote::__private::push_ident(&mut _s, "enter");
                        ::quote::__private::push_group(&mut _s,
                            ::quote::__private::Delimiter::Parenthesis,
                            ::quote::__private::TokenStream::new());
                        ::quote::__private::push_semi(&mut _s);
                        _s
                    });
                _s
            };
        match (err_event, ret_event) {
            (Some(err_event), Some(ret_event)) => {
                let mut _s = ::quote::__private::TokenStream::new();
                let _span: ::quote::__private::Span = block.span();
                ::quote::ToTokens::to_tokens(&span, &mut _s);
                ::quote::__private::push_pound_spanned(&mut _s, _span);
                ::quote::__private::push_group_spanned(&mut _s, _span,
                    ::quote::__private::Delimiter::Bracket,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        let _span: ::quote::__private::Span = _span;
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "allow");
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                    "clippy");
                                ::quote::__private::push_colon2_spanned(&mut _s, _span);
                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                    "redundant_closure_call");
                                _s
                            });
                        _s
                    });
                ::quote::__private::push_ident_spanned(&mut _s, _span,
                    "match");
                ::quote::__private::push_group_spanned(&mut _s, _span,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        let _span: ::quote::__private::Span = _span;
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "move");
                        ::quote::__private::push_or_or_spanned(&mut _s, _span);
                        ::quote::ToTokens::to_tokens(&block, &mut _s);
                        _s
                    });
                ::quote::__private::push_group_spanned(&mut _s, _span,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let _: ::quote::__private::Span = _span;
                        ::quote::__private::TokenStream::new()
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
                                    "allow");
                                ::quote::__private::push_group_spanned(&mut _s, _span,
                                    ::quote::__private::Delimiter::Parenthesis,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        let _span: ::quote::__private::Span = _span;
                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                            "clippy");
                                        ::quote::__private::push_colon2_spanned(&mut _s, _span);
                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                            "unit_arg");
                                        _s
                                    });
                                _s
                            });
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "Ok");
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                                _s
                            });
                        ::quote::__private::push_fat_arrow_spanned(&mut _s, _span);
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::ToTokens::to_tokens(&ret_event, &mut _s);
                                ::quote::__private::push_semi_spanned(&mut _s, _span);
                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                    "Ok");
                                ::quote::__private::push_group_spanned(&mut _s, _span,
                                    ::quote::__private::Delimiter::Parenthesis,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        let _span: ::quote::__private::Span = _span;
                                        ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                                        _s
                                    });
                                _s
                            });
                        ::quote::__private::push_comma_spanned(&mut _s, _span);
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "Err");
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::__private::push_ident_spanned(&mut _s, _span, "e");
                                _s
                            });
                        ::quote::__private::push_fat_arrow_spanned(&mut _s, _span);
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::ToTokens::to_tokens(&err_event, &mut _s);
                                ::quote::__private::push_semi_spanned(&mut _s, _span);
                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                    "Err");
                                ::quote::__private::push_group_spanned(&mut _s, _span,
                                    ::quote::__private::Delimiter::Parenthesis,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        let _span: ::quote::__private::Span = _span;
                                        ::quote::__private::push_ident_spanned(&mut _s, _span, "e");
                                        _s
                                    });
                                _s
                            });
                        _s
                    });
                _s
            }
            (Some(err_event), None) => {
                let mut _s = ::quote::__private::TokenStream::new();
                let _span: ::quote::__private::Span = block.span();
                ::quote::ToTokens::to_tokens(&span, &mut _s);
                ::quote::__private::push_pound_spanned(&mut _s, _span);
                ::quote::__private::push_group_spanned(&mut _s, _span,
                    ::quote::__private::Delimiter::Bracket,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        let _span: ::quote::__private::Span = _span;
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "allow");
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                    "clippy");
                                ::quote::__private::push_colon2_spanned(&mut _s, _span);
                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                    "redundant_closure_call");
                                _s
                            });
                        _s
                    });
                ::quote::__private::push_ident_spanned(&mut _s, _span,
                    "match");
                ::quote::__private::push_group_spanned(&mut _s, _span,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        let _span: ::quote::__private::Span = _span;
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "move");
                        ::quote::__private::push_or_or_spanned(&mut _s, _span);
                        ::quote::ToTokens::to_tokens(&block, &mut _s);
                        _s
                    });
                ::quote::__private::push_group_spanned(&mut _s, _span,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let _: ::quote::__private::Span = _span;
                        ::quote::__private::TokenStream::new()
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
                                    "allow");
                                ::quote::__private::push_group_spanned(&mut _s, _span,
                                    ::quote::__private::Delimiter::Parenthesis,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        let _span: ::quote::__private::Span = _span;
                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                            "clippy");
                                        ::quote::__private::push_colon2_spanned(&mut _s, _span);
                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                            "unit_arg");
                                        _s
                                    });
                                _s
                            });
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "Ok");
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                                _s
                            });
                        ::quote::__private::push_fat_arrow_spanned(&mut _s, _span);
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "Ok");
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                                _s
                            });
                        ::quote::__private::push_comma_spanned(&mut _s, _span);
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "Err");
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::__private::push_ident_spanned(&mut _s, _span, "e");
                                _s
                            });
                        ::quote::__private::push_fat_arrow_spanned(&mut _s, _span);
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::ToTokens::to_tokens(&err_event, &mut _s);
                                ::quote::__private::push_semi_spanned(&mut _s, _span);
                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                    "Err");
                                ::quote::__private::push_group_spanned(&mut _s, _span,
                                    ::quote::__private::Delimiter::Parenthesis,
                                    {
                                        let mut _s = ::quote::__private::TokenStream::new();
                                        let _span: ::quote::__private::Span = _span;
                                        ::quote::__private::push_ident_spanned(&mut _s, _span, "e");
                                        _s
                                    });
                                _s
                            });
                        _s
                    });
                _s
            }
            (None, Some(ret_event)) => {
                let mut _s = ::quote::__private::TokenStream::new();
                let _span: ::quote::__private::Span = block.span();
                ::quote::ToTokens::to_tokens(&span, &mut _s);
                ::quote::__private::push_pound_spanned(&mut _s, _span);
                ::quote::__private::push_group_spanned(&mut _s, _span,
                    ::quote::__private::Delimiter::Bracket,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        let _span: ::quote::__private::Span = _span;
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "allow");
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                    "clippy");
                                ::quote::__private::push_colon2_spanned(&mut _s, _span);
                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                    "redundant_closure_call");
                                _s
                            });
                        _s
                    });
                ::quote::__private::push_ident_spanned(&mut _s, _span, "let");
                ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                ::quote::__private::push_eq_spanned(&mut _s, _span);
                ::quote::__private::push_group_spanned(&mut _s, _span,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        let _span: ::quote::__private::Span = _span;
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "move");
                        ::quote::__private::push_or_or_spanned(&mut _s, _span);
                        ::quote::ToTokens::to_tokens(&block, &mut _s);
                        _s
                    });
                ::quote::__private::push_group_spanned(&mut _s, _span,
                    ::quote::__private::Delimiter::Parenthesis,
                    {
                        let _: ::quote::__private::Span = _span;
                        ::quote::__private::TokenStream::new()
                    });
                ::quote::__private::push_semi_spanned(&mut _s, _span);
                ::quote::ToTokens::to_tokens(&ret_event, &mut _s);
                ::quote::__private::push_semi_spanned(&mut _s, _span);
                ::quote::__private::push_ident_spanned(&mut _s, _span, "x");
                _s
            }
            (None, None) => {
                let mut _s = ::quote::__private::TokenStream::new();
                let _span: ::quote::__private::Span = block.span();
                ::quote::__private::push_pound_spanned(&mut _s, _span);
                ::quote::__private::push_group_spanned(&mut _s, _span,
                    ::quote::__private::Delimiter::Bracket,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        let _span: ::quote::__private::Span = _span;
                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                            "allow");
                        ::quote::__private::push_group_spanned(&mut _s, _span,
                            ::quote::__private::Delimiter::Parenthesis,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                let _span: ::quote::__private::Span = _span;
                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                    "clippy");
                                ::quote::__private::push_colon2_spanned(&mut _s, _span);
                                ::quote::__private::push_ident_spanned(&mut _s, _span,
                                    "suspicious_else_formatting");
                                _s
                            });
                        _s
                    });
                ::quote::__private::push_group_spanned(&mut _s, _span,
                    ::quote::__private::Delimiter::Brace,
                    {
                        let mut _s = ::quote::__private::TokenStream::new();
                        let _span: ::quote::__private::Span = _span;
                        ::quote::ToTokens::to_tokens(&span, &mut _s);
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
                                            "clippy");
                                        ::quote::__private::push_colon2_spanned(&mut _s, _span);
                                        ::quote::__private::push_ident_spanned(&mut _s, _span,
                                            "suspicious_else_formatting");
                                        _s
                                    });
                                _s
                            });
                        ::quote::ToTokens::to_tokens(&block, &mut _s);
                        _s
                    });
                _s
            }
        }
    }    enum RecordType {
        Value,
        Debug,
    }
    impl RecordType {        const TYPES_FOR_VALUE: &'static [&'static str] =
            &["bool", "str", "u8", "i8", "u16", "i16", "u32", "i32", "u64",
                        "i64", "f32", "f64", "usize", "isize", "NonZeroU8",
                        "NonZeroI8", "NonZeroU16", "NonZeroI16", "NonZeroU32",
                        "NonZeroI32", "NonZeroU64", "NonZeroI64", "NonZeroUsize",
                        "NonZeroIsize", "Wrapping"];
        fn parse_from_ty(ty: &Type) -> Self {
            match ty {
                Type::Path(TypePath { path, .. }) if
                    path.segments.iter().last().map(|path_segment|
                                {
                                    let ident = path_segment.ident.to_string();
                                    Self::TYPES_FOR_VALUE.iter().any(|&t| t == ident)
                                }).unwrap_or(false) => {
                    RecordType::Value
                }
                Type::Reference(syn::TypeReference { elem, .. }) =>
                    RecordType::parse_from_ty(elem),
                _ => RecordType::Debug,
            }
        }
    }
    fn param_names(pat: Pat, record_type: RecordType)
        -> Box<dyn Iterator<Item = (Ident, RecordType)>> {
        match pat {
            Pat::Ident(PatIdent { ident, .. }) =>
                Box::new(iter::once((ident, record_type))),
            Pat::Reference(PatReference { pat, .. }) =>
                param_names(*pat, record_type),
            Pat::Struct(PatStruct { fields, .. }) =>
                Box::new(fields.into_iter().flat_map(|FieldPat { pat, .. }|
                            param_names(*pat, RecordType::Debug))),
            Pat::Tuple(PatTuple { elems, .. }) =>
                Box::new(elems.into_iter().flat_map(|p|
                            param_names(p, RecordType::Debug))),
            Pat::TupleStruct(PatTupleStruct { pat: PatTuple { elems, .. }, ..
                }) =>
                Box::new(elems.into_iter().flat_map(|p|
                            param_names(p, RecordType::Debug))),
            _ => Box::new(iter::empty()),
        }
    }    enum AsyncKind<'a> {        Function(&'a ItemFn),
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
    impl<'block> AsyncInfo<'block> {        pub(crate) fn from_fn(input: &'block ItemFn) -> Option<Self> {
            if input.sig.asyncness.is_some() { return None; }
            let block = &input.block;
            let inside_funs =
                block.stmts.iter().filter_map(|stmt|
                        {
                            if let Stmt::Item(Item::Fn(fun)) = &stmt {
                                    if fun.sig.asyncness.is_some() { return Some((stmt, fun)); }
                                }
                            None
                        });
            let (last_expr_stmt, last_expr) =
                block.stmts.iter().rev().find_map(|stmt|
                            {
                                if let Stmt::Expr(expr) = stmt {
                                        Some((stmt, expr))
                                    } else { None }
                            })?;
            if let Expr::Async(async_expr) = last_expr {
                    return Some(AsyncInfo {
                                source_stmt: last_expr_stmt,
                                kind: AsyncKind::Async { async_expr, pinned_box: false },
                                self_type: None,
                                input,
                            });
                }
            let (outside_func, outside_args) =
                match last_expr {
                    Expr::Call(ExprCall { func, args, .. }) => (func, args),
                    _ => return None,
                };
            let path =
                match outside_func.as_ref() {
                    Expr::Path(path) => &path.path,
                    _ => return None,
                };
            if !path_to_string(path).ends_with("Box::pin") { return None; }
            if outside_args.is_empty() { return None; }
            if let Expr::Async(async_expr) = &outside_args[0] {
                    return Some(AsyncInfo {
                                source_stmt: last_expr_stmt,
                                kind: AsyncKind::Async { async_expr, pinned_box: true },
                                self_type: None,
                                input,
                            });
                }
            let func =
                match &outside_args[0] {
                    Expr::Call(ExprCall { func, .. }) => func,
                    _ => return None,
                };
            let func_name =
                match **func {
                    Expr::Path(ref func_path) =>
                        path_to_string(&func_path.path),
                    _ => return None,
                };
            let (stmt_func_declaration, func) =
                inside_funs.into_iter().find(|(_, fun)|
                            fun.sig.ident == func_name)?;
            let mut self_type = None;
            for arg in &func.sig.inputs {
                if let FnArg::Typed(ty) = arg {
                        if let Pat::Ident(PatIdent { ref ident, .. }) = *ty.pat {
                                if ident == "_self" {
                                        let mut ty = *ty.ty.clone();
                                        if let Type::Reference(syn::TypeReference { elem, .. }) = ty
                                                {
                                                ty = *elem;
                                            }
                                        if let Type::Path(tp) = ty { self_type = Some(tp); break; }
                                    }
                            }
                    }
            }
            Some(AsyncInfo {
                    source_stmt: stmt_func_declaration,
                    kind: AsyncKind::Function(func),
                    self_type,
                    input,
                })
        }
        pub(crate) fn gen_async(self, args: InstrumentArgs,
            instrumented_function_name: &str)
            -> Result<proc_macro::TokenStream, syn::Error> {
            let mut out_stmts: Vec<TokenStream> =
                self.input.block.stmts.iter().map(|stmt|
                            stmt.to_token_stream()).collect();
            if let Some((iter, _stmt)) =
                        self.input.block.stmts.iter().enumerate().find(|(_iter,
                                    stmt)| *stmt == self.source_stmt) {
                    out_stmts[iter] =
                        match self.kind {
                            AsyncKind::Function(fun) => {
                                let fun = MaybeItemFn::from(fun.clone());
                                gen_function(fun.as_ref(), args, instrumented_function_name,
                                    self.self_type.as_ref())
                            }
                            AsyncKind::Async { async_expr, pinned_box } => {
                                let instrumented_block =
                                    gen_block(&async_expr.block, &self.input.sig.inputs, true,
                                        args, instrumented_function_name, None);
                                let async_attrs = &async_expr.attrs;
                                if pinned_box {
                                        {
                                            let mut _s = ::quote::__private::TokenStream::new();
                                            ::quote::__private::push_ident(&mut _s, "Box");
                                            ::quote::__private::push_colon2(&mut _s);
                                            ::quote::__private::push_ident(&mut _s, "pin");
                                            ::quote::__private::push_group(&mut _s,
                                                ::quote::__private::Delimiter::Parenthesis,
                                                {
                                                    let mut _s = ::quote::__private::TokenStream::new();
                                                    {
                                                        use ::quote::__private::ext::*;
                                                        let has_iter =
                                                            ::quote::__private::ThereIsNoIteratorInRepetition;
                                                        #[allow(unused_mut)]
                                                        let (mut async_attrs, i) = async_attrs.quote_into_iter();
                                                        let has_iter = has_iter | i;
                                                        let _: ::quote::__private::HasIterator = has_iter;
                                                        while true {
                                                            let async_attrs =
                                                                match async_attrs.next() {
                                                                    Some(_x) => ::quote::__private::RepInterp(_x),
                                                                    None => break,
                                                                };
                                                            ::quote::ToTokens::to_tokens(&async_attrs, &mut _s);
                                                        }
                                                    }
                                                    ::quote::__private::push_ident(&mut _s, "async");
                                                    ::quote::__private::push_ident(&mut _s, "move");
                                                    ::quote::__private::push_group(&mut _s,
                                                        ::quote::__private::Delimiter::Brace,
                                                        {
                                                            let mut _s = ::quote::__private::TokenStream::new();
                                                            ::quote::ToTokens::to_tokens(&instrumented_block, &mut _s);
                                                            _s
                                                        });
                                                    _s
                                                });
                                            _s
                                        }
                                    } else {
                                       {
                                           let mut _s = ::quote::__private::TokenStream::new();
                                           {
                                               use ::quote::__private::ext::*;
                                               let has_iter =
                                                   ::quote::__private::ThereIsNoIteratorInRepetition;
                                               #[allow(unused_mut)]
                                               let (mut async_attrs, i) = async_attrs.quote_into_iter();
                                               let has_iter = has_iter | i;
                                               let _: ::quote::__private::HasIterator = has_iter;
                                               while true {
                                                   let async_attrs =
                                                       match async_attrs.next() {
                                                           Some(_x) => ::quote::__private::RepInterp(_x),
                                                           None => break,
                                                       };
                                                   ::quote::ToTokens::to_tokens(&async_attrs, &mut _s);
                                               }
                                           }
                                           ::quote::__private::push_ident(&mut _s, "async");
                                           ::quote::__private::push_ident(&mut _s, "move");
                                           ::quote::__private::push_group(&mut _s,
                                               ::quote::__private::Delimiter::Brace,
                                               {
                                                   let mut _s = ::quote::__private::TokenStream::new();
                                                   ::quote::ToTokens::to_tokens(&instrumented_block, &mut _s);
                                                   _s
                                               });
                                           _s
                                       }
                                   }
                            }
                        };
                }
            let vis = &self.input.vis;
            let sig = &self.input.sig;
            let attrs = &self.input.attrs;
            Ok({
                        let mut _s = ::quote::__private::TokenStream::new();
                        {
                            use ::quote::__private::ext::*;
                            let has_iter =
                                ::quote::__private::ThereIsNoIteratorInRepetition;
                            #[allow(unused_mut)]
                            let (mut attrs, i) = attrs.quote_into_iter();
                            let has_iter = has_iter | i;
                            let _: ::quote::__private::HasIterator = has_iter;
                            while true {
                                let attrs =
                                    match attrs.next() {
                                        Some(_x) => ::quote::__private::RepInterp(_x),
                                        None => break,
                                    };
                                ::quote::ToTokens::to_tokens(&attrs, &mut _s);
                            }
                        }
                        ::quote::ToTokens::to_tokens(&vis, &mut _s);
                        ::quote::ToTokens::to_tokens(&sig, &mut _s);
                        ::quote::__private::push_group(&mut _s,
                            ::quote::__private::Delimiter::Brace,
                            {
                                let mut _s = ::quote::__private::TokenStream::new();
                                {
                                    use ::quote::__private::ext::*;
                                    let has_iter =
                                        ::quote::__private::ThereIsNoIteratorInRepetition;
                                    #[allow(unused_mut)]
                                    let (mut out_stmts, i) = out_stmts.quote_into_iter();
                                    let has_iter = has_iter | i;
                                    let _: ::quote::__private::HasIterator = has_iter;
                                    while true {
                                        let out_stmts =
                                            match out_stmts.next() {
                                                Some(_x) => ::quote::__private::RepInterp(_x),
                                                None => break,
                                            };
                                        ::quote::ToTokens::to_tokens(&out_stmts, &mut _s);
                                    }
                                }
                                _s
                            });
                        _s
                    }.into())
        }
    }
    fn path_to_string(path: &Path) -> String {
        use std::fmt::Write;
        let mut res = String::with_capacity(path.segments.len() * 5);
        for i in 0..path.segments.len() {
            (&mut res).write_fmt(format_args!("{0}",
                        path.segments[i].ident)).expect("writing to a String should never fail");
            if i < path.segments.len() - 1 { res.push_str("::"); }
        }
        res
    }
    struct IdentAndTypesRenamer<'a> {
        types: Vec<(&'a str, TypePath)>,
        idents: Vec<(Ident, Ident)>,
    }
    impl<'a> VisitMut for IdentAndTypesRenamer<'a> {
        #[allow(clippy :: cmp_owned)]
        fn visit_ident_mut(&mut self, id: &mut Ident) {
            for (old_ident, new_ident) in &self.idents {
                if id.to_string() == old_ident.to_string() {
                        *id = new_ident.clone();
                    }
            }
        }
        fn visit_type_mut(&mut self, ty: &mut Type) {
            for (type_name, new_type) in &self.types {
                if let Type::Path(TypePath { path, .. }) = ty {
                        if path_to_string(path) == *type_name {
                                *ty = Type::Path(new_type.clone());
                            }
                    }
            }
        }
    }
    struct AsyncTraitBlockReplacer<'a> {
        block: &'a Block,
        patched_block: Block,
    }
    impl<'a> VisitMut for AsyncTraitBlockReplacer<'a> {
        fn visit_block_mut(&mut self, i: &mut Block) {
            if i == self.block { *i = self.patched_block.clone(); }
        }
    }
    struct ImplTraitEraser;
    impl VisitMut for ImplTraitEraser {
        fn visit_type_mut(&mut self, t: &mut Type) {
            if let Type::ImplTrait(..) = t {
                    *t =
                        syn::TypeInfer {
                                underscore_token: ::syn::token::Underscore(t.span()),
                            }.into();
                } else { syn::visit_mut::visit_type_mut(self, t); }
        }
    }
    fn erase_impl_trait(ty: &Type) -> Type {
        let mut ty = ty.clone();
        ImplTraitEraser.visit_type_mut(&mut ty);
        ty
    }
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
}fn instrument_speculative(args: attr::InstrumentArgs,
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
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(f, "MaybeItemFn",
            "outer_attrs", &self.outer_attrs, "inner_attrs",
            &self.inner_attrs, "vis", &self.vis, "sig", &self.sig, "block",
            &&self.block)
    }
}
#[automatically_derived]
impl ::core::clone::Clone for MaybeItemFn {
    #[inline]
    fn clone(&self) -> MaybeItemFn {
        MaybeItemFn {
            outer_attrs: ::core::clone::Clone::clone(&self.outer_attrs),
            inner_attrs: ::core::clone::Clone::clone(&self.inner_attrs),
            vis: ::core::clone::Clone::clone(&self.vis),
            sig: ::core::clone::Clone::clone(&self.sig),
            block: ::core::clone::Clone::clone(&self.block),
        }
    }
}
impl MaybeItemFn {
    fn as_ref(&self) -> MaybeItemFnRef<'_, TokenStream> {
        MaybeItemFnRef {
            outer_attrs: &self.outer_attrs,
            inner_attrs: &self.inner_attrs,
            vis: &self.vis,
            sig: &self.sig,
            block: &self.block,
        }
    }
}
impl Parse for MaybeItemFn {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let outer_attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;
        let inner_attrs = input.call(Attribute::parse_inner)?;
        let block: TokenStream = input.parse()?;
        Ok(Self { outer_attrs, inner_attrs, vis, sig, block })
    }
}
impl From<ItemFn> for MaybeItemFn {
    fn from(ItemFn { attrs, vis, sig, block }: ItemFn) -> Self {
        let (outer_attrs, inner_attrs) =
            attrs.into_iter().partition(|attr|
                    attr.style == syn::AttrStyle::Outer);
        Self {
            outer_attrs,
            inner_attrs,
            vis,
            sig,
            block: block.to_token_stream(),
        }
    }
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
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field5_finish(f,
            "MaybeItemFnRef", "outer_attrs", &self.outer_attrs, "inner_attrs",
            &self.inner_attrs, "vis", &self.vis, "sig", &self.sig, "block",
            &&self.block)
    }
}
#[automatically_derived]
impl<'a, B: ::core::clone::Clone + ToTokens> ::core::clone::Clone for
    MaybeItemFnRef<'a, B> {
    #[inline]
    fn clone(&self) -> MaybeItemFnRef<'a, B> {
        MaybeItemFnRef {
            outer_attrs: ::core::clone::Clone::clone(&self.outer_attrs),
            inner_attrs: ::core::clone::Clone::clone(&self.inner_attrs),
            vis: ::core::clone::Clone::clone(&self.vis),
            sig: ::core::clone::Clone::clone(&self.sig),
            block: ::core::clone::Clone::clone(&self.block),
        }
    }
}
const _: () =
    {
        extern crate proc_macro;
        #[rustc_proc_macro_decls]
        #[used]
        #[allow(deprecated)]
        static _DECLS: &[proc_macro::bridge::client::ProcMacro] =
            &[proc_macro::bridge::client::ProcMacro::attr("instrument",
                            instrument)];
    };

