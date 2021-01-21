// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{TreeIter, TreeModel, TreeModelFlags, TreePath};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::{Cast, Value};

/// # Safety:
///
/// The TreeModel trait is unsafe because it expect you to create
/// a TreeIter, creating such objects along with the optional
/// `ref_node` & `unref_node` functions cannot be done with safe Rust
pub unsafe trait TreeModelImpl: ObjectImpl {
    fn row_changed(&self, tree_model: &Self::Type, path: &TreePath, iter: &TreeIter) {
        unsafe {
            let type_ = ffi::gtk_tree_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkTreeModelIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).row_changed.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    path.to_glib_none().0 as *mut _,
                    iter.to_glib_none().0 as *mut _,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn row_inserted(&self, tree_model: &Self::Type, path: &TreePath, iter: &TreeIter) {
        unsafe {
            let type_ = ffi::gtk_tree_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkTreeModelIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).row_inserted.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    path.to_glib_none().0 as *mut _,
                    iter.to_glib_none().0 as *mut _,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn row_has_child_toggled(&self, tree_model: &Self::Type, path: &TreePath, iter: &TreeIter) {
        unsafe {
            let type_ = ffi::gtk_tree_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkTreeModelIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).row_has_child_toggled.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    path.to_glib_none().0 as *mut _,
                    iter.to_glib_none().0 as *mut _,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn row_deleted(&self, tree_model: &Self::Type, path: &TreePath) {
        unsafe {
            let type_ = ffi::gtk_tree_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkTreeModelIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).row_deleted.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    path.to_glib_none().0 as *mut _,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn get_flags(&self, tree_model: &Self::Type) -> TreeModelFlags {
        unsafe {
            let type_ = ffi::gtk_tree_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkTreeModelIface;
            assert!(!iface.is_null());

            let f = (*iface).get_flags.as_ref().unwrap();
            let ret = f(tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0);

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            from_glib(ret)
        }
    }

    fn get_n_columns(&self, tree_model: &Self::Type) -> i32;
    fn get_column_type(&self, tree_model: &Self::Type, index: i32) -> glib::Type;
    fn get_iter(&self, tree_model: &Self::Type, path: &TreePath) -> Option<TreeIter>;
    fn get_path(&self, tree_model: &Self::Type, iter: &TreeIter) -> TreePath;
    fn get_value(&self, tree_model: &Self::Type, iter: &TreeIter, index: i32) -> Value;
    fn iter_next(&self, tree_model: &Self::Type) -> Option<TreeIter>;

    fn iter_previous(&self, tree_model: &Self::Type) -> Option<TreeIter> {
        unsafe {
            let type_ = ffi::gtk_tree_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkTreeModelIface;
            assert!(!iface.is_null());
            let iter = std::ptr::null_mut();
            let f = (*iface).iter_previous.as_ref().unwrap();
            let ret: bool = from_glib(f(
                tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                iter,
            ));

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
            if ret {
                let iter = from_glib_none(iter);
                Some(iter)
            } else {
                None
            }
        }
    }

    fn iter_has_child(&self, tree_model: &Self::Type, iter: &TreeIter) -> bool;
    fn iter_n_children(&self, tree_model: &Self::Type, iter: Option<&TreeIter>) -> i32;
    fn iter_nth_child(
        &self,
        tree_model: &Self::Type,
        parent: Option<&TreeIter>,
        index: i32,
    ) -> Option<TreeIter>;
    fn iter_parent(&self, tree_model: &Self::Type, child: &TreeIter) -> Option<TreeIter>;

    fn ref_node(&self, tree_model: &Self::Type, iter: &TreeIter) {
        unsafe {
            let type_ = ffi::gtk_tree_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkTreeModelIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).ref_node.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    iter.to_glib_none().0 as *mut _,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }

    fn unref_node(&self, tree_model: &Self::Type, iter: &TreeIter) {
        unsafe {
            let type_ = ffi::gtk_tree_model_get_type();
            let iface = glib::gobject_ffi::g_type_default_interface_ref(type_)
                as *mut ffi::GtkTreeModelIface;
            assert!(!iface.is_null());

            if let Some(f) = (*iface).unref_node.as_ref() {
                f(
                    tree_model.unsafe_cast_ref::<TreeModel>().to_glib_none().0,
                    iter.to_glib_none().0 as *mut _,
                );
            }

            glib::gobject_ffi::g_type_default_interface_unref(iface as glib::ffi::gpointer);
        }
    }
}

unsafe impl<T: TreeModelImpl> IsImplementable<T> for TreeModel {
    unsafe extern "C" fn interface_init(
        iface: glib::ffi::gpointer,
        _iface_data: glib::ffi::gpointer,
    ) {
        let tree_model_iface = &mut *(iface as *mut ffi::GtkTreeModelIface);

        tree_model_iface.row_changed = Some(tree_model_row_changed::<T>);
        tree_model_iface.row_inserted = Some(tree_model_row_inserted::<T>);
        tree_model_iface.row_has_child_toggled = Some(tree_model_row_has_child_toggled::<T>);
        tree_model_iface.row_deleted = Some(tree_model_row_deleted::<T>);
        tree_model_iface.get_flags = Some(tree_model_get_flags::<T>);
        tree_model_iface.get_n_columns = Some(tree_model_get_n_columns::<T>);
        tree_model_iface.get_column_type = Some(tree_model_get_column_type::<T>);
        tree_model_iface.get_iter = Some(tree_model_get_iter::<T>);
        tree_model_iface.get_path = Some(tree_model_get_path::<T>);
        tree_model_iface.get_value = Some(tree_model_get_value::<T>);
        tree_model_iface.iter_next = Some(tree_model_iter_next::<T>);
        tree_model_iface.iter_previous = Some(tree_model_iter_previous::<T>);
        tree_model_iface.iter_has_child = Some(tree_model_iter_has_child::<T>);
        tree_model_iface.iter_n_children = Some(tree_model_iter_n_children::<T>);
        tree_model_iface.iter_nth_child = Some(tree_model_iter_nth_child::<T>);
        tree_model_iface.iter_parent = Some(tree_model_iter_parent::<T>);
        tree_model_iface.ref_node = Some(tree_model_ref_node::<T>);
        tree_model_iface.unref_node = Some(tree_model_unref_node::<T>);
    }
}

unsafe extern "C" fn tree_model_row_changed<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    treeptr: *mut ffi::GtkTreePath,
    iterptr: *mut ffi::GtkTreeIter,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let tree: Borrowed<TreePath> = from_glib_borrow(treeptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.row_changed(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &tree,
        &iter,
    )
}

unsafe extern "C" fn tree_model_row_inserted<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    treeptr: *mut ffi::GtkTreePath,
    iterptr: *mut ffi::GtkTreeIter,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let tree: Borrowed<TreePath> = from_glib_borrow(treeptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.row_inserted(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &tree,
        &iter,
    )
}

unsafe extern "C" fn tree_model_row_has_child_toggled<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    treeptr: *mut ffi::GtkTreePath,
    iterptr: *mut ffi::GtkTreeIter,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let tree: Borrowed<TreePath> = from_glib_borrow(treeptr);
    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.row_has_child_toggled(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &tree,
        &iter,
    )
}

unsafe extern "C" fn tree_model_row_deleted<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    treeptr: *mut ffi::GtkTreePath,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let tree: Borrowed<TreePath> = from_glib_borrow(treeptr);

    imp.row_deleted(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &tree,
    )
}

unsafe extern "C" fn tree_model_get_flags<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
) -> ffi::GtkTreeModelFlags {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_flags(from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref())
        .to_glib()
}

unsafe extern "C" fn tree_model_get_n_columns<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
) -> i32 {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_n_columns(from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref())
}

