// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Device;
use Object;

glib_wrapper! {
    pub struct DeviceInfiniband(Object<nm_sys::NMDeviceInfiniband, nm_sys::NMDeviceInfinibandClass, DeviceInfinibandClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_infiniband_get_type(),
    }
}

impl DeviceInfiniband {
    pub fn get_carrier(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_device_infiniband_get_carrier(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn connect_property_carrier_notify<F: Fn(&DeviceInfiniband) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_carrier_trampoline<F: Fn(&DeviceInfiniband) + 'static>(
            this: *mut nm_sys::NMDeviceInfiniband,
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
                b"notify::carrier\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_carrier_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceInfiniband {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceInfiniband")
    }
}
