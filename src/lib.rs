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
    /// [`into_owned`]: Self::into_owned
    type Owned;

    /// Consumes [`Self`] and converts it into the associated owned type.
    fn into_owned(self) -> Self::Owned;
}

pub mod iterable;

mod array;
mod non_zero;
mod primitive;
mod simple;
mod tuple;
mod unit;

cfg_if! {
    if #[cfg(any(feature = "std", feature = "alloc"))] {
        pub mod cow;

        mod boxed;
        mod collections;
        mod vec;
    }
}

#[cfg(feature = "std")]
mod hash;
