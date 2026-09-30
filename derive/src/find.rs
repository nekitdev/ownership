use std::collections::HashSet;

use syn::{
    Field, Generics, Ident, Lifetime, Path, Type, TypePath,
    punctuated::Pair,
    visit::{Visit, visit_field, visit_lifetime, visit_path},
};
use trait_aliases::trait_aliases;

use crate::{
    By,
    ast::{Container, Data},
    attributes::{ContainerAttributes, FieldAttributes, VariantAttributes},
    exactly_one::ExactlyOne,
    names::Name,
};

trait_aliases! {
    pub trait Filter = Fn(ContainerAttributes, FieldAttributes, Option<VariantAttributes>) -> bool;
}

pub type All<'f> = HashSet<&'f Ident>;
pub type Relevant<'f> = HashSet<&'f Ident>;
pub type Associated<'f> = Vec<&'f TypePath>;
pub type Lifetimes<'f> = HashSet<&'f Lifetime>;

pub struct Finder<'f> {
    all: All<'f>,
    relevant: Relevant<'f>,
    associated: Associated<'f>,
    lifetimes: Lifetimes<'f>,
}

impl<'f> Finder<'f> {
    pub fn new(generics: &'f Generics) -> Self {
        let all = Self::collect_all(generics);

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

    pub fn collect_all(generics: &'f Generics) -> All<'f> {
        generics
            .type_params()
            .map(|type_parameter| type_parameter.ident.by_ref())
            .collect()
    }

    pub fn split(self) -> (Relevant<'f>, Associated<'f>, Lifetimes<'f>) {
        (self.relevant, self.associated, self.lifetimes)
    }
}

impl<'f> Finder<'f> {
    pub fn visit_container<F: Filter>(&mut self, container: &'f Container<'f>, filter: &F) {
        match container.data() {
            Data::Enum(enum_data) => enum_data.variants().iter().for_each(|variant| {
                variant
                    .fields()
                    .iter()
                    .filter(|field| {
                        filter(
                            container.attributes(),
                            field.attributes(),
                            Some(variant.attributes()),
                        )
                    })
                    .for_each(|relevant| self.visit_field(relevant.input()))
            }),
            Data::Struct(struct_data) => struct_data
                .fields()
                .iter()
                .filter(|field| filter(container.attributes(), field.attributes(), None))
                .for_each(|relevant| self.visit_field(relevant.input())),
        }
    }
}

impl<'f> Visit<'f> for Finder<'f> {
    fn visit_field(&mut self, field: &'f Field) {
        let mut ungrouped = field.ty.by_ref();

        while let Type::Group(grouped) = ungrouped {
            ungrouped = grouped.elem.by_ref();
        }

        #[allow(clippy::collapsible_if)]
        if let Type::Path(path_type) = ungrouped {
            if let Some(Pair::Punctuated(path_segment, _)) = path_type.path.segments.pairs().next()
            {
                if self.all.contains(path_segment.ident.by_ref()) {
                    self.associated.push(path_type);
                }
            }
        }

        visit_field(self, field);
    }

    fn visit_path(&mut self, path: &'f Path) {
        if Name::PHANTOM_DATA.is_last_in(path) {
            // NOTE: `PhantomData<T>` is `IntoOwned` regardless of `T`
            return;
        }

        #[allow(clippy::collapsible_if)]
        if path.leading_colon.is_none() {
            if let Ok(one) = path.segments.iter().exactly_one() {
                let identifier = one.ident.by_ref();

                if self.all.contains(identifier) {
                    self.relevant.insert(identifier);
                }
            }
        }

        visit_path(self, path);
    }

    fn visit_lifetime(&mut self, lifetime: &'f Lifetime) {
        self.lifetimes.insert(lifetime);

        visit_lifetime(self, lifetime);
    }
}

pub fn find<'a, F: Filter>(
    container: &'a Container<'a>,
    generics: &'a Generics,
    filter: &F,
) -> Finder<'a> {
    let mut finder = Finder::new(generics);

    finder.visit_container(container, filter);

    finder
}
