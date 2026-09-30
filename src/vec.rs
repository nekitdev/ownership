cfg_select! {
    feature = "std" => {
        // no imports needed
    }
    feature = "alloc" => {
        use alloc::vec::Vec;
    }
    _ => {
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
