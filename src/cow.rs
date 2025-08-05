//! Obtaining ownership of [`Cow<'_, T>`].

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::borrow::{Cow, ToOwned};
    } else if #[cfg(feature = "alloc")] {
        use alloc::borrow::{Cow, ToOwned};
    } else {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

use crate::IntoOwned;

/// Converts [`Cow<'_, T>`] into [`Cow<'static, T>`].
pub fn into_owned<T: ToOwned + ?Sized + 'static>(cow: Cow<'_, T>) -> Cow<'static, T> {
    Cow::Owned(cow.into_owned())
}

impl<T: ToOwned + ?Sized + 'static> IntoOwned for Cow<'_, T> {
    type Owned = Cow<'static, T>;

    fn into_owned(self) -> <Self as IntoOwned>::Owned {
        into_owned(self)
    }
}
