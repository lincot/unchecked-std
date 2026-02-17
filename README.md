# unchecked-std

Common Rust standard library methods with capacity and bounds checks removed to
improve performance.

For safety, assertions are enabled in debug builds.

Most APIs are unchecked counterparts of standard methods, with an exception of
`push_many_unchecked`, which doesn't have a corresponding safe std method.

Most implementations are manual, except for `extend_from_slice_unchecked` which
calls `extend_from_slice` and uses `unreachable_unchecked` with
[a codegen test](tests/codegen.rs) to confirm that the capacity check gets
elided.

The crate is `no_std`, but requires `alloc`.

## Feature flags

The `heapless` feature adds unchecked methods for
[heapless](https://github.com/rust-embedded/heapless) data structures.

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

Example [benchmark](benches/bench.rs) result:

```
test bench_hello_format    ... bench:          30.56 ns/iter (+/- 0.30)
test bench_hello_checked   ... bench:          14.57 ns/iter (+/- 0.44)
test bench_hello_unchecked ... bench:           9.98 ns/iter (+/- 0.22)
```
