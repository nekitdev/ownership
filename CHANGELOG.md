# Changelog

<!-- changelogging: start -->

## [0.3.0](https://github.com/nekitdev/ownership/tree/v0.3.0) (2025-11-17)

### Features

- Added `IntoOwned` for `time` types.

  Specifically, the trait is implemented for `Duration` unconditionally,
  and with `std` feature it is also implemented for `Instant` and `SystemTime`.

## [0.2.4](https://github.com/nekitdev/ownership/tree/v0.2.4) (2025-10-15)

No significant changes.

## [0.2.3](https://github.com/nekitdev/ownership/tree/v0.2.3) (2025-10-14)

No significant changes.

## [0.2.2](https://github.com/nekitdev/ownership/tree/v0.2.2) (2025-09-29)

No significant changes.

## [0.2.1](https://github.com/nekitdev/ownership/tree/v0.2.1) (2025-09-29)

No significant changes.

## [0.2.0](https://github.com/nekitdev/ownership/tree/v0.2.0) (2025-08-26)

### Features

- Added `derive` feature to enable `#[derive(IntoOwned)]`.

- Added more implementations of `IntoOwned` on standard types.
  See the [documentation](https://docs.rs/ownership) for more.

## [0.1.1](https://github.com/nekitdev/ownership/tree/v0.1.1) (2025-08-05)

No significant changes.

## [0.1.0](https://github.com/nekitdev/ownership/tree/v0.1.0) (2025-08-05)

Initial release.
