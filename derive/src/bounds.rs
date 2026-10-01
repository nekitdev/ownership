use std::iter::once;

use syn::{
    AngleBracketedGenericArguments, ConstParam, GenericArgument, GenericParam, Generics, Ident,
    Lifetime, LifetimeParam, PredicateType, TraitBound, TraitBoundModifiers, Type, TypeParam,
    TypeParamBound, TypePath, WherePredicate,
    fold::Fold,
    parse_quote,
    punctuated::Punctuated,
    token::{Colon, Gt, Lt, Plus},
};

use crate::{
    By,
    ast::Container,
    defaults,
    expand::into_owned_trait,
    find::{Filter, Relevant, find},
    lifetimes::{self, Promoted},
    map::Mapper,
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
        .chain(propagate(promoted.by_ref(), relevant.by_ref(), generics))
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

pub fn predicate_type_with(path: TypePath, bounds: Bounds) -> PredicateType {
    PredicateType {
        attrs: Vec::new(),
        lifetimes: None,
        bounded_ty: Type::Path(path),
        colon_token: Colon::default(),
        bounds,
    }
}

pub fn predicate_type(path: TypePath) -> PredicateType {
    let bound = TypeParamBound::Trait(into_owned_bound());

    predicate_type_with(path, one(bound))
}

pub fn where_predicate(path: TypePath) -> WherePredicate {
    WherePredicate::Type(predicate_type(path))
}

pub fn one<T, C: FromIterator<T>>(value: T) -> C {
    once(value).collect()
}

pub type Bounds = Punctuated<TypeParamBound, Plus>;

pub fn map_bounds<'b, B: IntoIterator<Item = &'b TypeParamBound>>(
    mapper: &mut Mapper<'_>,
    bounds: B,
) -> Bounds {
    bounds
        .into_iter()
        .filter(|bound| is_predicate_bound(bound))
        .cloned()
        .map(|bound| mapper.fold_type_param_bound(bound))
        .collect()
}

pub fn is_predicate_bound(bound: &TypeParamBound) -> bool {
    match bound {
        // `?Sized` is only permitted where the parameter is declared
        TypeParamBound::Trait(trait_bound) => trait_bound.maybe.is_none(),
        // accept lifetimes
        TypeParamBound::Lifetime(_) => true,
        // `use<...>` and uninterpreted bounds can not be predicates
        _ => false,
    }
}

pub fn propagate_type(mut mapper: Mapper<'_>, parameter: &TypeParam) -> Option<PredicateType> {
    let bounds = map_bounds(mapper.by_mut(), parameter.bounds.iter());

    if !mapper.is_mappable() || bounds.is_empty() {
        return None; // either no bounds or can not map certainly
    }

    let path = as_into_owned(parameter.ident.clone());

    let predicate = predicate_type_with(path, bounds);

    Some(predicate)
}

pub fn propagate_where_predicate(
    mut mapper: Mapper<'_>,
    predicate: &WherePredicate,
) -> Option<PredicateType> {
    let WherePredicate::Type(predicate_type) = predicate else {
        return None; // only type predicates are relevant
    };

    let bounded = mapper.fold_type(predicate_type.bounded_ty.clone());
    let bounds = map_bounds(mapper.by_mut(), predicate_type.bounds.iter());

    if !mapper.is_mappable() || !mapper.has_mapped() || bounds.is_empty() {
        return None; // no bounds, nothing was mapped, or can not map certainly
    }

    let mapped = PredicateType {
        attrs: Vec::new(),
        lifetimes: predicate_type.lifetimes.clone(),
        bounded_ty: bounded,
        colon_token: predicate_type.colon_token,
        bounds,
    };

    Some(mapped)
}

pub fn where_type(predicate_type: PredicateType) -> WherePredicate {
    WherePredicate::Type(predicate_type)
}

pub fn propagate(
    promoted: &Promoted<'_>,
    relevant: &Relevant<'_>,
    generics: &Generics,
) -> Vec<WherePredicate> {
    let mapper = Mapper::new(promoted, relevant);

    let type_iter = generics
        .type_params()
        .filter(|type_parameter| mapper.is_relevant(type_parameter.ident.by_ref()))
        .flat_map(|type_parameter| propagate_type(mapper.reset_clone(), type_parameter));

    if let Some(ref where_clause) = generics.where_clause {
        let where_iter = where_clause
            .predicates
            .iter()
            .filter_map(|predicate| propagate_where_predicate(mapper.reset_clone(), predicate));

        type_iter.chain(where_iter).map(where_type).collect()
    } else {
        type_iter.map(where_type).collect()
    }
}
