// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{CellArea, CellLayout, CellRenderer};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

#[derive(Debug)]
pub struct CellLayoutDataCallback {
    callback: ffi::GtkCellLayoutDataFunc,
    user_data: glib::ffi::gpointer,
    destroy_notify: glib::ffi::GDestroyNotify,
}

pub trait CellLayoutImpl: ObjectImpl {
    fn add_attribute(
        &self,
        cell_layout: &Self::Type,
        cell: &CellRenderer,
        attribute: &str,
        column: i32,
    ) {
        unsafe {
            let type_ = ffi::gtk_cell_layout_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkCellLayoutIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).add_attribute.as_ref() {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.to_glib_none().0,
                    attribute.to_glib_none().0,
                    column,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn clear_attributes(&self, cell_layout: &Self::Type, cell: &CellRenderer) {
        unsafe {
            let type_ = ffi::gtk_cell_layout_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkCellLayoutIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).clear_attributes.as_ref() {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.to_glib_none().0,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn get_cells(&self, cell_layout: &Self::Type) -> Vec<CellRenderer> {
        unsafe {
            let type_ = ffi::gtk_cell_layout_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkCellLayoutIface;
            assert!(!iface.is_null());

            let f = (*iface).get_cells.as_ref().unwrap();

            let cells = f(cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0);
            let cells = FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(cells);

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            cells
        }
    }

    fn set_cell_data_func(
        &self,
        cell_layout: &Self::Type,
        cell: &CellRenderer,
        callback: Option<&CellLayoutDataCallback>,
    ) {
        unsafe {
            let type_ = ffi::gtk_cell_layout_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkCellLayoutIface;
            assert!(!iface.is_null());

            let f = (*iface).set_cell_data_func.as_ref().unwrap();

            if let Some(data_cb) = callback {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.to_glib_none().0,
                    data_cb.callback,
                    data_cb.user_data,
                    data_cb.destroy_notify,
                );
            } else {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.to_glib_none().0,
                    None,
                    std::ptr::null_mut(),
                    None,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn reorder(&self, cell_layout: &Self::Type, cell: &CellRenderer, position: i32) {
        {
            unsafe {
                let type_ = ffi::gtk_cell_layout_get_type();
                let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                    as *mut ffi::GtkCellLayoutIface;
                assert!(!iface.is_null());

                if let Some(f) = (*iface).reorder.as_ref() {
                    f(
                        cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                        cell.to_glib_none().0,
                        position,
                    );
                }

                glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            }
        }
    }

    fn clear(&self, cell_layout: &Self::Type) {
        unsafe {
            let type_ = ffi::gtk_cell_layout_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkCellLayoutIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).clear.as_ref() {
                f(cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0);
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn pack_start(&self, cell_layout: &Self::Type, cell: &CellRenderer, expand: bool) {
        unsafe {
            let type_ = ffi::gtk_cell_layout_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkCellLayoutIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).pack_start.as_ref() {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.to_glib_none().0,
                    expand.to_glib(),
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn pack_end(&self, cell_layout: &Self::Type, cell: &CellRenderer, expand: bool) {
        unsafe {
            let type_ = ffi::gtk_cell_layout_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkCellLayoutIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).pack_end.as_ref() {
                f(
                    cell_layout.unsafe_cast_ref::<CellLayout>().to_glib_none().0,
                    cell.to_glib_none().0,
                    expand.to_glib(),
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn get_area(&self, cell_layout: &Self::Type) -> Option<CellArea> {
        unsafe {
            let type_ = ffi::gtk_cell_layout_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkCellLayoutIface;
            assert!(!iface.is_null());

            let ret = if let Some(f) = (*iface).get_area.as_ref() {
                Some(from_glib_none(f(cell_layout
                    .unsafe_cast_ref::<CellLayout>()
                    .to_glib_none()
                    .0)))
            } else {
                None
            };

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            ret
        }
    }
}

unsafe impl<T: CellLayoutImpl> IsImplementable<T> for CellLayout {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let cell_layout_iface = &mut *(iface as *mut ffi::GtkCellLayoutIface);

        cell_layout_iface.get_area = Some(cell_layout_get_area::<T>);
        cell_layout_iface.pack_start = Some(cell_layout_pack_start::<T>);
        cell_layout_iface.pack_end = Some(cell_layout_pack_end::<T>);
        cell_layout_iface.clear = Some(cell_layout_clear::<T>);
        cell_layout_iface.reorder = Some(cell_layout_reorder::<T>);
        cell_layout_iface.add_attribute = Some(cell_layout_add_attribute::<T>);
        cell_layout_iface.clear_attributes = Some(cell_layout_clear_attributes::<T>);
        cell_layout_iface.set_cell_data_func = Some(cell_layout_set_cell_data_func::<T>);
        cell_layout_iface.get_cells = Some(cell_layout_get_cells::<T>);
    }
}

unsafe extern "C" fn cell_layout_get_area<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
) -> *mut ffi::GtkCellArea {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_area(from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref())
        .to_glib_full()
}

unsafe extern "C" fn cell_layout_pack_start<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    expand: glib::ffi::gboolean,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.get_impl();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.pack_start(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &cell,
        from_glib(expand),
    )
}

unsafe extern "C" fn cell_layout_pack_end<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    expand: glib::ffi::gboolean,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.get_impl();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.pack_end(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &cell,
        from_glib(expand),
    )
}

unsafe extern "C" fn cell_layout_clear<T: CellLayoutImpl>(cell_layout: *mut ffi::GtkCellLayout) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.get_impl();

    imp.clear(from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref())
}

unsafe extern "C" fn cell_layout_reorder<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    position: i32,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.get_impl();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.reorder(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &cell,
        position,
    )
}

unsafe extern "C" fn cell_layout_add_attribute<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    attributeptr: *const libc::c_char,
    column: i32,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.get_impl();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);
    let attribute: Borrowed<glib::GString> = from_glib_borrow(attributeptr);

    imp.add_attribute(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &cell,
        &attribute,
        column,
    )
}

unsafe extern "C" fn cell_layout_clear_attributes<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.get_impl();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    imp.clear_attributes(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &cell,
    )
}

unsafe extern "C" fn cell_layout_set_cell_data_func<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
    cellptr: *mut ffi::GtkCellRenderer,
    callback: ffi::GtkCellLayoutDataFunc,
    user_data: glib::ffi::gpointer,
    destroy_notify: glib::ffi::GDestroyNotify,
) {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.get_impl();

    let cell: Borrowed<CellRenderer> = from_glib_borrow(cellptr);

    let callback = if callback.is_none() {
        None
    } else {
        Some(CellLayoutDataCallback {
            callback,
            user_data,
            destroy_notify,
        })
    };

    imp.set_cell_data_func(
        from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref(),
        &cell,
        callback.as_ref(),
    )
}

unsafe extern "C" fn cell_layout_get_cells<T: CellLayoutImpl>(
    cell_layout: *mut ffi::GtkCellLayout,
) -> *mut glib::ffi::GList {
    let instance = &*(cell_layout as *mut T::Instance);
    let imp = instance.get_impl();

    let cells = imp.get_cells(from_glib_borrow::<_, CellLayout>(cell_layout).unsafe_cast_ref());
    ToGlibContainerFromSlice::to_glib_container_from_slice(&cells).0
}
