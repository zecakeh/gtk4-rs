// Take a look at the license at the top of the repository in the LICENSE file.

use crate::FontChooser;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, GString, ObjectExt, Quark};
use once_cell::sync::Lazy;
use pango::{FontFace, FontFamily, FontMap};

#[derive(Debug)]
pub struct FilterCallback {
    filter_func: ffi::GtkFontFilterFunc,
    user_data: glib::ffi::gpointer,
    destroy_ptr: glib::ffi::GDestroyNotify,
}

pub trait FontChooserImpl: ObjectImpl {
    fn get_font_family(&self, font_chooser: &Self::Type) -> Option<FontFamily>;
    fn get_font_face(&self, font_chooser: &Self::Type) -> Option<FontFace>;
    fn get_font_size(&self, font_chooser: &Self::Type) -> i32;
    fn set_filter_func(&self, font_chooser: &Self::Type, callback: Option<&FilterCallback>);

    fn set_font_map(&self, font_chooser: &Self::Type, map: Option<&FontMap>) {
        unsafe {
            let type_ = ffi::gtk_print_operation_preview_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkFontChooserIface;
            assert!(!iface.is_null());

            ((*iface).set_font_map.as_ref().unwrap())(
                font_chooser
                    .unsafe_cast_ref::<FontChooser>()
                    .to_glib_none()
                    .0,
                map.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn get_font_map(&self, font_chooser: &Self::Type) -> Option<FontMap> {
        unsafe {
            let type_ = ffi::gtk_print_operation_preview_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkFontChooserIface;
            assert!(!iface.is_null());

            let ret = ((*iface).get_font_map.as_ref().unwrap())(
                font_chooser
                    .unsafe_cast_ref::<FontChooser>()
                    .to_glib_none()
                    .0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            from_glib_none(ret)
        }
    }

    fn font_activated(&self, font_chooser: &Self::Type, font_name: &str) {
        unsafe {
            let type_ = ffi::gtk_print_operation_preview_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkFontChooserIface;
            assert!(!iface.is_null());

            ((*iface).font_activated.as_ref().unwrap())(
                font_chooser
                    .unsafe_cast_ref::<FontChooser>()
                    .to_glib_none()
                    .0,
                font_name.to_glib_none().0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }
}

unsafe impl<T: FontChooserImpl> IsImplementable<T> for FontChooser {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let font_chooser_iface = &mut *(iface as *mut ffi::GtkFontChooserIface);
        font_chooser_iface.get_font_family = Some(font_chooser_get_font_family::<T>);
        font_chooser_iface.get_font_face = Some(font_chooser_get_font_face::<T>);
        font_chooser_iface.get_font_size = Some(font_chooser_get_font_size::<T>);
        font_chooser_iface.font_activated = Some(font_chooser_font_activated::<T>);
        font_chooser_iface.set_font_map = Some(font_chooser_set_font_map::<T>);
        font_chooser_iface.get_font_map = Some(font_chooser_get_font_map::<T>);
        font_chooser_iface.set_filter_func = Some(font_chooser_set_filter_func::<T>);
    }
}

static FONT_CHOOSER_GET_FONT_FAMILY_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk4-rs-subclass-font-chooser-font-family"));

#[derive(Debug)]
struct FontFamilyWrapper(*mut pango::ffi::PangoFontFamily);

impl Drop for FontFamilyWrapper {
    fn drop(&mut self) {
        unsafe { glib::gobject_ffi::g_object_unref(self.0 as *mut _) }
    }
}

unsafe extern "C" fn font_chooser_get_font_family<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
) -> *mut pango::ffi::PangoFontFamily {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow::<_, FontChooser>(font_chooser);

    let ret = imp.get_font_family(wrap.unsafe_cast_ref());
    if let Some(font_family) = ret {
        let font_family = font_family.to_glib_full();
        wrap.set_qdata(
            *FONT_CHOOSER_GET_FONT_FAMILY_QUARK,
            FontFamilyWrapper(font_family),
        );
        font_family
    } else {
        std::ptr::null_mut()
    }
}

static FONT_CHOOSER_GET_FONT_FACE_QUARK: Lazy<Quark> =
    Lazy::new(|| Quark::from_string("gtk4-rs-subclass-font-chooser-font-face"));

#[derive(Debug)]
struct FontFaceWrapper(*mut pango::ffi::PangoFontFace);

impl Drop for FontFaceWrapper {
    fn drop(&mut self) {
        unsafe { glib::gobject_ffi::g_object_unref(self.0 as *mut _) }
    }
}

unsafe extern "C" fn font_chooser_get_font_face<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
) -> *mut pango::ffi::PangoFontFace {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.get_impl();
    let wrap = from_glib_borrow::<_, FontChooser>(font_chooser);

    let ret = imp.get_font_face(wrap.unsafe_cast_ref());
    if let Some(font_face) = ret {
        let font_face = font_face.to_glib_full();
        wrap.set_qdata(
            *FONT_CHOOSER_GET_FONT_FACE_QUARK,
            FontFaceWrapper(font_face),
        );
        font_face
    } else {
        std::ptr::null_mut()
    }
}

unsafe extern "C" fn font_chooser_get_font_size<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
) -> i32 {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_font_size(from_glib_borrow::<_, FontChooser>(font_chooser).unsafe_cast_ref())
}

unsafe extern "C" fn font_chooser_font_activated<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
    font_nameptr: *const libc::c_char,
) {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.get_impl();
    let font_name: Borrowed<GString> = from_glib_borrow(font_nameptr);

    imp.font_activated(
        from_glib_borrow::<_, FontChooser>(font_chooser).unsafe_cast_ref(),
        &font_name,
    )
}

unsafe extern "C" fn font_chooser_set_font_map<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
    font_mapptr: *mut pango::ffi::PangoFontMap,
) {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.get_impl();
    let font_map: Borrowed<Option<FontMap>> = from_glib_borrow(font_mapptr);

    imp.set_font_map(
        from_glib_borrow::<_, FontChooser>(font_chooser).unsafe_cast_ref(),
        font_map.as_ref().as_ref(),
    )
}

unsafe extern "C" fn font_chooser_get_font_map<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
) -> *mut pango::ffi::PangoFontMap {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_font_map(from_glib_borrow::<_, FontChooser>(font_chooser).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn font_chooser_set_filter_func<T: FontChooserImpl>(
    font_chooser: *mut ffi::GtkFontChooser,
    filter_func: ffi::GtkFontFilterFunc,
    user_data: glib::ffi::gpointer,
    destroy_ptr: glib::ffi::GDestroyNotify,
) {
    let instance = &*(font_chooser as *mut T::Instance);
    let imp = instance.get_impl();

    let callback = if filter_func.is_some() {
        None
    } else {
        Some(FilterCallback {
            filter_func,
            user_data,
            destroy_ptr,
        })
    };

    imp.set_filter_func(
        from_glib_borrow::<_, FontChooser>(font_chooser).unsafe_cast_ref(),
        callback.as_ref(),
    );
}
