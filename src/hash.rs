#[cfg(not(feature = "std"))]
compile_error!("expected `std` to be enabled");

use core::hash::Hash;

use std::{
    collections::{HashMap, HashSet},
    hash::BuildHasher,
};

use crate::{IntoOwned, iterable::recollect};

impl<T: IntoOwned, S: BuildHasher + Default> IntoOwned for HashSet<T, S>
where
    T::Owned: Eq + Hash,
{
    type Owned = HashSet<T::Owned, S>;

    fn into_owned(self) -> Self::Owned {
        recollect(self)
    }
}

impl<K: IntoOwned, V: IntoOwned, S: BuildHasher + Default> IntoOwned for HashMap<K, V, S>
where
    K::Owned: Eq + Hash,
{
    type Owned = HashMap<K::Owned, V::Owned, S>;

    fn into_owned(self) -> Self::Owned {
        recollect(self)
    }
}
