// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
#[cfg(any(feature = "v1_4", feature = "dox"))]
use glib::GString;
use glib::Value;
use glib_sys;
use gobject_sys;
use nm_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Setting;
#[cfg(any(feature = "v1_2", feature = "dox"))]
use SettingIP6ConfigAddrGenMode;
use SettingIP6ConfigPrivacy;
use SettingIPConfig;

glib_wrapper! {
    pub struct SettingIP6Config(Object<nm_sys::NMSettingIP6Config, nm_sys::NMSettingIP6ConfigClass, SettingIP6ConfigClass>) @extends SettingIPConfig, Setting;

    match fn {
        get_type => || nm_sys::nm_setting_ip6_config_get_type(),
    }
}

impl SettingIP6Config {
    pub fn new() -> SettingIP6Config {
        unsafe { Setting::from_glib_full(nm_sys::nm_setting_ip6_config_new()).unsafe_cast() }
    }
}

impl Default for SettingIP6Config {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SETTING_IP6_CONFIG: Option<&SettingIP6Config> = None;

pub trait SettingIP6ConfigExt: 'static {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_addr_gen_mode(&self) -> SettingIP6ConfigAddrGenMode;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_dhcp_duid(&self) -> Option<GString>;

    fn get_ip6_privacy(&self) -> SettingIP6ConfigPrivacy;

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    fn get_ra_timeout(&self) -> i32;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_token(&self) -> Option<GString>;

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_addr_gen_mode(&self, addr_gen_mode: i32);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_dhcp_duid(&self, dhcp_duid: Option<&str>);

    fn set_property_ip6_privacy(&self, ip6_privacy: SettingIP6ConfigPrivacy);

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    fn set_property_ra_timeout(&self, ra_timeout: i32);

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn set_property_token(&self, token: Option<&str>);

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_addr_gen_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_dhcp_duid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ip6_privacy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    fn connect_property_ra_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn connect_property_token_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SettingIP6Config>> SettingIP6ConfigExt for O {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn get_addr_gen_mode(&self) -> SettingIP6ConfigAddrGenMode {
        unsafe {
            from_glib(nm_sys::nm_setting_ip6_config_get_addr_gen_mode(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn get_dhcp_duid(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_ip6_config_get_dhcp_duid(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_ip6_privacy(&self) -> SettingIP6ConfigPrivacy {
        unsafe {
            from_glib(nm_sys::nm_setting_ip6_config_get_ip6_privacy(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    fn get_ra_timeout(&self) -> i32 {
        unsafe { nm_sys::nm_setting_ip6_config_get_ra_timeout(self.as_ref().to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn get_token(&self) -> Option<GString> {
        unsafe {
            from_glib_none(nm_sys::nm_setting_ip6_config_get_token(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn set_property_addr_gen_mode(&self, addr_gen_mode: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"addr-gen-mode\0".as_ptr() as *const _,
                Value::from(&addr_gen_mode).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn set_property_dhcp_duid(&self, dhcp_duid: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"dhcp-duid\0".as_ptr() as *const _,
                Value::from(dhcp_duid).to_glib_none().0,
            );
        }
    }

    fn set_property_ip6_privacy(&self, ip6_privacy: SettingIP6ConfigPrivacy) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"ip6-privacy\0".as_ptr() as *const _,
                Value::from(&ip6_privacy).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    fn set_property_ra_timeout(&self, ra_timeout: i32) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"ra-timeout\0".as_ptr() as *const _,
                Value::from(&ra_timeout).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn set_property_token(&self, token: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"token\0".as_ptr() as *const _,
                Value::from(token).to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    fn connect_property_addr_gen_mode_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_addr_gen_mode_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingIP6Config,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingIP6Config>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingIP6Config::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::addr-gen-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_addr_gen_mode_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn connect_property_dhcp_duid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dhcp_duid_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingIP6Config,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingIP6Config>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingIP6Config::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dhcp-duid\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dhcp_duid_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_ip6_privacy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ip6_privacy_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingIP6Config,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingIP6Config>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingIP6Config::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ip6-privacy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ip6_privacy_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_24", feature = "dox"))]
    fn connect_property_ra_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ra_timeout_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingIP6Config,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingIP6Config>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingIP6Config::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ra-timeout\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ra_timeout_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_4", feature = "dox"))]
    fn connect_property_token_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_token_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut nm_sys::NMSettingIP6Config,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<SettingIP6Config>,
        {
            let f: &F = &*(f as *const F);
            f(&SettingIP6Config::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::token\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_token_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for SettingIP6Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SettingIP6Config")
    }
}
