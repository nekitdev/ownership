use std::{collections::HashSet, iter::once};

use syn::{
    AngleBracketedGenericArguments, Field, GenericArgument, GenericParam, Generics, Ident,
    Lifetime, Path, PredicateType, TraitBound, TraitBoundModifier, Type, TypeParamBound, TypePath,
    WherePredicate, parse_quote,
    punctuated::{Pair, Punctuated},
    token::{Colon, Gt, Lt},
    visit::{Visit, visit_field, visit_lifetime, visit_path},
};

use crate::{
    ast::{Container, Data},
    attributes::{ContainerAttributes, FieldAttributes, VariantAttributes},
    expand::into_owned_trait,
    name::Name,
};

pub fn remove_default(parameter: &mut GenericParam) {
    match parameter {
        GenericParam::Type(type_parameter) => {
            type_parameter.eq_token = None;
            type_parameter.default = None;
        }
        GenericParam::Const(const_parameter) => {
            const_parameter.eq_token = None;
            const_parameter.default = None;
        }
        GenericParam::Lifetime(_) => {
            // lifetime parameters do not have defaults
        }
    }
}

pub fn remove_defaults(generics: &mut Generics) {
    generics.params.iter_mut().for_each(remove_default);
}

pub fn build_generic_arguments<L: Fn(&Ident) -> bool, T: Fn(&Ident) -> bool>(
    lifetime_predicate: L,
    type_predicate: T,
    generics: &Generics,
) -> AngleBracketedGenericArguments {
    let mut generic_arguments = AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Lt::default(),
        args: Punctuated::new(),
        gt_token: Gt::default(),
    };

    let into_owned = into_owned_trait();

    generic_arguments
        .args
        .extend(generics.params.iter().map(|parameter| match parameter {
            GenericParam::Lifetime(lifetime_parameter) => {
                let mut lifetime = lifetime_parameter.lifetime.clone();

                if lifetime_predicate(&lifetime.ident) {
                    lifetime.ident = Name::STATIC.identifier();
                }

                GenericArgument::Lifetime(lifetime)
            }
            GenericParam::Type(type_parameter) => {
                let identifier = type_parameter.ident.clone();

                let generated: Type = if type_predicate(&identifier) {
                    parse_quote! {
                        <#identifier as #into_owned>::Owned
                    }
                } else {
                    parse_quote! {
                        #identifier
                    }
                };

                GenericArgument::Type(generated)
            }
            GenericParam::Const(const_parameter) => {
                let identifier = const_parameter.ident.clone();

                let generated: Type = parse_quote! {
                    #identifier
                };

                GenericArgument::Type(generated)
            }
        }));

    generic_arguments
}

const ONE: usize = 1;

pub fn apply_build<
    F: Fn(&ContainerAttributes, &FieldAttributes, Option<&VariantAttributes>) -> bool,
>(
    container: &Container<'_>,
    generics: &mut Generics,
    filter: F,
) -> AngleBracketedGenericArguments {
    struct FindParameters<'a> {
        all: HashSet<Ident>,
        relevant: HashSet<Ident>,
        associated: Vec<&'a TypePath>,
        lifetimes: HashSet<Ident>,
    }

    impl FindParameters<'_> {
        fn new(generics: &Generics) -> Self {
            let all = generics
                .type_params()
                .map(|type_parameter| type_parameter.ident.clone())
                .collect();

            let relevant = HashSet::new();
            let associated = Vec::new();
            let lifetimes = HashSet::new();

            Self {
                all,
                relevant,
                associated,
                lifetimes,
            }
        }
    }

    impl<'a> Visit<'a> for FindParameters<'a> {
        fn visit_field(&mut self, field: &'a Field) {
            let mut ungrouped = &field.ty;

            while let Type::Group(grouped) = ungrouped {
                ungrouped = &grouped.elem;
            }

            #[allow(clippy::collapsible_if)]
            if let Type::Path(path_type) = ungrouped {
                if let Some(Pair::Punctuated(path_segment, _)) =
                    path_type.path.segments.pairs().next()
                {
                    if self.all.contains(&path_segment.ident) {
                        self.associated.push(path_type);
                    }
                }
            }

            visit_field(self, field);
        }

        fn visit_path(&mut self, path: &'a Path) {
            if Name::PHANTOM_DATA.is_last_in(path) {
                // NOTE: `PhantomData<T>` is `IntoOwned` regardless of `T`
                return;
            }

            if path.leading_colon.is_none() && path.segments.len() == ONE {
                let identifier = &path.segments.first().unwrap().ident;

                if self.all.contains(identifier) {
                    self.relevant.insert(identifier.clone());
                }
            }

            visit_path(self, path);
        }

        fn visit_lifetime(&mut self, lifetime: &'a Lifetime) {
            self.lifetimes.insert(lifetime.ident.clone());

            visit_lifetime(self, lifetime);
        }
    }

    let mut find_parameters = FindParameters::new(generics);

    match container.data {
        Data::Enum(ref variants) => variants.iter().for_each(|variant| {
            variant
                .fields
                .iter()
                .filter(|field| {
                    filter(
                        &container.attributes,
                        &field.attributes,
                        Some(&variant.attributes),
                    )
                })
                .for_each(|relevant| find_parameters.visit_field(relevant.input));
        }),
        Data::Struct(_, ref fields) => fields
            .iter()
            .filter(|field| filter(&container.attributes, &field.attributes, None))
            .for_each(|field| find_parameters.visit_field(field.input)),
    }

    let relevant = find_parameters.relevant;
    let associated = find_parameters.associated;

    let mut lifetimes = find_parameters.lifetimes;

    generics.lifetimes().for_each(|lifetime_parameter| {
        if lifetime_parameter
            .bounds
            .iter()
            .any(|lifetime| lifetimes.contains(&lifetime.ident))
        {
            lifetimes.insert(lifetime_parameter.lifetime.ident.clone());
        }
    });

    let predicates: Vec<_> = generics
        .type_params()
        .map(|type_parameter| type_parameter.ident.clone())
        .filter(|identifier| relevant.contains(identifier))
        .map(|identifier| TypePath {
            qself: None,
            path: identifier.into(),
        })
        .chain(associated.into_iter().cloned())
        .map(|bounded| {
            WherePredicate::Type(PredicateType {
                lifetimes: None,
                bounded_ty: Type::Path(bounded),
                colon_token: Colon::default(),
                bounds: once(TypeParamBound::Trait(TraitBound {
                    paren_token: None,
                    modifier: TraitBoundModifier::None,
                    lifetimes: None,
                    path: into_owned_trait(),
                }))
                .collect(),
            })
        })
        .collect();

    generics.make_where_clause().predicates.extend(predicates);

    build_generic_arguments(
        |lifetime| lifetimes.contains(lifetime),
        |identifier| relevant.contains(identifier),
        generics,
    )
}
