#!/bin/sh

RUSTFLAGS="--emit=llvm-ir" cargo test --release --no-run --test codegen
FileCheck --input-file target/release/deps/codegen-*.ll tests/codegen.rs
