#![allow(clippy::no_mangle_with_rust_abi)]

use unchecked_std::ExtendFromSliceUnchecked;

const SLICE_LEN: usize = 10;
const N_EXTENDS: usize = 16;

// CHECK-LABEL: @test_extend_from_slice
#[no_mangle]
pub fn test_extend_from_slice(xs: &[u8; SLICE_LEN]) -> Vec<u8> {
    // CHECK: do_reserve_and_handle
    let mut res = Vec::with_capacity(N_EXTENDS * SLICE_LEN);
    for _ in 0..N_EXTENDS {
        res.extend_from_slice(xs);
    }
    res
}

// CHECK-LABEL: @test_extend_from_slice_unchecked
#[no_mangle]
pub fn test_extend_from_slice_unchecked(xs: &[u8; SLICE_LEN]) -> Vec<u8> {
    // CHECK-NOT: do_reserve_and_handle
    let mut res = Vec::with_capacity(N_EXTENDS * SLICE_LEN);
    for _ in 0..N_EXTENDS {
        unsafe { res.extend_from_slice_unchecked(xs) };
    }
    res
}
