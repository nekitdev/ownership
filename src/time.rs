#[cfg(not(feature = "std"))]
compile_error!("expected `std` to be enabled");

use std::time::{Instant, SystemTime};

use crate::impl_identity;

impl_identity!(Instant, SystemTime);
