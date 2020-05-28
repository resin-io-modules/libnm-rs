// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Object;

glib_wrapper! {
    pub struct DhcpConfig(Object<nm_sys::NMDhcpConfig, nm_sys::NMDhcpConfigClass, DhcpConfigClass>) @extends Object;

    match fn {
        get_type => || nm_sys::nm_dhcp_config_get_type(),
    }
}

impl DhcpConfig {
    pub fn get_family(&self) -> i32 {
        unsafe { nm_sys::nm_dhcp_config_get_family(self.to_glib_none().0) }
    }

    pub fn get_one_option(&self, option: &str) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_dhcp_config_get_one_option(
                self.to_glib_none().0,
                option.to_glib_none().0,
            ))
        }
    }

    //pub fn get_options(&self) -> /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
    //    unsafe { TODO: call nm_sys:nm_dhcp_config_get_options() }
    //}

    pub fn connect_property_family_notify<F: Fn(&DhcpConfig) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_family_trampoline<F: Fn(&DhcpConfig) + 'static>(
            this: *mut nm_sys::NMDhcpConfig,
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
                b"notify::family\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_family_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_options_notify<F: Fn(&DhcpConfig) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_options_trampoline<F: Fn(&DhcpConfig) + 'static>(
            this: *mut nm_sys::NMDhcpConfig,
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
                b"notify::options\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_options_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DhcpConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DhcpConfig")
    }
}
