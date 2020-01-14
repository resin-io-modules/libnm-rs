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
#[cfg(any(feature = "v1_12", feature = "dox"))]
use std::mem;
use std::mem::transmute;
use Setting;

glib_wrapper! {
    pub struct SettingVpn(Object<nm_sys::NMSettingVpn, nm_sys::NMSettingVpnClass, SettingVpnClass>) @extends Setting;

    match fn {
        get_type => || nm_sys::nm_setting_vpn_get_type(),
    }
}

impl SettingVpn {
    pub fn new() -> SettingVpn {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_vpn_new()).unsafe_cast() }
    }
}

impl Default for SettingVpn {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_VPN: Option<&SettingVpn> = None;

pub trait SettingVpnExt: 'static {
    fn add_data_item(&self, key: &str, item: &str);

    fn add_secret(&self, key: &str, secret: &str);

    fn foreach_data_item<P: FnMut(&str, &str)>(&self, func: P);

    fn foreach_secret<P: FnMut(&str, &str)>(&self, func: P);

    fn get_data_item(&self, key: &str) -> Option<GString>;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_data_keys(&self) -> Vec<GString>;

    fn get_num_data_items(&self) -> u32;

    fn get_num_secrets(&self) -> u32;

    fn get_persistent(&self) -> bool;

    fn get_secret(&self, key: &str) -> Option<GString>;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_secret_keys(&self) -> Vec<GString>;

    fn get_service_type(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_timeout(&self) -> u32;

    fn get_user_name(&self) -> Option<GString>;

    fn remove_data_item(&self, key: &str) -> bool;

    fn remove_secret(&self, key: &str) -> bool;

    //fn get_property_data(&self) -> /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 };

    //fn set_property_data(&self, data: /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 });

    fn set_property_persistent(&self, persistent: bool);

    //fn get_property_secrets(&self) -> /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 };

    //fn set_property_secrets(&self, secrets: /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 });

    fn set_property_service_type(&self, service_type: Option<&str>);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_timeout(&self, timeout: u32);

    fn set_property_user_name(&self, user_name: Option<&str>);

    fn connect_property_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_persistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_secrets_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_service_type_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_user_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingVpn>> SettingVpnExt for O {
    fn add_data_item(&self, key: &str, item: &str) {
        unsafe {
            nm_sys::nm_setting_vpn_add_data_item(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                item.to_glib_none().0,
            );
        }
    }

    fn add_secret(&self, key: &str, secret: &str) {
        unsafe {
            nm_sys::nm_setting_vpn_add_secret(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
                secret.to_glib_none().0,
            );
        }
    }

    fn foreach_data_item<P: FnMut(&str, &str)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&str, &str)>(
            key: *const libc::c_char,
            value: *const libc::c_char,
            user_data: glib_sys::gpointer,
        ) {
            let key: GString = from_glib_borrow(key);
            let value: GString = from_glib_borrow(value);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(key.as_str(), value.as_str());
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            nm_sys::nm_setting_vpn_foreach_data_item(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn foreach_secret<P: FnMut(&str, &str)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&str, &str)>(
            key: *const libc::c_char,
            value: *const libc::c_char,
            user_data: glib_sys::gpointer,
        ) {
            let key: GString = from_glib_borrow(key);
            let value: GString = from_glib_borrow(value);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(key.as_str(), value.as_str());
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            nm_sys::nm_setting_vpn_foreach_secret(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    fn get_data_item(&self, key: &str) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_vpn_get_data_item(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_data_keys(&self) -> Vec<GString> {
        unsafe {
            let mut out_length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(
                nm_sys::nm_setting_vpn_get_data_keys(
                    self.as_ref().to_glib_none().0,
                    out_length.as_mut_ptr(),
                ),
                out_length.assume_init() as usize,
            );
            ret
        }
    }

    fn get_num_data_items(&self) -> u32 {
        unsafe { nm_sys::nm_setting_vpn_get_num_data_items(self.as_ref().to_glib_none().0) }
    }

    fn get_num_secrets(&self) -> u32 {
        unsafe { nm_sys::nm_setting_vpn_get_num_secrets(self.as_ref().to_glib_none().0) }
    }

    fn get_persistent(&self) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_vpn_get_persistent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_secret(&self, key: &str) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_vpn_get_secret(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_secret_keys(&self) -> Vec<GString> {
        unsafe {
            let mut out_length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_container_num(
                nm_sys::nm_setting_vpn_get_secret_keys(
                    self.as_ref().to_glib_none().0,
                    out_length.as_mut_ptr(),
                ),
                out_length.assume_init() as usize,
            );
            ret
        }
    }

    fn get_service_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_vpn_get_service_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_timeout(&self) -> u32 {
        unsafe { nm_sys::nm_setting_vpn_get_timeout(self.as_ref().to_glib_none().0) }
    }

    fn get_user_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_vpn_get_user_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_data_item(&self, key: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_vpn_remove_data_item(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    fn remove_secret(&self, key: &str) -> bool {
        unsafe {
            from_glib(nm_sys::nm_setting_vpn_remove_secret(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    //fn get_property_data(&self) -> /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"data\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `data` getter").unwrap()
    //    }
    //}

    //fn set_property_data(&self, data: /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"data\0".as_ptr() as *const _, Value::from(&data).to_glib_none().0);
    //    }
    //}

    fn set_property_persistent(&self, persistent: bool) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"persistent\0".as_ptr() as *const _,
                Value::from(&persistent).to_glib_none().0,
            );
        }
    }

    //fn get_property_secrets(&self) -> /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 } {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"secrets\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().expect("Return Value for property `secrets` getter").unwrap()
    //    }
    //}

    //fn set_property_secrets(&self, secrets: /*Unimplemented*/HashTable TypeId { ns_id: 0, id: 28 }/TypeId { ns_id: 0, id: 28 }) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"secrets\0".as_ptr() as *const _, Value::from(&secrets).to_glib_none().0);
    //    }
    //}

    fn set_property_service_type(&self, service_type: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"service-type\0".as_ptr() as *const _,
                Value::from(service_type).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_timeout(&self, timeout: u32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"timeout\0".as_ptr() as *const _,
                Value::from(&timeout).to_glib_none().0,
            );
        }
    }

    fn set_property_user_name(&self, user_name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"user-name\0".as_ptr() as *const _,
                Value::from(user_name).to_glib_none().0,
            );
        }
    }

    fn connect_property_data_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_data_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingVpn,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingVpn>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingVpn::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::data\0".as_ptr() as *const _,
                Some(transmute(notify_data_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_persistent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_persistent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingVpn,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingVpn>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingVpn::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::persistent\0".as_ptr() as *const _,
                Some(transmute(notify_persistent_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_secrets_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_secrets_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingVpn,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingVpn>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingVpn::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::secrets\0".as_ptr() as *const _,
                Some(transmute(notify_secrets_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_service_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_service_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingVpn,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingVpn>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingVpn::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::service-type\0".as_ptr() as *const _,
                Some(transmute(
                    notify_service_type_trampoline::<Self, F> as usize,
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_timeout_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingVpn,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingVpn>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingVpn::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::timeout\0".as_ptr() as *const _,
                Some(transmute(notify_timeout_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_user_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_user_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingVpn,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingVpn>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingVpn::from_glib_borrow(this).unsafe_cast())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::user-name\0".as_ptr() as *const _,
                Some(transmute(notify_user_name_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingVpn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingVpn")
    }
}