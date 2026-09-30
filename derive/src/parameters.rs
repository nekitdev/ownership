use syn::{AngleBracketedGenericArguments, Generics, Ident};

use crate::{ast::Container, attributes::not_as_is, bounds::apply_and_build};

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
        let mut generics = container.generics().clone();

        let generic_arguments = apply_and_build(container, &mut generics, &not_as_is);

        (generics, generic_arguments)
    }

    pub const fn name(&self) -> &Ident {
        &self.name
    }

    pub const fn generics(&self) -> &Generics {
        &self.generics
    }

    pub const fn generic_arguments(&self) -> &AngleBracketedGenericArguments {
        &self.generic_arguments
    }
}
