# base::utils::functional

Shared Rust utilities and models used by services in this workspace.

## Functional Helpers

This crate exports two macros for Ramda-style partial application:

- `partial!(f, [bound...])`
- `partial_right!(f, [bound...])`

These macros capture bound values and return a closure.

### Single Remaining Argument

Use this shorthand when only one argument remains after partial application.

```rust
use base::utils::functional::{partial, partial_right};

fn add(a: i32, b: i32) -> i32 {
    a + b
}

let add_10 = partial!(add, [10]);
let plus_10 = partial_right!(add, [10]);

assert_eq!(add_10(5), 15);
assert_eq!(plus_10(5), 15);
```

### Multiple Remaining Arguments

Use the 3-argument macro form and name the remaining closure parameters.

```rust
use base::utils::functional{partial, partial_right};

fn format_name(first: String, last: String, suffix: String) -> String {
    format!("{} {}{}", first, last, suffix)
}

let with_first = partial!(format_name, [String::from("Ada")], [last, suffix]);
let with_suffix = partial_right!(format_name, [String::from("!")], [first, last]);

assert_eq!(
    with_first(String::from("Lovelace"), String::from("!")),
    String::from("Ada Lovelace!")
);

assert_eq!(
    with_suffix(String::from("Ada"), String::from("Lovelace")),
    String::from("Ada Lovelace!")
);
```

### Notes

- Bound arguments are captured by move and cloned on each call (`bound.clone()`).
- For cheap scalar values this is trivial; for larger values use `Arc<T>` or another shared owner when needed.
- The helper currently targets ergonomic function-style usage and explicit closure parameter naming for larger arities.
- Running tests: `storm-sword> cargo test -p core --lib` 
