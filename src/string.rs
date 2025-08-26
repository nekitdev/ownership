use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "std")] {
        // everything needed is already in scope
    } else if #[cfg(feature = "alloc")] {
        use alloc::string::String;
    } else {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

use crate::impl_identity;

impl_identity!(String);
