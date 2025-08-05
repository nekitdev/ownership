macro_rules! impl_primitive {
    ($($type: ty),+ $(,)?) => {
        $(
            impl $crate::IntoOwned for $type {
                type Owned = Self;

                fn into_owned(self) -> Self::Owned {
                    self
                }
            }
        )+
    }
}

impl_primitive!(bool, char);
impl_primitive!(i8, i16, i32, i64, i128, isize);
impl_primitive!(u8, u16, u32, u64, u128, usize);
impl_primitive!(f32, f64);
