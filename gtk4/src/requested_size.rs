// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct RequestedSize(ffi::GtkRequestedSize);

impl RequestedSize {
    pub fn get_minimum_size(&self) -> i32 {
        self.0.minimum_size
    }

    pub fn get_natural_size(&self) -> i32 {
        self.0.natural_size
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GtkRequestedSize> for RequestedSize {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GtkRequestedSize, Self> {
        let ptr: *mut RequestedSize = &mut *self;
        StashMut(ptr as *mut ffi::GtkRequestedSize, self)
    }
}
