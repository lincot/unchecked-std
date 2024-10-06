use alloc::vec::Vec;

pub trait GenericVec {
    type Item;

    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    fn as_ptr(&self) -> *const Self::Item;
    fn as_mut_ptr(&mut self) -> *mut Self::Item;
    unsafe fn set_len(&mut self, new_len: usize);
}

impl<T> GenericVec for Vec<T> {
    type Item = T;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.capacity()
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        self.as_ptr()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }

    #[inline]
    unsafe fn set_len(&mut self, new_len: usize) {
        self.set_len(new_len);
    }
}

#[cfg(feature = "heapless")]
impl<T, const N: usize> GenericVec for heapless::Vec<T, N> {
    type Item = T;

    #[inline]
    fn len(&self) -> usize {
        self.as_slice().len()
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.capacity()
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        self.as_ptr()
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        self.as_mut_ptr()
    }

    #[inline]
    unsafe fn set_len(&mut self, new_len: usize) {
        self.set_len(new_len);
    }
}
