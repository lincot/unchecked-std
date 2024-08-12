# unchecked-std

Rust standard library methods with some checks removed
for the sake of performance and binary size.

For safety, assertions are present in debug mode.

Most implementations do not rely on corresponding std methods, except for
`extend_from_slice_unchecked` which works based on `unreachable_unchecked`
and has [a codegen test](tests/codegen.rs)
to confirm that the capacity check gets elided.

The crate is `no_std`, but requires `alloc`.

## Example

`format!` way:

```rust
fn hello_format(name: &str) -> String {
    format!("Hello, {name}!")
}
```

macro-free `std` way:

```rust
fn hello_checked(name: &str) -> String {
    let mut s = String::with_capacity("Hello, !".len() + name.len());
    s.push_str("Hello, ");
    s.push_str(name);
    s.push('!');
    s
}
```

`unchecked-std` way:

```rust
use unchecked_std::prelude::*;

fn hello_unchecked(name: &str) -> String {
    let mut s = String::with_capacity("Hello, !".len() + name.len());
    // SAFETY: `s` has been initialized with sufficient capacity
    unsafe {
        s.push_str_unchecked("Hello, ");
        s.push_str_unchecked(name);
        s.push_unchecked('!');
    }
    s
}
```

The [benchmark](benches/bench.rs) result is:

```
test bench_hello_format    ... bench:          29.50 ns/iter (+/- 0.74)
test bench_hello_checked   ... bench:          15.47 ns/iter (+/- 0.31)
test bench_hello_unchecked ... bench:          11.45 ns/iter (+/- 0.65)
```

## Feature flags

`heapless` adds unchecked methods for
[heapless](https://github.com/rust-embedded/heapless) data structures.
