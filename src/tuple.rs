macro_rules! impl_tuple {
    ($($name: ident: $type: ident),+ $(,)?) => {
        impl<$($type: $crate::IntoOwned),+> $crate::IntoOwned for ($($type,)+) {
            type Owned = ($($type::Owned,)+);

            fn into_owned(self) -> Self::Owned {
                let ($($name,)+) = self;

                ($($name.into_owned(),)+)
            }
        }
    }
}

impl_tuple!(a: A);
impl_tuple!(a: A, b: B);
impl_tuple!(a: A, b: B, c: C);
impl_tuple!(a: A, b: B, c: C, d: D);
impl_tuple!(a: A, b: B, c: C, d: D, e: E);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K);
impl_tuple!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L);
