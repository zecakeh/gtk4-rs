// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct ToplevelSize(ffi::GdkToplevelSize);

impl ToplevelSize {
    #[doc(alias = "gdk_toplevel_size_get_bounds")]
    pub fn get_bounds(&self) -> (i32, i32) {
        unsafe {
            let bounds_width = std::ptr::null_mut();
            let bounds_height = std::ptr::null_mut();

            ffi::gdk_toplevel_size_get_bounds(
                self.to_glib_none().0 as *mut _,
                bounds_width,
                bounds_height,
            );
            (bounds_width as i32, bounds_height as i32)
        }
    }

    #[doc(alias = "gdk_toplevel_size_set_min_size")]
    pub fn set_min_size(&mut self, min_width: i32, min_height: i32) {
        unsafe {
            ffi::gdk_toplevel_size_set_min_size(self.to_glib_none_mut().0, min_width, min_height);
        }
    }

    #[doc(alias = "gdk_toplevel_size_set_shadow_width")]
    pub fn set_shadow_width(&mut self, left: i32, right: i32, top: i32, bottom: i32) {
        unsafe {
            ffi::gdk_toplevel_size_set_shadow_width(
                self.to_glib_none_mut().0,
                left,
                right,
                top,
                bottom,
            );
        }
    }

    #[doc(alias = "gdk_toplevel_size_set_size")]
    pub fn set_size(&mut self, width: i32, height: i32) {
        unsafe {
            ffi::gdk_toplevel_size_set_size(self.to_glib_none_mut().0, width, height);
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GdkToplevelSize> for ToplevelSize {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GdkToplevelSize, Self> {
        let ptr: *const ToplevelSize = &*self;
        Stash(ptr as *const ffi::GdkToplevelSize, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GdkToplevelSize> for ToplevelSize {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GdkToplevelSize, Self> {
        let ptr: *mut ToplevelSize = &mut *self;
        StashMut(ptr as *mut ffi::GdkToplevelSize, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::GdkToplevelSize> for ToplevelSize {
    unsafe fn from_glib_none(ptr: *const ffi::GdkToplevelSize) -> Self {
        *(ptr as *const ToplevelSize)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::GdkToplevelSize> for ToplevelSize {
    unsafe fn from_glib_none(ptr: *mut ffi::GdkToplevelSize) -> Self {
        *(ptr as *mut ToplevelSize)
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const ffi::GdkToplevelSize> for ToplevelSize {
    unsafe fn from_glib_borrow(
        ptr: *const ffi::GdkToplevelSize,
    ) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *const ToplevelSize))
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::GdkToplevelSize> for ToplevelSize {
    unsafe fn from_glib_borrow(ptr: *mut ffi::GdkToplevelSize) -> glib::translate::Borrowed<Self> {
        glib::translate::Borrowed::new(*(ptr as *mut ToplevelSize))
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::GdkToplevelSize> for ToplevelSize {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut ffi::GdkToplevelSize) -> Self {
        *(ptr as *mut ToplevelSize)
    }
}

impl glib::StaticType for ToplevelSize {
    #[doc(alias = "gdk_toplevel_size_get_type")]
    fn static_type() -> glib::types::Type {
        skip_assert_initialized!();
        unsafe { from_glib(ffi::gdk_toplevel_size_get_type()) }
    }
}

impl<'a> glib::value::FromValueOptional<'a> for ToplevelSize {
    unsafe fn from_value_optional(value: &'a glib::Value) -> Option<Self> {
        from_glib_full(glib::gobject_ffi::g_value_dup_boxed(value.to_glib_none().0)
            as *mut ffi::GdkToplevelSize)
    }
}

impl glib::value::SetValue for ToplevelSize {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            this.to_glib_none().0 as glib::ffi::gconstpointer,
        )
    }
}

impl glib::value::SetValueOptional for ToplevelSize {
    unsafe fn set_value_optional(value: &mut glib::Value, this: Option<&Self>) {
        glib::gobject_ffi::g_value_set_boxed(
            value.to_glib_none_mut().0,
            this.to_glib_none().0 as glib::ffi::gconstpointer,
        )
    }
}
