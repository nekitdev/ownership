#[cfg(not(feature = "std"))]
compile_error!("expected either `std` to be enabled");

use std::ffi::OsString;

use crate::impl_identity;

impl_identity!(OsString);
