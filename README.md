# `ownership`

[![License][License Badge]][License]
[![Version][Version Badge]][Crate]
[![Downloads][Downloads Badge]][Crate]
[![Documentation][Documentation Badge]][Documentation]
[![Test][Test Badge]][Actions]

> _Obtaining ownership._

## Installation

### `cargo`

You can add `ownership` as a dependency with the following command:

```console
$ cargo add ownership
```

Or by directly specifying it in the configuration like so:

```toml
[dependencies]
ownership = "0.4.0"
```

Alternatively, you can add it directly from the source:

```toml
[dependencies.ownership]
git = "https://github.com/nekitdev/ownership.git"
```

## Usage

One of the main ideas of this crate is to allow going from `'a` to `'static` via switching
to owned types (which is usually done through the `ToOwned` trait).

This is especially useful when working with `Cow<'_, T>`, as the implementation of `IntoOwned`
for `Cow<'_, T>` will result in `Cow<'static, T>`.

## Derive

The `derive` feature allows one to derive the `IntoOwned` trait for custom types.

Firstly, enable the `derive` feature either through the console:

```console
$ cargo add ownership --features derive
```

Or by specifying it in the configuration like so:

```toml
[dependencies.ownership]
version = "0.4.0"
features = ["derive"]
```

And then the `IntoOwned` trait can be derived, for instance:

```rust
use std::borrow::Cow;

use ownership::IntoOwned;

#[derive(IntoOwned)]
pub struct Item<'i, T> {
    pub name: Cow<'i, str>,
    pub value: T,
}
```

This code will generate the following implementation:

```rust
impl<'i, T: IntoOwned> IntoOwned for Item<'i, T> {
    type Owned = Item<'static, <T as IntoOwned>::Owned>;

    fn into_owned(self) -> Self::Owned {
        Self::Owned {
            name: IntoOwned::into_owned(self.name),
            value: IntoOwned::into_owned(self.value),
        }
    }
}
```

Then one can "lift" `Item<'_, T>` to `Item<'static, <T as IntoOwned>::Owned>` by calling
the `into_owned` method.

### Bounds

Since `T` is replaced by `<T as IntoOwned>::Owned` in the `Owned` type, any bound declared
on `T` is also required of `<T as IntoOwned>::Owned`. Such bounds are propagated
automatically, so this code:

```rust
use ownership::IntoOwned;

#[derive(IntoOwned)]
pub struct Item<T: Bound> {
    pub value: T,
}
```

Will generate the following implementation:

```rust
impl<T: Bound> IntoOwned for Item<T>
where
    T: IntoOwned,
    <T as IntoOwned>::Owned: Bound,
{
    type Owned = Item<<T as IntoOwned>::Owned>;

    fn into_owned(self) -> Self::Owned {
        Self::Owned {
            value: IntoOwned::into_owned(self.value),
        }
    }
}
```

Bounds written in `where` clauses are propagated in the same way. Note that associated types
written in shorthand form, like `T::Item`, can not be propagated, as the trait defining them
is unknown to the derive macro; the fully qualified `<T as Trait>::Item` form is propagated
as expected.

## Documentation

You can find the documentation [here][Documentation].

## Support

If you need support with the library, you can send an [email][Email].

## Changelog

You can find the changelog [here][Changelog].

## Security Policy

You can find the Security Policy of `ownership` [here][Security].

## Contributing

If you are interested in contributing to `ownership`, make sure to take a look at the
[Contributing Guide][Contributing Guide], as well as the [Code of Conduct][Code of Conduct].

## License

`ownership` is licensed under the MIT License terms. See [License][License] for details.

[Email]: mailto:support@nekit.dev
[Discord]: https://nekit.dev/chat
[Actions]: https://github.com/nekitdev/ownership/actions
[Changelog]: https://github.com/nekitdev/ownership/blob/main/CHANGELOG.md
[Code of Conduct]: https://github.com/nekitdev/ownership/blob/main/CODE_OF_CONDUCT.md
[Contributing Guide]: https://github.com/nekitdev/ownership/blob/main/CONTRIBUTING.md
[Security]: https://github.com/nekitdev/ownership/blob/main/SECURITY.md
[License]: https://github.com/nekitdev/ownership/blob/main/LICENSE
[Crate]: https://crates.io/crates/ownership
[Documentation]: https://docs.rs/ownership
[License Badge]: https://img.shields.io/crates/l/ownership
[Version Badge]: https://img.shields.io/crates/v/ownership
[Downloads Badge]: https://img.shields.io/crates/dr/ownership
[Documentation Badge]: https://img.shields.io/docsrs/ownership
[Test Badge]: https://github.com/nekitdev/ownership/workflows/test/badge.svg
