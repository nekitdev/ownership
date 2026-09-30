cfg_select! {
    feature = "std" => {
        // no imports needed
    }
    feature = "alloc" => {
        use alloc::boxed::Box;
    }
    _ => {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

use crate::{IntoOwned, iterable::recollect};

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
