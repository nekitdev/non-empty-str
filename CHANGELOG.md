# Changelog

<!-- changelogging: start -->

## [0.11.2](https://github.com/nekitdev/non-empty-str/tree/v0.11.2) (2025-10-15)

No significant changes.

## [0.11.1](https://github.com/nekitdev/non-empty-str/tree/v0.11.1) (2025-10-14)

### Features

- Added `FromStr` implementation for `NonEmptyString`.

## [0.11.0](https://github.com/nekitdev/non-empty-str/tree/v0.11.0) (2025-10-14)

### Changes

- The entire crate was rewritten. Refer to the [docs](https://docs.rs/non-empty-str) for more info.

## [0.10.0](https://github.com/nekitdev/non-empty-str/tree/v0.10.0) (2025-08-01)

### Features

- Added `AsRef<Self>` for `Str` and `OwnedStr`; also implemented `AsRef<String>` for `OwnedStr`.

- Added `as_string` to `OwnedStr` that returns `&String`.

### Changes

- The `Deref` target of `OwnedStr` was changed to `String` instead of `Str`.

## [0.9.0](https://github.com/nekitdev/non-empty-str/tree/v0.9.0) (2025-07-31)

### Changes

- The entire crate was rewritten; see [docs](https://docs.rs/non-empty-str) for more information.

## [0.8.1](https://github.com/nekitdev/non-empty-str/tree/v0.8.1) (2025-07-24)

No significant changes.

## [0.8.0](https://github.com/nekitdev/non-empty-str/tree/v0.8.0) (2025-04-30)

No significant changes.

## [0.7.1](https://github.com/nekitdev/non-empty-str/tree/v0.7.1) (2025-04-30)

No significant changes.

## [0.7.0](https://github.com/nekitdev/non-empty-str/tree/v0.7.0) (2025-04-24)

No significant changes.

## [0.6.0](https://github.com/nekitdev/non-empty-str/tree/v0.6.0) (2025-04-17)

### Features

- Enabling `static` feature exports `StaticStr` and `StaticCowStr` starting from this version.

### Changes

- Changed `into-static` feature to `static`.

## [0.5.0](https://github.com/nekitdev/non-empty-str/tree/v0.5.0) (2025-03-31)

No significant changes.

## [0.4.0](https://github.com/nekitdev/non-empty-str/tree/v0.4.0) (2025-03-27)

### Features

- Added `OwnedStr` that wraps `String`.

### Removals

- Removed `check_str` and `check` from the `empty` module.

## [0.3.0](https://github.com/nekitdev/non-empty-str/tree/v0.3.0) (2025-03-25)

### Changes

- The `into_static` method on `CowStr<'_>` was moved to implementing `IntoStatic`,
  gated behind the `into-static` feature.

### Removals

- Removed `StaticStr` and `StaticCowStr`.

## [0.2.2](https://github.com/nekitdev/non-empty-str/tree/v0.2.2) (2025-03-22)

### Features

- Added `into_static` for `CowStr` along with the following type aliases:

  ```rust
  pub type StaticCowStr = CowStr<'static>;

  pub type StaticStr = Str<'static>;
  ```

## [0.2.1](https://github.com/nekitdev/non-empty-str/tree/v0.2.1) (2025-03-15)

### Changes

- There are now two implementations of `Deserialize` for `CowStr`, as follows:

  - disabled `borrow` feature (default): `CowStr` allocates new strings;
  - enabled `borrow` feature: `CowStr` borrows from the input.

  The `borrow` feature is disabled by default because enabling it requires
  `#[serde(borrow)]` attributes to be applied to all `CowStr` fields:

  ```rust
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Deserialize, Serialize)]
  pub struct Type<'t> {
      #[serde(borrow)]
      pub name: CowStr<'t>,
      #[serde(borrow)]
      pub description: CowStr<'t>,
  }
  ```

  With `borrow` disabled (again, by default), one does not need any additional attributes:

  ```rust
  use serde::{Deserialize, Serialize};

  #[derive(Debug, Deserialize, Serialize)]
  pub struct Type<'t> {
      pub name: CowStr<'t>,
      pub description: CowStr<'t>,
  }
  ```

## [0.2.0](https://github.com/nekitdev/non-empty-str/tree/v0.2.0) (2025-03-15)

### Features

- Improved ergonomics; please refer to the [documentation](https://docs.rs/non-empty-str)
  for more information.

## [0.1.0](https://github.com/nekitdev/non-empty-str/tree/v0.1.0) (2025-03-14)

No significant changes.
