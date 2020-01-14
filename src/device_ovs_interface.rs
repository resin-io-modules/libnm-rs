// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use nm_sys;
use std::fmt;
use Device;
use Object;

glib_wrapper! {
    pub struct DeviceOvsInterface(Object<nm_sys::NMDeviceOvsInterface, nm_sys::NMDeviceOvsInterfaceClass, DeviceOvsInterfaceClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_ovs_interface_get_type(),
    }
}

impl DeviceOvsInterface {}

impl fmt::Display for DeviceOvsInterface {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceOvsInterface")
    }
}