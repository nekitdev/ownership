use syn::{
    BoundLifetimes, Ident, Lifetime, Type,
    fold::{Fold, fold_type},
};

use crate::{
    By,
    bounds::{as_into_owned, make_static},
    find::Relevant,
    lifetimes::Promoted,
    names::Name,
};

pub type PromotedRef<'m> = &'m Promoted<'m>;
pub type RelevantRef<'m> = &'m Relevant<'m>;

pub struct Mapper<'m> {
    promoted: PromotedRef<'m>,
    relevant: RelevantRef<'m>,
    mapped: bool,
    mappable: bool,
}

impl<'m> Mapper<'m> {
    pub const fn new(promoted: PromotedRef<'m>, relevant: RelevantRef<'m>) -> Self {
        Self {
            promoted,
            relevant,
            mapped: false,
            mappable: true,
        }
    }

    pub const fn promoted(&self) -> PromotedRef<'m> {
        self.promoted
    }

    pub const fn relevant(&self) -> RelevantRef<'m> {
        self.relevant
    }

    pub const fn has_mapped(&self) -> bool {
        self.mapped
    }

    pub const fn is_mappable(&self) -> bool {
        self.mappable
    }

    pub const fn set_mapped(&mut self) {
        self.mapped = true;
    }

    pub const fn not_mappable(&mut self) {
        self.mappable = false;
    }

    pub fn is_promoted(&self, lifetime_ref: &Lifetime) -> bool {
        self.promoted().contains(lifetime_ref)
    }

    pub fn is_relevant(&self, identifier_ref: &Ident) -> bool {
        self.relevant().contains(identifier_ref)
    }

    pub const fn reset_clone(&self) -> Self {
        Self::new(self.promoted(), self.relevant())
    }
}

pub fn is_static(lifetime: &Lifetime) -> bool {
    lifetime.ident == Name::STATIC.get()
}

pub const ONE: usize = 1;

impl Fold for Mapper<'_> {
    fn fold_lifetime(&mut self, lifetime: Lifetime) -> Lifetime {
        let mut lifetime = lifetime;

        if self.is_promoted(lifetime.by_ref()) && !is_static(lifetime.by_ref()) {
            make_static(lifetime.by_mut());

            self.set_mapped();
        }

        lifetime
    }

    fn fold_bound_lifetimes(&mut self, bound_lifetimes: BoundLifetimes) -> BoundLifetimes {
        // `for<'a>` binds the lifetime `'a`, so leave it alone (do not map to `'static`)
        bound_lifetimes
    }

    fn fold_type(&mut self, ty: Type) -> Type {
        if let Type::Path(ref type_path) = ty {
            let path = type_path.path.by_ref();

            #[allow(clippy::collapsible_if)]
            if type_path.qself.is_none() && path.leading_colon.is_none() {
                if let Some(first) = path.segments.first() {
                    let name = first.ident.by_ref();

                    if self.is_relevant(name) {
                        if path.segments.len() == ONE && first.arguments.is_none() {
                            self.set_mapped();

                            return Type::Path(as_into_owned(name.clone()));
                        }

                        self.not_mappable();
                    }
                }
            }
        }

        fold_type(self, ty)
    }
}
