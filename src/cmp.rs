use core::cmp::{Ordering, Reverse};

use crate::IntoOwned;

fn reverse_into_owned<T: IntoOwned>(reverse: Reverse<T>) -> Reverse<T::Owned> {
    Reverse(reverse.0.into_owned())
}

impl<T: IntoOwned> IntoOwned for Reverse<T> {
    type Owned = Reverse<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        reverse_into_owned(self)
    }
}

impl_identity!(Ordering);
