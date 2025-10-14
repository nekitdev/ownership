use std::fmt;

use non_empty_str::{NonEmptyStr, const_non_empty_str};
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, TokenStreamExt};
use syn::{Ident, Path};

pub type ConstNonEmptyStr = &'static NonEmptyStr;

#[derive(Clone, Copy)]
pub struct Name {
    string: ConstNonEmptyStr,
}

impl Name {
    pub const fn new(string: ConstNonEmptyStr) -> Self {
        Self { string }
    }

    pub const fn get(self) -> ConstNonEmptyStr {
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
        Ident::new(self.get().as_str(), Span::call_site())
    }
}

pub const PHANTOM_DATA: ConstNonEmptyStr = const_non_empty_str!("PhantomData");
pub const DERIVE: ConstNonEmptyStr = const_non_empty_str!("derive");
pub const STATIC: ConstNonEmptyStr = const_non_empty_str!("static");
pub const OWNERSHIP: ConstNonEmptyStr = const_non_empty_str!("ownership");
pub const AS_IS: ConstNonEmptyStr = const_non_empty_str!("as_is");

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
