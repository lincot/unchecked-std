#![no_std]

extern crate alloc;
use alloc::{string::String, vec::Vec};

pub trait PushUnchecked<T> {
    /// Appends an element to the back of a collection without the capacity check.
    ///
    /// # Safety
    ///
    /// The capacity of the collection must be sufficient for the new item.
    unsafe fn push_unchecked(&mut self, value: T);
}

impl<T> PushUnchecked<T> for Vec<T> {
    /// [`Vec::push`] without the capacity check.
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

impl PushUnchecked<char> for String {
    /// [`String::push`] without the capacity check.
    ///
    /// # Safety
    ///
    /// `self.len() + ch.len_utf8()` must be `<= self.capacity()`.
    #[inline]
    unsafe fn push_unchecked(&mut self, ch: char) {
        let len = self.len();
        let ptr = self.as_mut_vec().as_mut_ptr().add(len);
        let ch_len = ch.len_utf8();
        debug_assert!(len + ch_len <= self.capacity());
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

pub trait ExtendUnchecked<T, I> {
    /// Extends a collection with the contents of an iterator without the capacity check.
    ///
    /// # Safety
    ///
    /// The capacity of the collection must be sufficient for the new items.
    unsafe fn extend_unchecked(&mut self, iter: I);
}

impl<I: IntoIterator<Item = char>> ExtendUnchecked<char, I> for String {
    /// `<String as Extend<char>>::extend` without the capacity check.
    ///
    /// # Safety
    ///
    /// `self.len() + iter.into_iter().count()` must be `<= self.capacity()`.
    #[inline]
    unsafe fn extend_unchecked(&mut self, iter: I) {
        for value in iter {
            self.push_unchecked(value);
        }
    }
}

pub trait ExtendFromSliceUnchecked<T> {
    /// Clones and appends all elements in a slice to the collection.
    ///
    /// # Safety
    ///
    /// The capacity of the collection must be sufficient for the new items.
    unsafe fn extend_from_slice_unchecked(&mut self, other: &[T]);
}

impl<T: Clone> ExtendFromSliceUnchecked<T> for Vec<T> {
    /// [`Vec::extend_from_slice`] without the capacity check.
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
    /// [`heapless::Vec::extend_from_slice`] without the capacity check.
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

pub trait ExtendFromWithinUnchecked {
    /// Copies elements from `src` range to the end of the collection
    /// without the capacity check and the bounds check for the range.
    ///
    /// # Safety
    ///
    /// The capacity of the collection must be sufficient for the new items.
    unsafe fn extend_from_within_unchecked(&mut self, src: core::ops::Range<usize>);
}

impl<T: Copy> ExtendFromWithinUnchecked for Vec<T> {
    /// [`Vec::extend_from_within`] without the capacity check
    /// and the bounds check for the range.
    ///
    /// # Safety
    ///
    /// - `src` needs to be valid index
    /// - `self.capacity() - self.len()` must be `>= src.len()`
    unsafe fn extend_from_within_unchecked(&mut self, src: core::ops::Range<usize>) {
        let count = src.len();
        debug_assert!(src.start <= src.end || src.end <= self.len());
        debug_assert!(self.capacity() - self.len() >= count);
        let source = self.get_unchecked(src);
        core::ptr::copy_nonoverlapping(source.as_ptr(), self.as_mut_ptr().add(self.len()), count);
        self.set_len(self.len() + count);
    }
}

pub trait PushStrUnchecked {
    /// Appends a given string slice onto the end of this collection without the capacity check.
    ///
    /// # Safety
    ///
    /// The capacity of the collection must be sufficient for the appended string.
    unsafe fn push_str_unchecked(&mut self, string: &str);
}

impl PushStrUnchecked for String {
    /// [`String::push_str`] without the capacity check.
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

pub trait CopyFromSliceUnchecked<T> {
    /// Copies all elements from `src` into `self` without the length check.
    ///
    /// # Safety
    ///
    /// The length of `self` must be equal to the length of `src`.
    unsafe fn copy_from_slice_unchecked(&mut self, src: &[T]);
}

impl<T: Copy> CopyFromSliceUnchecked<T> for [T] {
    /// [`<[T]>::copy_from_slice`] without the length check.
    ///
    /// # Safety
    ///
    /// `self.len()` must be `== src.len()`.
    unsafe fn copy_from_slice_unchecked(&mut self, src: &[T]) {
        debug_assert!(self.len() == src.len());
        unsafe {
            core::ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr(), self.len());
        }
    }
}

pub mod prelude {
    pub use super::*;
}
