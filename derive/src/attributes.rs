use std::marker::PhantomData;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute as AttributeInput, Error, Meta, Path, meta::ParseNestedMeta};

use crate::{
    context::Context,
    name::{ConstStr, Name},
};

pub struct Attribute<'c, T> {
    context: &'c Context,
    name: Name,
    tokens: TokenStream,
    value: Option<T>,
}

impl<'c, T> Attribute<'c, T> {
    fn none(context: &'c Context, name: Name) -> Self {
        Self {
            context,
            name,
            tokens: TokenStream::new(),
            value: None,
        }
    }

    fn set<I: ToTokens>(&mut self, tokens: I, value: T) {
        if self.value.is_some() {
            let message = format!(
                "duplicate `{ownership}` attribute `{name}`",
                ownership = Name::OWNERSHIP,
                name = self.name
            );

            self.context.error_spanned_by(tokens, message);
        } else {
            self.tokens = tokens.into_token_stream();

            self.value = Some(value);
        }
    }

    fn get(self) -> Option<T> {
        self.value
    }
}

struct BoolAttribute<'c> {
    attribute: Attribute<'c, ()>,
}

impl<'c> BoolAttribute<'c> {
    const fn new(attribute: Attribute<'c, ()>) -> Self {
        Self { attribute }
    }

    fn none(context: &'c Context, name: Name) -> Self {
        Self::new(Attribute::none(context, name))
    }

    fn set<I: ToTokens>(&mut self, item: I) {
        self.attribute.set(item, ());
    }

    fn get(self) -> bool {
        self.attribute.get().is_some()
    }
}

const SPACE: char = ' ';
const EMPTY: &str = "";

mod sealed {
    pub trait Sealed {}
}

pub trait Kind: sealed::Sealed {
    const NAME: ConstStr;

    fn unknown_meta(meta: &ParseNestedMeta<'_>, path: &Path) -> Error {
        let name = path.to_token_stream().to_string().replace(SPACE, EMPTY);

        let message = format!(
            "unknown `{ownership}` {kind} attribute `{name}`",
            ownership = Name::OWNERSHIP,
            kind = Self::NAME
        );

        meta.error(message)
    }
}

pub struct Container {
    private: PhantomData<()>,
}

pub struct Field {
    private: PhantomData<()>,
}

pub struct Variant {
    private: PhantomData<()>,
}

impl sealed::Sealed for Container {}
impl sealed::Sealed for Field {}
impl sealed::Sealed for Variant {}

impl Kind for Container {
    const NAME: ConstStr = "container";
}

impl Kind for Field {
    const NAME: ConstStr = "field";
}

impl Kind for Variant {
    const NAME: ConstStr = "variant";
}

pub struct Attributes<K: Kind> {
    as_is: bool,
    kind: PhantomData<K>,
}

pub type ContainerAttributes = Attributes<Container>;
pub type FieldAttributes = Attributes<Field>;
pub type VariantAttributes = Attributes<Variant>;

impl<K: Kind> Attributes<K> {
    pub fn from_ast<'a, A: IntoIterator<Item = &'a AttributeInput>>(
        context: &Context,
        attrs: A,
    ) -> Self {
        let mut as_is = BoolAttribute::none(context, Name::AS_IS);

        for attr in attrs {
            if !Name::OWNERSHIP.match_path(attr.path()) {
                continue;
            }

            if let Meta::List(ref meta) = attr.meta
                && meta.tokens.is_empty()
            {
                continue;
            }

            if let Err(error) = attr.parse_nested_meta(|meta| {
                let path = &meta.path;

                if Name::AS_IS.match_path(path) {
                    as_is.set(path);
                } else {
                    return Err(K::unknown_meta(&meta, path));
                }

                Ok(())
            }) {
                context.error(error);
            }
        }

        Self {
            as_is: as_is.get(),
            kind: PhantomData,
        }
    }

    pub const fn as_is(&self) -> bool {
        self.as_is
    }
}
