#[cfg(not(feature = "std"))]
compile_error!("expected `std` to be enabled");

use std::time::{Instant, SystemTime};

impl_identity!(Instant, SystemTime);
