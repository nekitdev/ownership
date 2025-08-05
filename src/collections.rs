use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
    } else if #[cfg(feature = "alloc")] {
        use alloc::collections::{BTreeMap, BTreeSet, BinaryHeap, LinkedList, VecDeque};
    } else {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

use crate::{IntoOwned, iterable::recollect};

impl<T: IntoOwned> IntoOwned for VecDeque<T> {
    type Owned = VecDeque<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        recollect(self)
    }
}

impl<T: IntoOwned> IntoOwned for LinkedList<T> {
    type Owned = LinkedList<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        recollect(self)
    }
}

impl<T: IntoOwned> IntoOwned for BinaryHeap<T>
where
    T::Owned: Ord,
{
    type Owned = BinaryHeap<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        recollect(self)
    }
}

impl<K: IntoOwned, V: IntoOwned> IntoOwned for BTreeMap<K, V>
where
    K::Owned: Ord,
{
    type Owned = BTreeMap<K::Owned, V::Owned>;

    fn into_owned(self) -> Self::Owned {
        recollect(self)
    }
}

impl<T: IntoOwned> IntoOwned for BTreeSet<T>
where
    T::Owned: Ord,
{
    type Owned = BTreeSet<T::Owned>;

    fn into_owned(self) -> Self::Owned {
        recollect(self)
    }
}
