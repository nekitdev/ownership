use crate::IntoOwned;

impl<T: IntoOwned> IntoOwned for Option<T> {
    type Owned = Option<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        self.map(IntoOwned::into_owned)
    }
}

impl<T: IntoOwned, E: IntoOwned> IntoOwned for Result<T, E> {
    type Owned = Result<T::Owned, E::Owned>;

    fn into_owned(self) -> Self::Owned {
        self.map(IntoOwned::into_owned)
            .map_err(IntoOwned::into_owned)
    }
}
