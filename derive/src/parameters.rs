use syn::{AngleBracketedGenericArguments, Generics, Ident};

use crate::{
    ast::Container,
    attributes::{ContainerAttributes, FieldAttributes, VariantAttributes},
    bounds::{apply_build, remove_defaults},
};

pub struct Parameters {
    pub name: Ident,
    pub generics: Generics,
    pub generic_arguments: AngleBracketedGenericArguments,
}

impl Parameters {
    pub fn new(container: &Container<'_>) -> Self {
        let name = container.name.clone();

        let (generics, generic_arguments) = Self::build_generics(container);

        Self {
            name,
            generics,
            generic_arguments,
        }
    }

    pub fn build_generics(container: &Container<'_>) -> (Generics, AngleBracketedGenericArguments) {
        let mut generics = container.generics.clone();

        remove_defaults(&mut generics);

        let generic_arguments = apply_build(container, &mut generics, Self::not_as_is);

        (generics, generic_arguments)
    }

    pub fn not_as_is(
        container: &ContainerAttributes,
        field: &FieldAttributes,
        variant_option: Option<&VariantAttributes>,
    ) -> bool {
        !container.as_is()
            && !field.as_is()
            && variant_option.is_none_or(|variant| !variant.as_is())
    }
}
