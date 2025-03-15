There are now two implementations of `Deserialize` for `CowStr`, as follows:

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
