# unchecked-std

Some methods from the Rust standard library without some checks
made non-reallocating and non-panicking
for the sake of performance and binary size.

For safety, assertions are present in debug mode.

The crate is `no_std`, but ruquires `alloc`.

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
test bench_hello_format    ... bench:          28 ns/iter (+/- 0)
test bench_hello_checked   ... bench:          15 ns/iter (+/- 0)
test bench_hello_unchecked ... bench:          11 ns/iter (+/- 0)
```

## Feature flags

`heapless` adds unchecked methods for
[heapless](https://github.com/rust-embedded/heapless) data structures.
