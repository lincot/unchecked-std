#![no_std]

extern crate alloc;
use alloc::{string::String, vec, vec::Vec};
use core::{
    array,
    ops::{Bound, RangeBounds},
};
use rand::{distributions::Alphanumeric, prelude::*};
use rand_pcg::Pcg64Mcg;
use unchecked_std::prelude::*;

#[test]
fn test_vec_push_unchecked() {
    let len = 100;
    let mut v = Vec::with_capacity(len);
    let mut v_unchecked = Vec::with_capacity(len);
    for _ in 0..len {
        let value = 5u8;
        v.push(value);
        unsafe { v_unchecked.push_unchecked(value) };
        assert_eq!(v, v_unchecked);
    }
}

#[test]
fn test_string_push_unchecked() {
    const N_CHARS: usize = 100;
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut s = String::with_capacity(4 * N_CHARS);
    let mut s_unchecked = String::with_capacity(4 * N_CHARS);
    #[cfg(feature = "heapless")]
    let mut s_heapless = heapless::String::<{ 4 * N_CHARS }>::new();

    for _ in 0..N_CHARS {
        let ch = if rng.gen::<f64>() < 0.25 {
            rng.sample(Alphanumeric) as char
        } else {
            rng.gen()
        };
        s.push(ch);
        unsafe { s_unchecked.push_unchecked(ch) };
        assert_eq!(s, s_unchecked);
        #[cfg(feature = "heapless")]
        unsafe {
            s_heapless.push_unchecked(ch);
            assert_eq!(s, s_heapless.as_str());
        }
    }
}

#[test]
fn test_string_extend_unchecked() {
    const N_CHARS: usize = 100;
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let chars: [_; N_CHARS] = array::from_fn(|_| {
        if rng.gen::<f64>() < 0.25 {
            rng.sample(Alphanumeric) as char
        } else {
            rng.gen()
        }
    });
    let mut s = String::with_capacity(2 * 4 * chars.len());
    let mut s_unchecked = String::with_capacity(2 * 4 * chars.len());
    #[cfg(feature = "heapless")]
    let mut s_heapless = heapless::String::<{ 2 * 4 * N_CHARS }>::new();
    s.extend(&chars);
    unsafe { s_unchecked.extend_unchecked(&chars) };
    assert_eq!(s, s_unchecked);
    #[cfg(feature = "heapless")]
    {
        unsafe { s_heapless.extend_unchecked(&chars) };
        assert_eq!(s, s_heapless.as_str());
    }
    s.extend(chars);
    unsafe { s_unchecked.extend_unchecked(chars) };
    assert_eq!(s, s_unchecked);
    #[cfg(feature = "heapless")]
    {
        unsafe { s_heapless.extend_unchecked(chars) };
        assert_eq!(s, s_heapless.as_str());
    }
}

#[test]
fn test_vec_extend_from_slice_unchecked() {
    const LEN: usize = 100;
    const N_SLICES: usize = 3;
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut v = Vec::with_capacity(LEN * N_SLICES);
    let mut v_unchecked = Vec::with_capacity(LEN * N_SLICES);
    #[cfg(feature = "heapless")]
    let mut v_heapless = heapless::Vec::<_, { LEN * N_SLICES }>::new();
    for _ in 0..N_SLICES {
        let sl: [usize; LEN] = array::from_fn(|_| rng.gen());

        v.extend_from_slice(&sl);
        unsafe { v_unchecked.extend_from_slice_unchecked(&sl) };
        assert_eq!(v, v_unchecked);
        #[cfg(feature = "heapless")]
        {
            unsafe { v_heapless.extend_from_slice_unchecked(&sl) };
            assert_eq!(v, v_heapless.as_slice());
        }
    }
}

