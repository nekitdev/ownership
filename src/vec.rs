use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        // everything needed is already in scope
    } else if #[cfg(feature = "alloc")] {
        use alloc::vec::Vec;
    } else {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

use crate::{IntoOwned, iterable::recollect};

impl<T: IntoOwned> IntoOwned for Vec<T> {
    type Owned = Vec<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        recollect(self)
    }
}
