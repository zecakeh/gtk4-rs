// Take a look at the license at the top of the repository in the LICENSE file.

use crate::CellEditable;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait CellEditableImpl: ObjectImpl {
    fn editing_done(&self, cell_editable: &Self::Type) {
        unsafe {
            let type_ = ffi::gtk_cell_editable_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkCellEditableIface;
            assert!(!iface.is_null());

            ((*iface).editing_done.as_ref().unwrap())(
                cell_editable
                    .unsafe_cast_ref::<CellEditable>()
                    .to_glib_none()
                    .0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn remove_widget(&self, cell_editable: &Self::Type) {
        unsafe {
            let type_ = ffi::gtk_cell_editable_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkCellEditableIface;
            assert!(!iface.is_null());

            ((*iface).remove_widget.as_ref().unwrap())(
                cell_editable
                    .unsafe_cast_ref::<CellEditable>()
                    .to_glib_none()
                    .0,
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn start_editing(&self, cell_editable: &Self::Type, event: Option<&gdk::Event>);
}

unsafe impl<T: CellEditableImpl> IsImplementable<T> for CellEditable {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let cell_editable_iface = &mut *(iface as *mut ffi::GtkCellEditableIface);

        cell_editable_iface.editing_done = Some(cell_editable_editing_done::<T>);
        cell_editable_iface.remove_widget = Some(cell_editable_remove_widget::<T>);
        cell_editable_iface.start_editing = Some(cell_editable_start_editing::<T>);
    }
}

unsafe extern "C" fn cell_editable_editing_done<T: CellEditableImpl>(
    cell_editable: *mut ffi::GtkCellEditable,
) {
    let instance = &*(cell_editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.editing_done(from_glib_borrow::<_, CellEditable>(cell_editable).unsafe_cast_ref())
}

unsafe extern "C" fn cell_editable_remove_widget<T: CellEditableImpl>(
    cell_editable: *mut ffi::GtkCellEditable,
) {
    let instance = &*(cell_editable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.remove_widget(from_glib_borrow::<_, CellEditable>(cell_editable).unsafe_cast_ref())
}

unsafe extern "C" fn cell_editable_start_editing<T: CellEditableImpl>(
    cell_editable: *mut ffi::GtkCellEditable,
    eventptr: *mut gdk::ffi::GdkEvent,
) {
    let instance = &*(cell_editable as *mut T::Instance);
    let imp = instance.get_impl();
    let event: Borrowed<Option<gdk::Event>> = from_glib_borrow(eventptr);
    imp.start_editing(
        from_glib_borrow::<_, CellEditable>(cell_editable).unsafe_cast_ref(),
        event.as_ref().as_ref(),
    )
}
