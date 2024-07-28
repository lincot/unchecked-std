use core::ops::{Bound, RangeBounds};
use unchecked_std::prelude::*;

#[test]
fn test_extend_from_within_unchecked() {
    unsafe fn test_range(src: impl RangeBounds<usize> + Clone) {
        let mut v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut v_unchecked = v.clone();

        v.reserve(v.len());
        v.extend_from_within(src.clone());

        v_unchecked.reserve(v_unchecked.len());
        v_unchecked.extend_from_within_unchecked(src);

        assert_eq!(v, v_unchecked);
    }

    unsafe fn test_range_zero_sized(len: usize, src: impl RangeBounds<usize> + Clone) {
        let mut v = Vec::<()>::with_capacity(usize::MAX);
        v.set_len(len);
        let mut v_unchecked = v.clone();

        v.extend_from_within(src.clone());

        v_unchecked.extend_from_within_unchecked(src);

        assert_eq!(v.len(), v_unchecked.len());
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
