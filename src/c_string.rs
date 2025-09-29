#[cfg(not(any(feature = "std", feature = "alloc")))]
compile_error!("expected either `std` or `alloc` to be enabled");

#[cfg(feature = "std")]
use std::ffi::CString;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::ffi::CString;

use crate::impl_identity;

impl_identity!(CString);
