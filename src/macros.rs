//! Macros used for implementing [`IntoOwned`].
//!
//! [`IntoOwned`]: crate::IntoOwned

/// Implements [`IntoOwned`] via identity.
///
/// For example:
///
/// ```
/// use ownership::impl_identity;
///
/// struct Identity;
///
/// impl_identity!(Identity);
/// ```
///
/// expands to:
///
/// ```
/// struct Identity;
///
/// impl ownership::IntoOwned for Identity {
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
