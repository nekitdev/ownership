use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        use std::ffi::CString;
    } else if #[cfg(feature = "alloc")] {
        use alloc::ffi::CString;
    } else {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

use crate::impl_identity;

impl_identity!(CString);
