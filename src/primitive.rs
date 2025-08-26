use crate::impl_identity;

impl_identity!(bool, char);
impl_identity!(i8, i16, i32, i64, i128, isize);
impl_identity!(u8, u16, u32, u64, u128, usize);
impl_identity!(f32, f64);
