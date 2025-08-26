//! Macros used for implementing [`IntoOwned`].
//!
//! [`IntoOwned`]: crate::IntoOwned

/// Implements [`IntoOwned`] via identity.
///
/// For example:
///
/// ```ignore
/// impl_identity!(T);
/// ```
///
/// expands to:
///
/// ```ignore
/// impl IntoOwned for T {
///     type Owned = Self;
///
///     fn into_owned(self) -> Self::Owned {
///         self
///     }
/// }
/// ```
///
/// [`IntoOwned`]: crate::IntoOwned
#[macro_export]
macro_rules! impl_identity {
    ($($type: ty),+ $(,)?) => {
        $(
            impl $crate::IntoOwned for $type {
                type Owned = Self;

                fn into_owned(self) -> Self::Owned {
                    self
                }
            }
        )+
    };
}
