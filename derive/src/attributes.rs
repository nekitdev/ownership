use std::marker::PhantomData;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute as AttributeInput, Error, Meta, Path, meta::ParseNestedMeta};

use crate::{By, context::Context, names::Name};

pub struct Attribute<'c, T> {
    context: &'c Context,
    name: Name,
    stream: TokenStream,
    option: Option<T>,
}

impl<'c, T> Attribute<'c, T> {
    fn new(context: &'c Context, name: Name) -> Self {
        Self {
            context,
            name,
            stream: TokenStream::new(),
            option: None,
        }
    }

    fn error_spanned_by<S: ToTokens>(&self, tokens: S) {
        let message = format!(
            "duplicate `{ownership}` attribute `{name}`",
            ownership = Name::OWNERSHIP,
            name = self.name
        );

        self.context.error_spanned_by(tokens, message);
    }

    pub const fn has(&self) -> bool {
        self.option.is_some()
    }

    pub fn replace<S: ToTokens>(&mut self, tokens: S, value: T) -> Option<T> {
        self.stream = tokens.into_token_stream();

        self.option.replace(value)
    }

    pub fn set<S: ToTokens>(&mut self, tokens: S, value: T) -> Option<T> {
        if self.has() {
            self.error_spanned_by(tokens);

            Some(value)
        } else {
            self.replace(tokens, value)
        }
    }

    pub fn get(self) -> Option<T> {
        self.option
    }
}

impl<T: Default> Attribute<'_, T> {
    pub fn set_default<S: ToTokens>(&mut self, tokens: S) -> Option<T> {
        self.set(tokens, T::default())
    }
}

type UnitAttribute<'c> = Attribute<'c, ()>;

struct BoolAttribute<'c> {
    attribute: UnitAttribute<'c>,
}

impl<'c> BoolAttribute<'c> {
    const fn unit(attribute: UnitAttribute<'c>) -> Self {
        Self { attribute }
    }

    fn new(context: &'c Context, name: Name) -> Self {
        Self::unit(UnitAttribute::new(context, name))
    }

    fn set<T: ToTokens>(&mut self, tokens: T) {
        self.attribute.set_default(tokens);
    }

    fn get(self) -> bool {
        self.attribute.get().is_some()
    }
}

const SPACE: char = ' ';
const EMPTY: &str = "";

pub fn pretty_path(path: &Path) -> String {
    path.to_token_stream().to_string().replace(SPACE, EMPTY)
}

mod sealed {
    pub trait Sealed {}
}

pub trait Kind: sealed::Sealed {
    const NAME: Name;

    fn unknown_meta(meta: &ParseNestedMeta<'_>, path: &Path) -> Error {
        let message = format!(
            "unknown `{ownership}` {kind} attribute `{name}`",
            ownership = Name::OWNERSHIP,
            kind = Self::NAME,
            name = pretty_path(path)
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
    const NAME: Name = Name::CONTAINER;
}

impl Kind for Field {
    const NAME: Name = Name::FIELD;
}

impl Kind for Variant {
    const NAME: Name = Name::VARIANT;
}

pub struct Attributes<K: Kind> {
    as_is: bool,
    kind: PhantomData<K>,
}

impl<K: Kind> Attributes<K> {
    pub const fn new(as_is: bool) -> Self {
        Self {
            as_is,
            kind: PhantomData,
        }
    }

    pub const fn as_is(&self) -> bool {
        self.as_is
    }
}

impl<K: Kind> Clone for Attributes<K> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K: Kind> Copy for Attributes<K> {}

pub type ContainerAttributes = Attributes<Container>;
pub type FieldAttributes = Attributes<Field>;
pub type VariantAttributes = Attributes<Variant>;

impl<K: Kind> Attributes<K> {
    pub fn from_ast<'a, A: IntoIterator<Item = &'a AttributeInput>>(
        context: &Context,
        attrs: A,
    ) -> Self {
        let mut as_is = BoolAttribute::new(context, Name::AS_IS);

        for attr in attrs {
            if !Name::OWNERSHIP.matches_path(attr.path()) {
                continue;
            }

            if let Meta::List(ref meta) = attr.meta
                && meta.tokens.is_empty()
            {
                continue;
            }

            if let Err(error) = attr.parse_nested_meta(|meta| {
                let path = meta.path.by_ref();

                if Name::AS_IS.matches_path(path) {
                    as_is.set(path);
                } else {
                    return Err(K::unknown_meta(meta.by_ref(), path));
                }

                Ok(())
            }) {
                context.error(error);
            }
        }

        Self::new(as_is.get())
    }
}

pub fn as_is(
    container: ContainerAttributes,
    field: FieldAttributes,
    variant_option: Option<VariantAttributes>,
) -> bool {
    container.as_is() || field.as_is() || variant_option.is_some_and(|variant| variant.as_is())
}

pub fn not_as_is(
    container: ContainerAttributes,
    field: FieldAttributes,
    variant_option: Option<VariantAttributes>,
) -> bool {
    !as_is(container, field, variant_option)
}
