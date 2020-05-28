// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::GString;
use glib::Value;
use glib_sys;
use gobject_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Setting;

glib_wrapper! {
    pub struct SettingInfiniband(Object<nm_sys::NMSettingInfiniband, nm_sys::NMSettingInfinibandClass, SettingInfinibandClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_infiniband_get_type(),
    }
}

impl SettingInfiniband {
    pub fn new() -> SettingInfiniband {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_infiniband_new()).unsafe_cast() }
    }
}

impl Default for SettingInfiniband {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_INFINIBAND: Option<&SettingInfiniband> = None;

pub trait SettingInfinibandExt: 'static {
    fn get_mac_address(&self) -> Option<GString>;

    fn get_mtu(&self) -> u32;

    fn get_p_key(&self) -> i32;

    fn get_parent(&self) -> Option<GString>;

    fn get_transport_mode(&self) -> Option<GString>;

    fn get_virtual_interface_name(&self) -> Option<GString>;

    fn set_property_mac_address(&self, mac_address: Option<&str>);

    fn set_property_mtu(&self, mtu: u32);

    fn set_property_p_key(&self, p_key: i32);

    fn set_property_parent(&self, parent: Option<&str>);

    fn set_property_transport_mode(&self, transport_mode: Option<&str>);

    fn connect_property_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_p_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_transport_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<SettingInfiniband>> SettingInfinibandExt for O {
    fn get_mac_address(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_infiniband_get_mac_address(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_mtu(&self) -> u32 {
        unsafe { nm_sys::nm_setting_infiniband_get_mtu(self.as_ref().to_glib_none().0) }
    }

    fn get_p_key(&self) -> i32 {
        unsafe { nm_sys::nm_setting_infiniband_get_p_key(self.as_ref().to_glib_none().0) }
    }

    fn get_parent(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_infiniband_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_transport_mode(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_infiniband_get_transport_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_virtual_interface_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_infiniband_get_virtual_interface_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_property_mac_address(&self, mac_address: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mac-address\0".as_ptr() as *const _,
                Value::from(mac_address).to_glib_none().0,
            );
        }
    }

    fn set_property_mtu(&self, mtu: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"mtu\0".as_ptr() as *const _,
                Value::from(&mtu).to_glib_none().0,
            );
        }
    }

    fn set_property_p_key(&self, p_key: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"p-key\0".as_ptr() as *const _,
                Value::from(&p_key).to_glib_none().0,
            );
        }
    }

    fn set_property_parent(&self, parent: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"parent\0".as_ptr() as *const _,
                Value::from(parent).to_glib_none().0,
            );
        }
    }

    fn set_property_transport_mode(&self, transport_mode: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"transport-mode\0".as_ptr() as *const _,
                Value::from(transport_mode).to_glib_none().0,
            );
        }
    }

    fn connect_property_mac_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mac_address_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingInfiniband,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingInfiniband>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingInfiniband::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mac-address\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mac_address_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_mtu_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mtu_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingInfiniband,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingInfiniband>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingInfiniband::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mtu\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mtu_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_p_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_p_key_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingInfiniband,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingInfiniband>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingInfiniband::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::p-key\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_p_key_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingInfiniband,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingInfiniband>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingInfiniband::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_transport_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_transport_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingInfiniband,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingInfiniband>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingInfiniband::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::transport-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_transport_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingInfiniband {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingInfiniband")
    }
}