unsafe extern "C" fn tree_model_get_column_type<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    index: i32,
) -> glib::ffi::GType {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    imp.get_column_type(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        index,
    )
    .to_glib()
}

unsafe extern "C" fn tree_model_get_iter<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
    pathptr: *mut ffi::GtkTreePath,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let path: Borrowed<TreePath> = from_glib_borrow(pathptr);

    let ret = imp.get_iter(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &path,
    );
    if let Some(iter) = ret {
        *iterptr = *iter.to_glib_none().0;
        true.to_glib()
    } else {
        false.to_glib()
    }
}

unsafe extern "C" fn tree_model_get_path<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> *mut ffi::GtkTreePath {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.get_path(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &iter,
    )
    .to_glib_full() as *mut _
}

unsafe extern "C" fn tree_model_get_value<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
    index: i32,
    valueptr: *mut glib::gobject_ffi::GValue,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    let ret = imp.get_value(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &iter,
        index,
    );
    let ret = std::mem::ManuallyDrop::new(ret);
    *valueptr = *ret.to_glib_none().0;
}

unsafe extern "C" fn tree_model_iter_next<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let ret = imp.iter_next(from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref());
    if let Some(iter) = ret {
        *iterptr = *iter.to_glib_none().0;
        true.to_glib()
    } else {
        *iterptr = *std::ptr::null_mut();
        false.to_glib()
    }
}

