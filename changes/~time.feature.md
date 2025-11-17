Added `IntoOwned` for `time` types.

Specifically, the trait is implemented for `Duration` unconditionally,
and with `std` feature it is also implemented for `Instant` and `SystemTime`.