#[test]
fn test_vec_extend_from_within_unchecked() {
    unsafe fn test_range(src: impl RangeBounds<usize> + Clone) {
        const INIT: [u32; 10] = [1, 5, 85, 1_348_678, 34, 78_678_675, 69, 234, 42, 0];
        let mut v = Vec::with_capacity(2 * INIT.len());
        v.extend(INIT);
        let mut v_unchecked = Vec::with_capacity(2 * INIT.len());
        v_unchecked.extend(INIT);

        v.extend_from_within(src.clone());
        v_unchecked.extend_from_within_unchecked(src.clone());
        assert_eq!(v, v_unchecked);

        #[cfg(feature = "heapless")]
        {
            let mut v_heapless = heapless::Vec::<_, { 2 * INIT.len() }>::new();
            v_heapless.extend(INIT);
            v_heapless.extend_from_within_unchecked(src);
            assert_eq!(v, v_heapless.as_slice());
        }
    }

    unsafe fn test_range_zero_sized(len: usize, src: impl RangeBounds<usize> + Clone) {
        let mut v = Vec::<()>::with_capacity(usize::MAX);
        v.set_len(len);
        let mut v_unchecked = v.clone();

        v.extend_from_within(src.clone());

        v_unchecked.extend_from_within_unchecked(src.clone());

        assert_eq!(v.len(), v_unchecked.len());

        #[cfg(feature = "heapless")]
        {
            let mut v_heapless = heapless::Vec::<(), { usize::MAX }>::new();
            v_heapless.set_len(len);
            v_heapless.extend_from_within_unchecked(src);
            assert_eq!(v.len(), v_heapless.len());
        }
    }

    let mut v = vec![0, 1, 2];
    v.reserve(3);
    unsafe { v.extend_from_within_unchecked(..3) };
    assert_eq!(v, [0, 1, 2, 0, 1, 2]);

    unsafe {
        test_range(0..=0);
        test_range(0..3);
        test_range(..3);
        test_range(0..=3);
        test_range(1..3);
        test_range(1..=3);
        test_range(0..9);
        test_range(0..10);
        test_range(..10);
        test_range(9..10);
        test_range(10..10);
        for start in [Bound::Unbounded, Bound::Included(0), Bound::Excluded(0)] {
            for end in [
                Bound::Unbounded,
                Bound::Included(0),
                Bound::Excluded(0),
                Bound::Included(9),
                Bound::Excluded(9),
                Bound::Excluded(10),
            ] {
                if (start, end) != (Bound::Excluded(0), Bound::Excluded(0)) {
                    test_range((start, end));
                }
            }
        }

        test_range_zero_sized(usize::MAX - 5, 0..5);
        test_range_zero_sized(usize::MAX - 5, (Bound::Unbounded, Bound::Included(4)));
        test_range_zero_sized(usize::MAX, (Bound::Unbounded, Bound::Excluded(0)));
        test_range_zero_sized(usize::MAX - 1, (Bound::Unbounded, Bound::Excluded(1)));
        test_range_zero_sized(
            usize::MAX - 1,
            (
                Bound::Included(usize::MAX - 2),
                Bound::Excluded(usize::MAX - 1),
            ),
        );
    }
}

#[test]
fn test_string_push_str_unchecked() {
    const N_STRINGS_TO_PUSH: usize = 5;
    const N_CHARS: usize = 100;
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut s = String::with_capacity(N_STRINGS_TO_PUSH * 4 * N_CHARS);
    let mut s_unchecked = String::with_capacity(N_STRINGS_TO_PUSH * 4 * N_CHARS);
    #[cfg(feature = "heapless")]
    let mut s_heapless = heapless::String::<{ N_STRINGS_TO_PUSH * 4 * N_CHARS }>::new();
    for _ in 0..N_STRINGS_TO_PUSH {
        let mut string_to_push = String::with_capacity(4 * N_CHARS);
        for _ in 0..rng.gen_range(0..N_CHARS) {
            let ch = if rng.gen::<f64>() < 0.25 {
                rng.sample(Alphanumeric) as char
            } else {
                rng.gen()
            };
            string_to_push.push(ch);
        }

        s.push_str(&string_to_push);
        unsafe { s_unchecked.push_str_unchecked(&string_to_push) };
        assert_eq!(s, s_unchecked);
        #[cfg(feature = "heapless")]
        {
            unsafe { s_heapless.push_str_unchecked(&string_to_push) };
            assert_eq!(s, s_heapless.as_str());
        }
    }
}

#[test]
fn test_copy_from_slice_unchecked() {
    let mut arr: [usize; 100] = array::from_fn(|i| i);
    let mut arr_unchecked: [usize; 100] = array::from_fn(|i| i);
    let src0: [usize; 200] = array::from_fn(|i| 1337 * i);
    let src1: [usize; 50] = array::from_fn(|i| 42 * i);
    arr.copy_from_slice(&src0[50..150]);
    unsafe { arr_unchecked.copy_from_slice_unchecked(&src0[50..150]) };
    assert_eq!(arr, arr_unchecked);
    arr[25..75].copy_from_slice(&src1);
    unsafe { arr_unchecked[25..75].copy_from_slice_unchecked(&src1) };
    assert_eq!(arr, arr_unchecked);
}
