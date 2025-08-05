//! Recollecting iterables.

use crate::IntoOwned;

/// Recollects the given iterable into the collection of the given type.
///
/// Since implementing [`IntoOwned`] for collections can get repetitive, namely:
///
/// ```ignore
/// iterable.into_iter().map(IntoOwned::into_owned).collect()
/// ```
///
/// is used in many places, this function is provided to reduce boilerplate.
///
/// The implementation is equivalent to the snippet above.
pub fn recollect<T: IntoOwned, I: IntoIterator<Item = T>, C: FromIterator<T::Owned>>(
    iterable: I,
) -> C {
    iterable.into_iter().map(IntoOwned::into_owned).collect()
}
