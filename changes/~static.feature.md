Added `into_static` for `CowStr` along with the following type aliases:

```rust
pub type StaticCowStr = CowStr<'static>;

pub type StaticStr = Str<'static>;
```
