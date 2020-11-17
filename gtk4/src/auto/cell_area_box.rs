// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Buildable;
use CellArea;
use CellLayout;
use CellRenderer;
use Orientable;
use Orientation;

glib_wrapper! {
    pub struct CellAreaBox(Object<gtk_sys::GtkCellAreaBox>) @extends CellArea, @implements Buildable, CellLayout, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_cell_area_box_get_type(),
    }
}

impl CellAreaBox {
    pub fn new() -> CellAreaBox {
        assert_initialized_main_thread!();
        unsafe { CellArea::from_glib_none(gtk_sys::gtk_cell_area_box_new()).unsafe_cast() }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe { gtk_sys::gtk_cell_area_box_get_spacing(self.to_glib_none().0) }
    }

    pub fn pack_end<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        expand: bool,
        align: bool,
        fixed: bool,
    ) {
        unsafe {
            gtk_sys::gtk_cell_area_box_pack_end(
                self.to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                expand.to_glib(),
                align.to_glib(),
                fixed.to_glib(),
            );
        }
    }

    pub fn pack_start<P: IsA<CellRenderer>>(
        &self,
        renderer: &P,
        expand: bool,
        align: bool,
        fixed: bool,
    ) {
        unsafe {
            gtk_sys::gtk_cell_area_box_pack_start(
                self.to_glib_none().0,
                renderer.as_ref().to_glib_none().0,
                expand.to_glib(),
                align.to_glib(),
                fixed.to_glib(),
            );
        }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            gtk_sys::gtk_cell_area_box_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    pub fn connect_property_spacing_notify<F: Fn(&CellAreaBox) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_spacing_trampoline<F: Fn(&CellAreaBox) + 'static>(
            this: *mut gtk_sys::GtkCellAreaBox,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_spacing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for CellAreaBox {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct CellAreaBoxBuilder {
    spacing: Option<i32>,
    focus_cell: Option<CellRenderer>,
    orientation: Option<Orientation>,
}

impl CellAreaBoxBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> CellAreaBox {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref spacing) = self.spacing {
            properties.push(("spacing", spacing));
        }
        if let Some(ref focus_cell) = self.focus_cell {
            properties.push(("focus-cell", focus_cell));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        let ret = glib::Object::new(CellAreaBox::static_type(), &properties)
            .expect("object new")
            .downcast::<CellAreaBox>()
            .expect("downcast");
        ret
    }

    pub fn spacing(mut self, spacing: i32) -> Self {
        self.spacing = Some(spacing);
        self
    }

    pub fn focus_cell<P: IsA<CellRenderer>>(mut self, focus_cell: &P) -> Self {
        self.focus_cell = Some(focus_cell.clone().upcast());
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

impl fmt::Display for CellAreaBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellAreaBox")
    }
}