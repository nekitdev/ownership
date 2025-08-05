use crate::IntoOwned;

impl<T: IntoOwned, const N: usize> IntoOwned for [T; N] {
    type Owned = [T::Owned; N];

    fn into_owned(self) -> Self::Owned {
        self.map(IntoOwned::into_owned)
    }
}
