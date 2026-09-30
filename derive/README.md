# `ownership-derive`

> _Derive macros for obtaining ownership._

This crate is mostly an implementation detail of the [`ownership`][ownership] crate.

Instead of using this crate directly via:

```console
$ cargo add ownership-derive
```

It is recommended to use the `derive` feature of [`ownership`][ownership] instead:

```console
$ cargo add ownership --features derive
```

Or like so:

```toml
[dependencies.ownership]
version = "0.4.0"
features = ["derive"]
```

[ownership]: https://crates.io/crates/ownership
