use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::names::Name;

pub fn wrap_in_const<T: ToTokens>(tokens: &T) -> TokenStream {
    let ownership = Name::OWNERSHIP;

    quote! {
        #[doc(hidden)]
        const _: () = {
            use #ownership as _ownership;

            #tokens
        };
    }
}
