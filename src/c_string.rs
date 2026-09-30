cfg_select! {
    feature = "std" => {
        use std::ffi::CString;
    }
    feature = "alloc" => {
        use alloc::ffi::CString;
    }
    _ => {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

impl_identity!(CString);
