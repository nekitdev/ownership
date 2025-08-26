use proc_macro::TokenStream;
use syn::{DeriveInput, Error, parse_macro_input};

mod ast;
mod attributes;
mod bounds;
mod context;
mod dummy;
mod expand;
mod name;
mod parameters;

#[proc_macro_derive(IntoOwned, attributes(ownership))]
pub fn derive_into_owned(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    expand::derive_into_owned(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
