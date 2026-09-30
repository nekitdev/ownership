use proc_macro::TokenStream;
use syn::{DeriveInput, Error, parse_macro_input};

mod ast;
mod attributes;
mod bounds;
mod by;
mod context;
mod defaults;
mod dummy;
mod exactly_one;
mod expand;
mod find;
mod index;
mod lifetimes;
mod names;
mod parameters;

use crate::by::By;

#[proc_macro_derive(IntoOwned, attributes(ownership))]
pub fn derive_into_owned(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    expand::derive_into_owned(input.by_ref())
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
