# TriState

A three-valued type equivalent to `Option<bool>`:

```rust
enum TriState {
    Yes,
    No,
    Unknown
}
```

A nice way to use this type is with a domain-specific type alias. For example,
a spam classifier:

```rust
type Spam = TriState;

trait Classify {
    fn classify() -> Spam;
}

impl Classify for Message { /* ... */ }

// ...

match message.classify() {
    Spam::Yes     => /* ... */,
    Spam::No      => /* ... */,
    Spam::Unknown => /* ... */
}
```
