//! # `err-handler` is a non-intrusive error handling marco that enhances your error handling
//! If you need to handle errors externally or consolidate them in a single location, `err-handler` is your logical choice
//! ```toml
//! [dependencies]
//! err-handler = "0.1.1"
//! ```
//! # Examples
//! ```rust
//! use thiserror::Error;
//! use err_handler::err_handler;
//! #[derive(Debug, Error)]
//! enum Err {
//!     #[error("err1")]
//!     Err1,
//!     #[error("err2")]
//!     Err2,
//! }
//! // The ` err-handler` marco attribute can be any function name but must have a matching return type.
//! #[err_handler(task_handler)]
//! fn task(_v: i32) -> Result<i32, Err> {
//!     Err(Err::Err1)
//! }
//! fn task_handler(e: Err) -> Result<i32, Err> {
//!     match e {
//!         Err::Err1 => Ok(100),
//!         _ => Err(e)
//!     }
//! }
//! // If a target function is asynchronous, then its error handling function must also be asynchronous.
//! #[err_handler(crate::async_task_handler)]
//! async fn async_task(_v: i32) -> Result<i32, Err> {
//!     Err(Err::Err1)
//! }
//! async fn async_task_handler(e: Err) -> Result<i32, Err> {
//!     match e {
//!         Err::Err1 => Ok(100),
//!         _ => Err(e)
//!     }
//! }
//! #[tokio::main]
//! async fn main() -> Result<(), Err> {
//!     assert_eq!(task(0)?, 100);
//!     assert_eq!(async_task(0).await?, 100);
//!     Ok(())
//! }
//!
//! ```
//!

use proc_macro2::Punct;
use quote::{format_ident, quote, TokenStreamExt};
use std::ops::Deref;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, Token};

type TokenStream1 = proc_macro::TokenStream;
type TokenStream2 = proc_macro2::TokenStream;

#[proc_macro_attribute]
pub fn err_handler(attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    ast_gen(attr, item)
}

fn ast_gen(attr: TokenStream1, input: TokenStream1) -> TokenStream1 {
    let mut input = parse_macro_input!(input as ItemFn);

    let vis = input.vis.clone();
    let sig = input.sig.clone();

    input.sig.ident = format_ident!("_{}", input.sig.ident);

    input.sig.inputs = input
        .sig
        .inputs
        .into_iter()
        .filter(|v| {
            if let FnArg::Receiver(_) = v {
                return false;
            }
            true
        })
        .collect::<Punctuated<FnArg, Token![,]>>();

    let punct = Punct::new(',', proc_macro2::Spacing::Alone);

    let mut parameter = TokenStream2::default();
    for arg in &input.sig.inputs {
        if let FnArg::Typed(v) = arg {
            if let Pat::Ident(ident) = v.pat.deref() {
                if !parameter.is_empty() {
                    parameter.append(proc_macro2::TokenTree::from(punct.clone()));
                }
                parameter.append(ident.ident.clone());
            }
        }
    }

    let new_ident = &input.sig.ident;
    let handle = TokenStream2::from(attr);

    let match_call = if sig.asyncness.is_some() {
        quote! {
            match #new_ident(#parameter).await {
               Ok(v) => Ok(v),
               Err(e) => #handle(e).await
           }
        }
    } else {
        quote! {
            match #new_ident(#parameter) {
               Ok(v) => Ok(v),
               Err(e) => #handle(e)
           }
        }
    };

    let ast = quote! {
        #vis #sig {
            #input
            #match_call
        }
    };
    ast.into()
}
