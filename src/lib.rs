//! Fast, unchecked variants of common std methods.

#![no_std]

extern crate alloc;
use self::{generic_string::GenericString, generic_vec::GenericVec};
use alloc::vec::Vec;

mod generic_string;
mod generic_vec;

/// A trait for `push` without the capacity check.
pub trait PushUnchecked<T> {
    /// Appends an element to the back of a collection without the capacity check.
    ///
    /// # Safety
    ///
    /// The capacity of the collection must be sufficient for the new item.
    unsafe fn push_unchecked(&mut self, value: T);
}

impl<T> PushUnchecked<T> for Vec<T> {
    /// [`Self::push`] without the capacity check.
    ///
    /// # Safety
    ///
    /// `self.len()` must be `< self.capacity()`.
    #[inline]
    unsafe fn push_unchecked(&mut self, value: T) {
        debug_assert!(self.len() < self.capacity());
        if self.len() >= self.capacity() {
            core::hint::unreachable_unchecked();
        }

        core::ptr::write(self.as_mut_ptr().add(self.len()), value);
        self.set_len(self.len() + 1);
    }
}

impl<S: GenericString> PushUnchecked<char> for S {
    /// `push` without the capacity check.
    ///
    /// # Safety
    ///
    /// `self.len() + ch.len_utf8()` must be `<= self.capacity()`.
    #[inline]
    unsafe fn push_unchecked(&mut self, ch: char) {
        let len = self.len();
        let ch_len = ch.len_utf8();
        debug_assert!(len + ch_len <= self.capacity());
        let ptr = self.as_mut_vec().as_mut_ptr().add(len);
        match ch_len {
            1 => {
                *ptr = ch as u8;
            }
            2 => {
                *ptr = (ch as u32 >> 6 & 0x1F) as u8 | 0b1100_0000;
                *ptr.add(1) = (ch as u32 & 0x3F) as u8 | 0b1000_0000;
            }
            3 => {
                *ptr = (ch as u32 >> 12 & 0x0F) as u8 | 0b1110_0000;
                *ptr.add(1) = (ch as u32 >> 6 & 0x3F) as u8 | 0b1000_0000;
                *ptr.add(2) = (ch as u32 & 0x3F) as u8 | 0b1000_0000;
            }
            4 => {
                *ptr = (ch as u32 >> 18 & 0x07) as u8 | 0b1111_0000;
                *ptr.add(1) = (ch as u32 >> 12 & 0x3F) as u8 | 0b1000_0000;
                *ptr.add(2) = (ch as u32 >> 6 & 0x3F) as u8 | 0b1000_0000;
                *ptr.add(3) = (ch as u32 & 0x3F) as u8 | 0b1000_0000;
            }
            _ => core::hint::unreachable_unchecked(),
        }
        self.as_mut_vec().set_len(len + ch_len);
    }
}

/// A trait for `extend` without the capacity check.
pub trait ExtendUnchecked<T> {
    /// Extends a collection with the contents of an iterator without the
    /// capacity check.
    ///
    /// # Safety
    ///
    /// The capacity of the collection must be sufficient for the new items.
    unsafe fn extend_unchecked<I: IntoIterator<Item = T>>(&mut self, iter: I);
}

impl<S: GenericString> ExtendUnchecked<char> for S {
    /// [`Extend::extend`] without the capacity check.
    ///
    /// # Safety
    ///
    /// `self.len() + iter.into_iter().count()` must be `<= self.capacity()`.
    #[inline]
    unsafe fn extend_unchecked<I: IntoIterator<Item = char>>(&mut self, iter: I) {
        for ch in iter {
            self.push_unchecked(ch);
        }
    }
}

impl<'a, S: GenericString> ExtendUnchecked<&'a char> for S {
    /// [`Extend::extend`] without the capacity check.
    ///
    /// # Safety
    ///
    /// `self.len() + iter.into_iter().count()` must be `<= self.capacity()`.
    #[inline]
    unsafe fn extend_unchecked<I: IntoIterator<Item = &'a char>>(&mut self, iter: I) {
        for &ch in iter {
            self.push_unchecked(ch);
        }
    }
}

/// A trait for `extend_from_slice` without the capacity check.
pub trait ExtendFromSliceUnchecked<T> {
    /// Clones and appends all elements in a slice to the collection.
    ///
    /// # Safety
    ///
    /// The capacity of the collection must be sufficient for the new items.
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]);
}

impl<T: Clone> ExtendFromSliceUnchecked<T> for Vec<T> {
    /// [`Self::extend_from_slice`] without the capacity check.
    ///
    /// # Safety
    ///
    /// `other.len()` must be `<= self.capacity() - self.len()`.
    #[inline]
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]) {
        debug_assert!(other.len() <= self.capacity() - self.len());
        if other.len() > self.capacity() - self.len() {
            core::hint::unreachable_unchecked();
        }
        self.extend_from_slice(other);
    }
}

#[cfg(feature = "heapless")]
impl<T: Copy, const N: usize> ExtendFromSliceUnchecked<T> for heapless::Vec<T, N> {
    /// [`Self::extend_from_slice`] without the capacity check.
    ///
    /// # Safety
    ///
    /// `other.len()` must be `<= self.capacity() - self.len()`.
    #[inline]
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]) {
        let len = self.len();
        let count = other.len();
        debug_assert!(count <= self.capacity() - len);
        core::ptr::copy_nonoverlapping(other.as_ptr(), self.as_mut_ptr().add(len), count);
        self.set_len(len + count);
    }
}

