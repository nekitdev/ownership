#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

use crate::{IntoOwned, impl_identity, iterable::recollect};

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::boxed::Box;

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
