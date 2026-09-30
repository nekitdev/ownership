use std::iter::once;

use syn::{
    AngleBracketedGenericArguments, ConstParam, GenericArgument, GenericParam, Generics, Ident,
    Lifetime, LifetimeParam, PredicateType, TraitBound, TraitBoundModifiers, Type, TypeParam,
    TypeParamBound, TypePath, WherePredicate, parse_quote,
    token::{Colon, Gt, Lt},
};

use crate::{
    By,
    ast::Container,
    defaults,
    expand::into_owned_trait,
    find::{Filter, Relevant, find},
    lifetimes::{self, Promoted},
    names::Name,
};

pub fn build_generic_arguments(
    promoted: &Promoted<'_>,
    relevant: &Relevant<'_>,
    generics: &Generics,
) -> AngleBracketedGenericArguments {
    let iterator = generics.params.iter().map(|parameter| match parameter {
        GenericParam::Lifetime(lifetime_parameter) => {
            GenericArgument::Lifetime(map_lifetime(promoted, lifetime_parameter))
        }
        GenericParam::Type(type_parameter) => {
            GenericArgument::Type(map_type(relevant, type_parameter))
        }
        GenericParam::Const(const_parameter) => GenericArgument::Type(map_const(const_parameter)),
    });

    generic_arguments(iterator)
}

pub fn map_lifetime(promoted: &Promoted<'_>, parameter: &LifetimeParam) -> Lifetime {
    let mut lifetime = parameter.lifetime.clone();

    if promoted.contains(lifetime.by_ref()) {
        make_static(lifetime.by_mut());
    }

    lifetime
}

pub fn make_static(lifetime: &mut Lifetime) {
    lifetime.ident = Name::STATIC.ident();
}

pub fn map_type(relevant: &Relevant<'_>, parameter: &TypeParam) -> Type {
    let name = parameter.ident.clone();

    let path = if relevant.contains(name.by_ref()) {
        as_into_owned(name)
    } else {
        type_path(name)
    };

    Type::Path(path)
}

pub fn as_into_owned(name: Ident) -> TypePath {
    let into_owned = into_owned_trait();

    parse_quote! {
        <#name as #into_owned>::Owned
    }
}

pub fn map_const(parameter: &ConstParam) -> Type {
    let name = parameter.ident.clone();

    Type::Path(type_path(name))
}

pub fn generic_arguments<A: IntoIterator<Item = GenericArgument>>(
    arguments: A,
) -> AngleBracketedGenericArguments {
    AngleBracketedGenericArguments {
        colon2_token: None,
        lt_token: Lt::default(),
        args: arguments.into_iter().collect(),
        gt_token: Gt::default(),
    }
}

pub fn apply_and_build<F: Filter>(
    container: &Container<'_>,
    generics: &mut Generics,
    filter: &F,
) -> AngleBracketedGenericArguments {
    defaults::remove(generics);

    let finder = find(container, generics, filter);

    let (relevant, associated, start) = finder.split();

    let outlived = lifetimes::outlived(generics);
    let promoted = lifetimes::promoted(outlived.by_ref(), start);

    let predicates: Vec<_> = generics
        .type_params()
        .map(|type_parameter| type_parameter.ident.clone())
        .filter(|reference| relevant.contains(reference))
        .map(type_path)
        .chain(associated.into_iter().cloned())
        .map(where_predicate)
        .collect();

    let generic_arguments = build_generic_arguments(promoted.by_ref(), relevant.by_ref(), generics);

    generics.make_where_clause().predicates.extend(predicates);

    generic_arguments
}

pub fn type_path(name: Ident) -> TypePath {
    TypePath {
        attrs: Vec::new(),
        qself: None,
        path: name.into(),
    }
}

pub fn into_owned_bound() -> TraitBound {
    TraitBound {
        paren_token: None,
        // non-exhaustive, `Default` guarantees no modifiers
        lifetimes: None,
        modifiers: TraitBoundModifiers::default(),
        maybe: None,
        path: into_owned_trait(),
    }
}

pub fn predicate_type(path: TypePath) -> PredicateType {
    PredicateType {
        attrs: Vec::new(),
        lifetimes: None,
        bounded_ty: Type::Path(path),
        colon_token: Colon::default(),
        bounds: one(TypeParamBound::Trait(into_owned_bound())),
    }
}

pub fn where_predicate(path: TypePath) -> WherePredicate {
    WherePredicate::Type(predicate_type(path))
}

pub fn one<T, C: FromIterator<T>>(value: T) -> C {
    once(value).collect()
}
