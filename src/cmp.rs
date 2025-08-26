use core::cmp::{Ordering, Reverse};

use crate::IntoOwned;

impl<T: IntoOwned> IntoOwned for Reverse<T> {
    type Owned = Reverse<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        Reverse(self.0.into_owned())
    }
}

impl_identity!(Ordering);
