use cfg_if::cfg_if;

use crate::{IntoOwned, impl_identity, iterable::recollect};

cfg_if! {
    if #[cfg(feature = "std")] {
        // everything needed is already in scope
    } else if #[cfg(feature = "alloc")] {
        use alloc::boxed::Box;
    } else {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

impl<T: IntoOwned> IntoOwned for Box<T> {
    type Owned = Box<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        Self::Owned::new((*self).into_owned())
    }
}

impl<T: IntoOwned> IntoOwned for Box<[T]> {
    type Owned = Box<[T::Owned]>;

    fn into_owned(self) -> Self::Owned {
        recollect(self)
    }
}

impl_identity!(Box<str>);
