use std::ptr::NonNull;

pub struct RawPtr<T: Sized> {
    ptr: NonNull<T>,
}

impl <T: Sized> RawPtr<T> {
    pub fn new(ptr: *const T) -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(ptr as *mut T)},
        }
    }
}