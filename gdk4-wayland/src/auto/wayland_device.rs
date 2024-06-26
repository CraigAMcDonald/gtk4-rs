// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::ffi;
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GdkWaylandDevice")]
    pub struct WaylandDevice(Object<ffi::GdkWaylandDevice, ffi::GdkWaylandDeviceClass>) @extends gdk::Device;

    match fn {
        type_ => || ffi::gdk_wayland_device_get_type(),
    }
}

impl WaylandDevice {
    #[doc(alias = "gdk_wayland_device_get_node_path")]
    #[doc(alias = "get_node_path")]
    pub fn node_path(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gdk_wayland_device_get_node_path(self.to_glib_none().0)) }
    }
}
