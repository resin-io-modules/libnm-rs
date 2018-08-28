// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::Value;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Setting;

glib_wrapper! {
    pub struct SettingPppoe(Object<ffi::NMSettingPppoe, ffi::NMSettingPppoeClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_pppoe_get_type(),
    }
}

impl SettingPppoe {
    pub fn new() -> SettingPppoe {
        unsafe { Setting::from_glib_full(ffi::nm_setting_pppoe_new()).downcast_unchecked() }
    }
}

impl Default for SettingPppoe {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SettingPppoeExt {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_parent(&self) -> Option<String>;

    fn get_password(&self) -> Option<String>;

    //fn get_password_flags(&self) -> /*Ignored*/SettingSecretFlags;

    fn get_service(&self) -> Option<String>;

    fn get_username(&self) -> Option<String>;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_parent(&self, parent: Option<&str>);

    fn set_property_password(&self, password: Option<&str>);

    //fn set_property_password_flags(&self, password_flags: /*Ignored*/SettingSecretFlags);

    fn set_property_service(&self, service: Option<&str>);

    fn set_property_username(&self, username: Option<&str>);

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_password_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingPppoe> + IsA<glib::object::Object>> SettingPppoeExt for O {
    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn get_parent(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_pppoe_get_parent(self.to_glib_none().0)) }
    }

    fn get_password(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_pppoe_get_password(self.to_glib_none().0)) }
    }

    //fn get_password_flags(&self) -> /*Ignored*/SettingSecretFlags {
    //    unsafe { TODO: call ffi::nm_setting_pppoe_get_password_flags() }
    //}

    fn get_service(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_pppoe_get_service(self.to_glib_none().0)) }
    }

    fn get_username(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_setting_pppoe_get_username(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn set_property_parent(&self, parent: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "parent".to_glib_none().0,
                Value::from(parent).to_glib_none().0,
            );
        }
    }

    fn set_property_password(&self, password: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "password".to_glib_none().0,
                Value::from(password).to_glib_none().0,
            );
        }
    }

    //fn set_property_password_flags(&self, password_flags: /*Ignored*/SettingSecretFlags) {
    //    unsafe {
    //        gobject_ffi::g_object_set_property(self.to_glib_none().0, "password-flags".to_glib_none().0, Value::from(&password_flags).to_glib_none().0);
    //    }
    //}

    fn set_property_service(&self, service: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "service".to_glib_none().0,
                Value::from(service).to_glib_none().0,
            );
        }
    }

    fn set_property_username(&self, username: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "username".to_glib_none().0,
                Value::from(username).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::parent",
                transmute(notify_parent_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::password",
                transmute(notify_password_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_password_flags_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::password-flags",
                transmute(notify_password_flags_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::service",
                transmute(notify_service_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::username",
                transmute(notify_username_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_parent_trampoline<P>(
    this: *mut ffi::NMSettingPppoe,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPppoe>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPppoe::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_password_trampoline<P>(
    this: *mut ffi::NMSettingPppoe,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPppoe>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPppoe::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_password_flags_trampoline<P>(
    this: *mut ffi::NMSettingPppoe,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPppoe>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPppoe::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_service_trampoline<P>(
    this: *mut ffi::NMSettingPppoe,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPppoe>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPppoe::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_username_trampoline<P>(
    this: *mut ffi::NMSettingPppoe,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<SettingPppoe>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SettingPppoe::from_glib_borrow(this).downcast_unchecked())
}