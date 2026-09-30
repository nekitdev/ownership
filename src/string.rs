cfg_select! {
    feature = "std" => {
        // no imports needed
    }
    feature = "alloc" => {
        use alloc::string::String;
    }
    _ => {
        compile_error!("expected either `std` or `alloc` to be enabled");
    }
}

impl_identity!(String);
