use proc_macro2::TokenStream;
use quote::quote;

use crate::name::Name;

pub fn wrap_in_const(code: &TokenStream) -> TokenStream {
    let ownership = Name::OWNERSHIP;

    quote! {
        #[doc(hidden)]
        const _: () = {
            use #ownership as _ownership;

            #code
        };
    }
}