unsafe extern "C" fn tree_model_iter_previous<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let ret = imp.iter_previous(from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref());
    if let Some(iter) = ret {
        *iterptr = *iter.to_glib_none().0;
        true.to_glib()
    } else {
        *iterptr = *std::ptr::null_mut();
        false.to_glib()
    }
}

unsafe extern "C" fn tree_model_iter_has_child<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);
    imp.iter_has_child(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &iter,
    )
    .to_glib()
}

unsafe extern "C" fn tree_model_iter_n_children<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) -> i32 {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let iter: Borrowed<Option<TreeIter>> = from_glib_borrow(iterptr);

    imp.iter_n_children(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        iter.as_ref().as_ref(),
    )
}

unsafe extern "C" fn tree_model_iter_nth_child<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    parent_iterptr: *mut ffi::GtkTreeIter,
    child_iterptr: *mut ffi::GtkTreeIter,
    index: i32,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let parent: Borrowed<Option<TreeIter>> = from_glib_borrow(parent_iterptr);

    let ret = imp.iter_nth_child(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        parent.as_ref().as_ref(),
        index,
    );
    if let Some(child_iter) = ret {
        *child_iterptr = *child_iter.to_glib_none().0;
        true.to_glib()
    } else {
        *child_iterptr = *std::ptr::null_mut();
        false.to_glib()
    }
}

unsafe extern "C" fn tree_model_iter_parent<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    parent_iterptr: *mut ffi::GtkTreeIter,
    child_iterptr: *mut ffi::GtkTreeIter,
) -> glib::ffi::gboolean {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let child: Borrowed<TreeIter> = from_glib_borrow(child_iterptr);

    let ret = imp.iter_parent(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &child,
    );
    if let Some(parent_iter) = ret {
        *parent_iterptr = *parent_iter.to_glib_none().0;
        true.to_glib()
    } else {
        *parent_iterptr = *std::ptr::null_mut();
        false.to_glib()
    }
}

unsafe extern "C" fn tree_model_ref_node<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.ref_node(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &iter,
    )
}

unsafe extern "C" fn tree_model_unref_node<T: TreeModelImpl>(
    tree_model: *mut ffi::GtkTreeModel,
    iterptr: *mut ffi::GtkTreeIter,
) {
    let instance = &*(tree_model as *mut T::Instance);
    let imp = instance.get_impl();

    let iter: Borrowed<TreeIter> = from_glib_borrow(iterptr);

    imp.unref_node(
        from_glib_borrow::<_, TreeModel>(tree_model).unsafe_cast_ref(),
        &iter,
    )
}
