use std::fmt;

use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{Ident, Path};

pub type StaticStr = &'static str;

#[derive(Clone, Copy)]
pub struct Name {
    string: StaticStr,
}

impl Name {
    pub const fn new(string: StaticStr) -> Self {
        Self { string }
    }

    pub const fn get(self) -> StaticStr {
        self.string
    }

    pub const PHANTOM_DATA: Self = Self::new("PhantomData");

    pub const STATIC: Self = Self::new("static");

    pub const DERIVE: Self = Self::new("derive");

    pub const OWNERSHIP: Self = Self::new("ownership");

    pub const AS_IS: Self = Self::new("as_is");

    pub const CONTAINER: Self = Self::new("container");
    pub const FIELD: Self = Self::new("field");
    pub const VARIANT: Self = Self::new("variant");

    pub fn matches_path(self, path: &Path) -> bool {
        path.is_ident(self.get())
    }

    pub fn is_last_in(self, path: &Path) -> bool {
        path.segments
            .last()
            .is_some_and(|last| last.ident == self.get())
    }

    pub fn ident(self) -> Ident {
        Ident::new(self.get(), Span::call_site())
    }
}

impl fmt::Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl ToTokens for Name {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(self.ident());
    }
}
