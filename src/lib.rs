//! Obtaining ownership.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(feature = "alloc")]
extern crate alloc;

use cfg_if::cfg_if;

/// Obtaining ownership.
pub trait IntoOwned {
    /// The owned type produced by [`into_owned`].
    ///
    /// This type enforces *idempotence* through its bound.
    ///
    /// [`into_owned`]: Self::into_owned
    type Owned: IntoOwned<Owned = Self::Owned>;

    /// Consumes [`Self`] and converts it into the associated [`Owned`] type.
    ///
    /// [`Owned`]: Self::Owned
    fn into_owned(self) -> Self::Owned;
}

#[cfg(feature = "derive")]
pub use ownership_derive::IntoOwned;

pub mod iterable;

#[macro_use]
pub mod macros;

mod array;
mod cmp;
mod marker;
mod net;
mod never;
mod non_zero;
mod primitive;
mod simple;
mod tuple;
mod unit;

cfg_if! {
    if #[cfg(any(feature = "std", feature = "alloc"))] {
        pub mod cow;

        mod boxed;
        mod c_string;
        mod collections;
        mod string;
        mod vec;
    }
}

cfg_if! {
    if #[cfg(feature = "std")] {
        mod hash;
        mod os_string;
    }
}
