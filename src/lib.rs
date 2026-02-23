#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{Error, Fields, ItemEnum, parse_macro_input, parse_quote};

/// # Simple state enum attribute macro.
///
/// The first variant is default.
///
/// ## Requirements
///
/// - must have at least one variant.
/// - must only contain unit variants.
///
/// ## Derives
///
/// - [`Copy`]
/// - [`Clone`]
/// - [`Default`]
/// - [`PartialEq`]
///
#[doc = include_str!("../README.md")]
#[proc_macro_attribute]
pub fn state_enum(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemEnum);

    let ident = &input.ident;

    match input.variants.first_mut() {
        Some(v) => v.attrs.push(parse_quote! { #[default] }),
        None => {
            return Error::new(ident.span(), "state enum must have at least one variant")
                .into_compile_error()
                .into();
        }
    }

    let variants = &input.variants;

    for v in variants.iter() {
        if !matches!(v.fields, Fields::Unit) {
            return Error::new(ident.span(), "only unit variants allowed in state enum")
                .into_compile_error()
                .into();
        }
    }

    let variants: Vec<_> = variants.iter().map(|v| &v.ident).collect();
    let len = variants.len();

    TokenStream::from(quote! {
        #[derive(Copy, Clone, Default, PartialEq)]
        #input

        impl #ident {
            const ALL: [Self; #len] = [#(Self::#variants),*];

            pub fn next(self) -> Self {
                Self::ALL[(self as usize + 1) % #len]
            }

            pub fn prev(self) -> Self {
                Self::ALL[(self as usize + #len - 1) % #len]
            }
        }
    })
}
