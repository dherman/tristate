# TriState

A three-valued type equivalent to `Option<bool>`:

```rust
enum TriState {
    Yes,
    No,
    Unknown
}
```

A nice way to use this type is with a domain-specific type alias via `pub use`.
(For esoteric reasons, a simple "typedef" type alias doesn't work.) For example,
a spam classifier:

```rust
extern crate tristate;

pub use tristate::TriState as Spam;

trait Classify {
    fn classify(&self) -> Spam;
}

impl Classify for Message { /* ... */ }

// ...

match message.classify() {
    Spam::Yes     => /* ... */,
    Spam::No      => /* ... */,
    Spam::Unknown => /* ... */
}
```
