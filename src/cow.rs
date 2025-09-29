//! Obtaining ownership of [`Cow<'_, T>`].

#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::borrow::{Cow, ToOwned};

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
