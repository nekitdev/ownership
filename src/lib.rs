//! Obtaining ownership.
//!
//! The core of the `ownership` crate is the [`IntoOwned`] trait,
//! which allows to consume values and convert them into owned types.
//!
//! If the `derive` feature is enabled, the aforementioned trait can be derived.

#![cfg_attr(
    all(feature = "std", feature = "derive"),
    doc = r#"
# Derive

```
use std::borrow::Cow;

use ownership::IntoOwned;

#[derive(IntoOwned)]
struct Config<'h, 'c, T> {
    name: Cow<'c, str>,
    value: T,
    #[ownership(as_is)]
    help: &'h str, // <- returned as-is
}
```

Generates code functionally equivalent to:

```
use std::borrow::Cow;

use ownership::IntoOwned;

struct Config<'h, 'c, T> {
    name: Cow<'c, str>,
    value: T,
    help: &'h str,
}

impl<'h, 'c, T: IntoOwned> IntoOwned for Config<'h, 'c, T> {
    type Owned = Config<'h, 'static, <T as IntoOwned>::Owned>;

    fn into_owned(self) -> <Self as IntoOwned>::Owned {
        Self::Owned {
            name: IntoOwned::into_owned(self.name),
            value: IntoOwned::into_owned(self.value),
            help: self.help, // <- here
        }
    }
}
```
    "#
)]
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
