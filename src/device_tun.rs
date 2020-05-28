// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::object::ObjectType as ObjectType_;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::signal::connect_raw;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib::GString;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use glib_sys;
use nm_sys;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use std::mem::transmute;
use Device;
use Object;

glib_wrapper! {
    pub struct DeviceTun(Object<nm_sys::NMDeviceTun, nm_sys::NMDeviceTunClass, DeviceTunClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_tun_get_type(),
    }
}

impl DeviceTun {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_group(&self) -> i64 {
        unsafe { nm_sys::nm_device_tun_get_group(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_mode(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_device_tun_get_mode(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_multi_queue(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_tun_get_multi_queue(self.to_glib_none().0)) }
    }

    pub fn get_no_pi(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_tun_get_no_pi(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_owner(&self) -> i64 {
        unsafe { nm_sys::nm_device_tun_get_owner(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_vnet_hdr(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_tun_get_vnet_hdr(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_group_notify<F: Fn(&DeviceTun) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_group_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut nm_sys::NMDeviceTun,
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
                b"notify::group\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_group_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_mode_notify<F: Fn(&DeviceTun) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut nm_sys::NMDeviceTun,
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
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_multi_queue_notify<F: Fn(&DeviceTun) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_multi_queue_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut nm_sys::NMDeviceTun,
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
                b"notify::multi-queue\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_multi_queue_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_no_pi_notify<F: Fn(&DeviceTun) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_no_pi_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut nm_sys::NMDeviceTun,
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
                b"notify::no-pi\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_no_pi_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_owner_notify<F: Fn(&DeviceTun) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_owner_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut nm_sys::NMDeviceTun,
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
                b"notify::owner\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_owner_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_vnet_hdr_notify<F: Fn(&DeviceTun) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_vnet_hdr_trampoline<F: Fn(&DeviceTun) + 'static>(
            this: *mut nm_sys::NMDeviceTun,
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
                b"notify::vnet-hdr\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_vnet_hdr_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceTun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceTun")
    }
}
