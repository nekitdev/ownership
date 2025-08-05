use crate::IntoOwned;

impl IntoOwned for () {
    type Owned = Self;

    fn into_owned(self) -> Self::Owned {
        self
    }
}
