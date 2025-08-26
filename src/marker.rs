use core::marker::PhantomData;

use crate::IntoOwned;

impl<T: ?Sized> IntoOwned for PhantomData<T> {
    type Owned = Self;

    fn into_owned(self) -> Self::Owned {
        self
    }
}
