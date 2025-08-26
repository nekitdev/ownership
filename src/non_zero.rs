mod import {
    pub use core::num::NonZero;
}

macro_rules! impl_non_zero {
    ($($type: ty),+ $(,)?) => {
        $(
            $crate::impl_identity!($crate::non_zero::import::NonZero<$type>);
        )+
    };
}

impl_non_zero!(i8, i16, i32, i64, i128, isize);
impl_non_zero!(u8, u16, u32, u64, u128, usize);
impl_non_zero!(char);