/// A trait for `extend_from_within` without the capacity and bounds checks.
pub trait ExtendFromWithinUnchecked {
    /// Copies elements from `src` range to the end of the collection
    /// without the capacity check and the bounds check for the range.
    ///
    /// # Safety
    ///
    /// `src` must be a valid index for the collection.
    /// The capacity of the collection must be sufficient for the new items.
    unsafe fn extend_from_within_unchecked<R>(&mut self, src: R)
    where
        R: core::ops::RangeBounds<usize>;
}

impl<T: Copy, V: GenericVec<Item = T>> ExtendFromWithinUnchecked for V {
    /// `extend_from_within` without the capacity check
    /// and the bounds check for the range.
    ///
    /// # Safety
    ///
    /// - `src` must be a valid index for `self`
    /// - capacity of `self` must be sufficient for the new items
    #[inline]
    unsafe fn extend_from_within_unchecked<R>(&mut self, src: R)
    where
        R: core::ops::RangeBounds<usize>,
    {
        let start = match src.start_bound() {
            core::ops::Bound::Included(&start) => start,
            core::ops::Bound::Excluded(&start) => {
                debug_assert!(start != usize::MAX);
                start + 1
            }
            core::ops::Bound::Unbounded => 0,
        };
        let end = match src.end_bound() {
            core::ops::Bound::Included(&end) => {
                debug_assert!(end != usize::MAX);
                end + 1
            }
            core::ops::Bound::Excluded(&end) => end,
            core::ops::Bound::Unbounded => self.len(),
        };
        debug_assert!(start <= end && end <= self.len());

        let count = end - start;
        debug_assert!(self.capacity() - self.len() >= count);

        // NOTE: miri accepts this memcpy with Vec, but not with heapless::Vec,
        // unless -Zmiri-tree-borrows is used
        core::ptr::copy_nonoverlapping(
            self.as_ptr().add(start),
            self.as_mut_ptr().add(self.len()),
            count,
        );
        self.set_len(self.len() + count);
    }
}

/// A trait for `push_str` without the capacity check.
pub trait PushStrUnchecked {
    /// Appends a given string slice onto the end of this collection without
    /// the capacity check.
    ///
    /// # Safety
    ///
    /// The capacity of the collection must be sufficient for the appended string.
    unsafe fn push_str_unchecked(&mut self, string: &str);
}

impl<S: GenericString> PushStrUnchecked for S {
    /// `push_str` without the capacity check.
    ///
    /// # Safety
    ///
    /// `self.len() + string.len()` must be `<= self.capacity()`.
    #[inline]
    unsafe fn push_str_unchecked(&mut self, string: &str) {
        self.as_mut_vec()
            .extend_from_slice_unchecked(string.as_bytes());
    }
}

/// A trait for `copy_from_slice` without the length check.
pub trait CopyFromSliceUnchecked<T> {
    /// Copies all elements from `src` into `self` without the length check.
    ///
    /// # Safety
    ///
    /// The length of `self` must be equal to the length of `src`.
    unsafe fn copy_from_slice_unchecked(&mut self, src: &[T]);
}

impl<T: Copy> CopyFromSliceUnchecked<T> for [T] {
    /// [`Self::copy_from_slice`] without the length check.
    ///
    /// # Safety
    ///
    /// `self.len()` must be `== src.len()`.
    #[inline]
    unsafe fn copy_from_slice_unchecked(&mut self, src: &[T]) {
        debug_assert!(self.len() == src.len());
        unsafe {
            core::ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr(), self.len());
        }
    }
}

/// A trait to `push` many times without the capacity check.
pub trait PushManyUnchecked<T> {
    /// Appends an element `count` times to the back of a collection without
    /// the capacity check.
    ///
    /// # Safety
    ///
    /// The capacity of the collection must be sufficient for the new items.
    unsafe fn push_many_unchecked(&mut self, value: T, count: usize);
}

impl<V: GenericVec<Item = u8>> PushManyUnchecked<u8> for V {
    /// Appends a `byte` `count` times to the back of the vector without the
    /// capacity check.
    ///
    /// # Safety
    ///
    /// The capacity of the vector must be sufficient for the new bytes.
    #[inline]
    unsafe fn push_many_unchecked(&mut self, byte: u8, count: usize) {
        debug_assert!(self.capacity() - self.len() >= count);

        core::ptr::write_bytes(self.as_mut_ptr().add(self.len()), byte, count);
        self.set_len(self.len() + count);
    }
}

impl<V: GenericVec<Item = i8>> PushManyUnchecked<i8> for V {
    /// Appends a `byte` `count` times to the back of the vector without the
    /// capacity check.
    ///
    /// # Safety
    ///
    /// The capacity of the vector must be sufficient for the new bytes.
    #[inline]
    unsafe fn push_many_unchecked(&mut self, byte: i8, count: usize) {
        debug_assert!(self.capacity() - self.len() >= count);

        core::ptr::write_bytes(
            self.as_mut_ptr().add(self.len()),
            byte.cast_unsigned(),
            count,
        );
        self.set_len(self.len() + count);
    }
}

/// Duplicate exports in `prelude` to comply with `clippy::wildcard_imports`.
pub mod prelude {
    pub use super::*;
}
