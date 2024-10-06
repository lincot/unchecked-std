use super::{generic_vec::GenericVec, ExtendFromSliceUnchecked};
use alloc::{string::String, vec::Vec};

pub trait GenericString {
    type InnerVec: GenericVec<Item = u8> + ExtendFromSliceUnchecked<u8>;

    fn len(&self) -> usize;
    fn capacity(&self) -> usize;
    unsafe fn as_mut_vec(&mut self) -> &mut Self::InnerVec;
}

impl GenericString for String {
    type InnerVec = Vec<u8>;

    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.capacity()
    }

    #[inline]
    unsafe fn as_mut_vec(&mut self) -> &mut Self::InnerVec {
        self.as_mut_vec()
    }
}

#[cfg(feature = "heapless")]
impl<const N: usize> GenericString for heapless::String<N> {
    type InnerVec = heapless::Vec<u8, N>;

    #[inline]
    fn len(&self) -> usize {
        self.as_str().len()
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.capacity()
    }

    #[inline]
    unsafe fn as_mut_vec(&mut self) -> &mut Self::InnerVec {
        self.as_mut_vec()
    }
}
