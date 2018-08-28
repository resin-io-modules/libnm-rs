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
use glib::StaticType;
use glib::Value;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;
use Error;
use VpnPluginFailure;
use VpnServiceState;

glib_wrapper! {
    pub struct VpnServicePlugin(Object<ffi::NMVpnServicePlugin, ffi::NMVpnServicePluginClass>);

    match fn {
        get_type => || ffi::nm_vpn_service_plugin_get_type(),
    }
}

impl VpnServicePlugin {
    //pub fn get_secret_flags(data: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 }, secret_name: &str) -> Option<SettingSecretFlags> {
    //    unsafe { TODO: call ffi::nm_vpn_service_plugin_get_secret_flags() }
    //}

    //pub fn read_vpn_details(fd: i32, out_data: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 }, out_secrets: /*Unknown conversion*//*Unimplemented*/HashTable TypeId { ns_id: 0, id: 25 }/TypeId { ns_id: 0, id: 25 }) -> bool {
    //    unsafe { TODO: call ffi::nm_vpn_service_plugin_read_vpn_details() }
    //}
}

pub trait VpnServicePluginExt {
    fn disconnect(&self) -> Result<(), Error>;

    fn failure(&self, reason: VpnPluginFailure);

    //fn get_connection(&self) -> /*Ignored*/Option<gio::DBusConnection>;

    fn set_config(&self, config: &glib::Variant);

    fn set_ip4_config(&self, ip4_config: &glib::Variant);

    fn set_ip6_config(&self, ip6_config: &glib::Variant);

    fn set_login_banner(&self, banner: &str);

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn shutdown(&self);

    fn get_property_service_name(&self) -> Option<String>;

    fn get_property_state(&self) -> VpnServiceState;

    fn set_property_state(&self, state: VpnServiceState);

    fn get_property_watch_peer(&self) -> bool;

