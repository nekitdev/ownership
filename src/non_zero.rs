mod import {
    pub use core::num::NonZero;
}

macro_rules! impl_non_zero {
    ($($type: ty),+ $(,)?) => {
        $(
            impl $crate::IntoOwned for $crate::non_zero::import::NonZero<$type> {
                type Owned = Self;

                fn into_owned(self) -> Self::Owned {
                    self
                }
            }
        )+
    };
}

impl_non_zero!(i8, i16, i32, i64, i128, isize);
impl_non_zero!(u8, u16, u32, u64, u128, usize);
