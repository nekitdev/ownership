#[cfg(not(feature = "std"))]
compile_error!("expected either `std` to be enabled");

use std::ffi::OsString;

impl_identity!(OsString);