    fn connect_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_failure<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_ip4_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_ip6_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_login_banner<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_quit<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_secrets_required<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_state_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_service_name_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_watch_peer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<VpnServicePlugin> + IsA<glib::object::Object>> VpnServicePluginExt for O {
    fn disconnect(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::nm_vpn_service_plugin_disconnect(self.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn failure(&self, reason: VpnPluginFailure) {
        unsafe {
            ffi::nm_vpn_service_plugin_failure(self.to_glib_none().0, reason.to_glib());
        }
    }

    //fn get_connection(&self) -> /*Ignored*/Option<gio::DBusConnection> {
    //    unsafe { TODO: call ffi::nm_vpn_service_plugin_get_connection() }
    //}

    fn set_config(&self, config: &glib::Variant) {
        unsafe {
            ffi::nm_vpn_service_plugin_set_config(self.to_glib_none().0, config.to_glib_none().0);
        }
    }

    fn set_ip4_config(&self, ip4_config: &glib::Variant) {
        unsafe {
            ffi::nm_vpn_service_plugin_set_ip4_config(
                self.to_glib_none().0,
                ip4_config.to_glib_none().0,
            );
        }
    }

    fn set_ip6_config(&self, ip6_config: &glib::Variant) {
        unsafe {
            ffi::nm_vpn_service_plugin_set_ip6_config(
                self.to_glib_none().0,
                ip6_config.to_glib_none().0,
            );
        }
    }

    fn set_login_banner(&self, banner: &str) {
        unsafe {
            ffi::nm_vpn_service_plugin_set_login_banner(
                self.to_glib_none().0,
                banner.to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v1_12", feature = "dox"))]
    fn shutdown(&self) {
        unsafe {
            ffi::nm_vpn_service_plugin_shutdown(self.to_glib_none().0);
        }
    }

    fn get_property_service_name(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "service-name".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn get_property_state(&self) -> VpnServiceState {
        unsafe {
            let mut value = Value::from_type(<VpnServiceState as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "state".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn set_property_state(&self, state: VpnServiceState) {
        unsafe {
            gobject_ffi::g_object_set_property(
                self.to_glib_none().0,
                "state".to_glib_none().0,
                Value::from(&state).to_glib_none().0,
            );
        }
    }

    fn get_property_watch_peer(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "watch-peer".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get().unwrap()
        }
    }

    fn connect_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &glib::Variant) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "config",
                transmute(config_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_failure<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "failure",
                transmute(failure_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_ip4_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &glib::Variant) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "ip4-config",
                transmute(ip4_config_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_ip6_config<F: Fn(&Self, &glib::Variant) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &glib::Variant) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "ip6-config",
                transmute(ip6_config_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_login_banner<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "login-banner",
                transmute(login_banner_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_quit<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "quit",
                transmute(quit_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    //fn connect_secrets_required<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype p0: *.CArray TypeId { ns_id: 0, id: 28 }
    //}

    fn connect_state_changed<F: Fn(&Self, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "state-changed",
                transmute(state_changed_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_service_name_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::service-name",
                transmute(notify_service_name_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::state",
                transmute(notify_state_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_watch_peer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::watch-peer",
                transmute(notify_watch_peer_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn config_trampoline<P>(
    this: *mut ffi::NMVpnServicePlugin,
    object: *mut glib_ffi::GVariant,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnServicePlugin>,
{
    let f: &&(Fn(&P, &glib::Variant) + 'static) = transmute(f);
    f(
        &VpnServicePlugin::from_glib_borrow(this).downcast_unchecked(),
        &from_glib_borrow(object),
    )
}

unsafe extern "C" fn failure_trampoline<P>(
    this: *mut ffi::NMVpnServicePlugin,
    object: libc::c_uint,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnServicePlugin>,
{
    let f: &&(Fn(&P, u32) + 'static) = transmute(f);
    f(
        &VpnServicePlugin::from_glib_borrow(this).downcast_unchecked(),
        object,
    )
}

unsafe extern "C" fn ip4_config_trampoline<P>(
    this: *mut ffi::NMVpnServicePlugin,
    object: *mut glib_ffi::GVariant,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnServicePlugin>,
{
    let f: &&(Fn(&P, &glib::Variant) + 'static) = transmute(f);
    f(
        &VpnServicePlugin::from_glib_borrow(this).downcast_unchecked(),
        &from_glib_borrow(object),
    )
}

unsafe extern "C" fn ip6_config_trampoline<P>(
    this: *mut ffi::NMVpnServicePlugin,
    object: *mut glib_ffi::GVariant,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnServicePlugin>,
{
    let f: &&(Fn(&P, &glib::Variant) + 'static) = transmute(f);
    f(
        &VpnServicePlugin::from_glib_borrow(this).downcast_unchecked(),
        &from_glib_borrow(object),
    )
}

unsafe extern "C" fn login_banner_trampoline<P>(
    this: *mut ffi::NMVpnServicePlugin,
    object: *mut libc::c_char,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnServicePlugin>,
{
    let f: &&(Fn(&P, &str) + 'static) = transmute(f);
    f(
        &VpnServicePlugin::from_glib_borrow(this).downcast_unchecked(),
        &String::from_glib_none(object),
    )
}

unsafe extern "C" fn quit_trampoline<P>(this: *mut ffi::NMVpnServicePlugin, f: glib_ffi::gpointer)
where
    P: IsA<VpnServicePlugin>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&VpnServicePlugin::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn state_changed_trampoline<P>(
    this: *mut ffi::NMVpnServicePlugin,
    object: libc::c_uint,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnServicePlugin>,
{
    let f: &&(Fn(&P, u32) + 'static) = transmute(f);
    f(
        &VpnServicePlugin::from_glib_borrow(this).downcast_unchecked(),
        object,
    )
}

unsafe extern "C" fn notify_service_name_trampoline<P>(
    this: *mut ffi::NMVpnServicePlugin,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnServicePlugin>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&VpnServicePlugin::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_state_trampoline<P>(
    this: *mut ffi::NMVpnServicePlugin,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnServicePlugin>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&VpnServicePlugin::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_watch_peer_trampoline<P>(
    this: *mut ffi::NMVpnServicePlugin,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<VpnServicePlugin>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&VpnServicePlugin::from_glib_borrow(this).downcast_unchecked())
}