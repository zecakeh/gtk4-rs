// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{SortType, TreeSortable};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

#[derive(Debug)]
pub struct TreeIterCompareCallback {
    compare_func: ffi::GtkTreeIterCompareFunc,
    user_data: glib::ffi::gpointer,
    destroy_ptr: glib::ffi::GDestroyNotify,
}

pub trait TreeSortableImpl: ObjectImpl {
    fn sort_column_changed(&self, tree_sortable: &Self::Type) {
        unsafe {
            let type_ = ffi::gtk_tree_sortable_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkTreeSortableIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).sort_column_changed.as_ref() {
                f(tree_sortable
                    .unsafe_cast_ref::<TreeSortable>()
                    .to_glib_none()
                    .0);
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn get_sort_column_id(&self, tree_sortable: &Self::Type) -> Option<(i32, SortType)> {
        unsafe {
            let type_ = ffi::gtk_tree_sortable_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkTreeSortableIface;
            assert!(!iface.is_null());

            let f = (*iface).get_sort_column_id.as_ref().unwrap();
            let sort_column_id = std::ptr::null_mut();
            let sort_type = std::ptr::null_mut();
            let ret = from_glib(f(
                tree_sortable
                    .unsafe_cast_ref::<TreeSortable>()
                    .to_glib_none()
                    .0,
                sort_column_id,
                sort_type,
            ));

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            if ret {
                Some((sort_column_id as i32, from_glib(sort_type as i32)))
            } else {
                None
            }
        }
    }

    fn set_sort_column_id(
        &self,
        tree_sortable: &Self::Type,
        sort_column_id: i32,
        sort_type: SortType,
    ) {
        unsafe {
            let type_ = ffi::gtk_tree_sortable_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkTreeSortableIface;
            assert!(!iface.is_null());

            let f = (*iface).set_sort_column_id.as_ref().unwrap();

            f(
                tree_sortable
                    .unsafe_cast_ref::<TreeSortable>()
                    .to_glib_none()
                    .0,
                sort_column_id,
                sort_type.to_glib(),
            );

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn has_default_sort_func(&self, tree_sortable: &Self::Type) -> bool;
    fn set_sort_func(
        &self,
        tree_sortable: &Self::Type,
        sort_column_id: i32,
        filter_func: Option<&TreeIterCompareCallback>,
    );
    fn set_default_sort_func(
        &self,
        tree_sortable: &Self::Type,
        filter_func: Option<&TreeIterCompareCallback>,
    );
}

unsafe impl<T: TreeSortableImpl> IsImplementable<T> for TreeSortable {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let tree_sortable_iface = &mut *(iface as *mut ffi::GtkTreeSortableIface);

        tree_sortable_iface.sort_column_changed = Some(tree_sortable_sort_column_changed::<T>);
        tree_sortable_iface.get_sort_column_id = Some(tree_sortable_get_sort_column_id::<T>);
        tree_sortable_iface.set_sort_column_id = Some(tree_sortable_set_sort_column_id::<T>);
        tree_sortable_iface.has_default_sort_func = Some(tree_sortable_has_default_sort_func::<T>);
        tree_sortable_iface.set_sort_func = Some(tree_sortable_set_sort_func::<T>);
        tree_sortable_iface.set_default_sort_func = Some(tree_sortable_set_default_sort_func::<T>);
    }
}

unsafe extern "C" fn tree_sortable_sort_column_changed<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
) {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.sort_column_changed(from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref())
}

unsafe extern "C" fn tree_sortable_get_sort_column_id<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
    sort_column_idptr: *mut libc::c_int,
    sort_typeptr: *mut ffi::GtkSortType,
) -> glib::ffi::gboolean {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.get_impl();

    let ret = imp
        .get_sort_column_id(from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref());

    if let Some((sort_column_id, sort_type)) = ret {
        *sort_column_idptr = sort_column_id;
        *sort_typeptr = sort_type.to_glib();
        true.to_glib()
    } else {
        *sort_column_idptr = *std::ptr::null_mut();
        *sort_typeptr = *std::ptr::null_mut();
        false.to_glib()
    }
}

unsafe extern "C" fn tree_sortable_set_sort_column_id<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
    sort_column_id: i32,
    sort_typeptr: ffi::GtkSortType,
) {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.set_sort_column_id(
        from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref(),
        sort_column_id,
        from_glib(sort_typeptr),
    );
}

unsafe extern "C" fn tree_sortable_has_default_sort_func<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
) -> glib::ffi::gboolean {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.get_impl();

    imp.has_default_sort_func(from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref())
        .to_glib()
}

unsafe extern "C" fn tree_sortable_set_sort_func<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
    sort_column_id: i32,
    compare_func: ffi::GtkTreeIterCompareFunc,
    user_data: glib::ffi::gpointer,
    destroy_ptr: glib::ffi::GDestroyNotify,
) {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.get_impl();

    let callback = if compare_func.is_some() {
        None
    } else {
        Some(TreeIterCompareCallback {
            compare_func,
            user_data,
            destroy_ptr,
        })
    };

    imp.set_sort_func(
        from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref(),
        sort_column_id,
        callback.as_ref(),
    );
}

unsafe extern "C" fn tree_sortable_set_default_sort_func<T: TreeSortableImpl>(
    tree_sortable: *mut ffi::GtkTreeSortable,
    compare_func: ffi::GtkTreeIterCompareFunc,
    user_data: glib::ffi::gpointer,
    destroy_ptr: glib::ffi::GDestroyNotify,
) {
    let instance = &*(tree_sortable as *mut T::Instance);
    let imp = instance.get_impl();

    let callback = if compare_func.is_some() {
        None
    } else {
        Some(TreeIterCompareCallback {
            compare_func,
            user_data,
            destroy_ptr,
        })
    };
    imp.set_default_sort_func(
        from_glib_borrow::<_, TreeSortable>(tree_sortable).unsafe_cast_ref(),
        callback.as_ref(),
    );
}
