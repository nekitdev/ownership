use std::fmt;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{Ident, Path};

pub type ConstStr = &'static str;

#[derive(Clone, Copy)]
pub struct Name {
    string: ConstStr,
}

impl Name {
    pub const fn new(string: ConstStr) -> Self {
        Self { string }
    }

    pub const fn get(self) -> ConstStr {
        self.string
    }

    pub const PHANTOM_DATA: Self = Self::new(PHANTOM_DATA);
    pub const DERIVE: Self = Self::new(DERIVE);
    pub const STATIC: Self = Self::new(STATIC);
    pub const OWNERSHIP: Self = Self::new(OWNERSHIP);
    pub const AS_IS: Self = Self::new(AS_IS);

    pub fn match_path(self, path: &Path) -> bool {
        path.is_ident(self.get())
    }

    pub fn is_last_in(self, path: &Path) -> bool {
        let Some(last) = path.segments.last() else {
            return false;
        };

        last.ident == self.get()
    }

    pub fn identifier(self) -> Ident {
        Ident::new(self.get(), Span::call_site())
    }
}

pub const PHANTOM_DATA: ConstStr = "PhantomData";
pub const DERIVE: ConstStr = "derive";
pub const STATIC: ConstStr = "static";
pub const OWNERSHIP: ConstStr = "ownership";
pub const AS_IS: ConstStr = "as_is";

impl fmt::Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl ToTokens for Name {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.identifier());
    }
}
