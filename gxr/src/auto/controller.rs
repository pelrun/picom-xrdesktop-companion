// Generated by gir (https://github.com/gtk-rs/gir @ e0d8d8d645b1)
// from ../gir-files (@ 54ae87ae2ece+)
// from ../xrd-gir-files (@ 8cffc8b155f9)
// DO NOT EDIT

use crate::Context;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GxrController")]
    pub struct Controller(Object<ffi::GxrController, ffi::GxrControllerClass>);

    match fn {
        type_ => || ffi::gxr_controller_get_type(),
    }
}

impl Controller {
    #[doc(alias = "gxr_controller_new")]
    pub fn new(controller_handle: u64, context: &impl IsA<Context>, model_name: &str) -> Controller {
        unsafe {
            from_glib_full(ffi::gxr_controller_new(controller_handle, context.as_ref().to_glib_none().0, model_name.to_glib_none().0))
        }
    }

    //#[doc(alias = "gxr_controller_get_user_data")]
    //#[doc(alias = "get_user_data")]
    //pub fn user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi:gxr_controller_get_user_data() }
    //}

    #[doc(alias = "gxr_controller_hide_pointer")]
    pub fn hide_pointer(&self) {
        unsafe {
            ffi::gxr_controller_hide_pointer(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gxr_controller_is_pointer_pose_valid")]
    pub fn is_pointer_pose_valid(&self) -> bool {
        unsafe {
            from_glib(ffi::gxr_controller_is_pointer_pose_valid(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gxr_controller_is_pointer_visible")]
    pub fn is_pointer_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gxr_controller_is_pointer_visible(self.to_glib_none().0))
        }
    }

    #[doc(alias = "gxr_controller_reset_grab_state")]
    pub fn reset_grab_state(&self) {
        unsafe {
            ffi::gxr_controller_reset_grab_state(self.to_glib_none().0);
        }
    }

    #[doc(alias = "gxr_controller_reset_hover_state")]
    pub fn reset_hover_state(&self) {
        unsafe {
            ffi::gxr_controller_reset_hover_state(self.to_glib_none().0);
        }
    }

    //#[doc(alias = "gxr_controller_set_pointer")]
    //pub fn set_pointer(&self, pointer: /*Ignored*/&Pointer) {
    //    unsafe { TODO: call ffi:gxr_controller_set_pointer() }
    //}

    //#[doc(alias = "gxr_controller_set_pointer_tip")]
    //pub fn set_pointer_tip(&self, tip: /*Ignored*/&PointerTip) {
    //    unsafe { TODO: call ffi:gxr_controller_set_pointer_tip() }
    //}

    //#[doc(alias = "gxr_controller_set_user_data")]
    //pub fn set_user_data(&self, data: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi:gxr_controller_set_user_data() }
    //}

    #[doc(alias = "gxr_controller_show_pointer")]
    pub fn show_pointer(&self) {
        unsafe {
            ffi::gxr_controller_show_pointer(self.to_glib_none().0);
        }
    }
}

unsafe impl Send for Controller {}
unsafe impl Sync for Controller {}

impl fmt::Display for Controller {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Controller")
    }
}