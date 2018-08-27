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
use Device;
use RemoteConnection;

glib_wrapper! {
    pub struct ActiveConnection(Object<ffi::NMActiveConnection, ffi::NMActiveConnectionClass>);

    match fn {
        get_type => || ffi::nm_active_connection_get_type(),
    }
}

pub trait ActiveConnectionExt {
    fn get_connection(&self) -> Option<RemoteConnection>;

    fn get_connection_type(&self) -> Option<String>;

    fn get_default(&self) -> bool;

    fn get_default6(&self) -> bool;

    //fn get_devices(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 11 };

    //fn get_dhcp4_config(&self) -> /*Ignored*/Option<DhcpConfig>;

    //fn get_dhcp6_config(&self) -> /*Ignored*/Option<DhcpConfig>;

    fn get_id(&self) -> Option<String>;

    //fn get_ip4_config(&self) -> /*Ignored*/Option<IPConfig>;

    //fn get_ip6_config(&self) -> /*Ignored*/Option<IPConfig>;

    fn get_master(&self) -> Option<Device>;

    fn get_specific_object_path(&self) -> Option<String>;

    //fn get_state(&self) -> /*Ignored*/ActiveConnectionState;

    //#[cfg(any(feature = "v1_10", feature = "dox"))]
    //fn get_state_flags(&self) -> /*Ignored*/ActivationStateFlags;

    //#[cfg(any(feature = "v1_8", feature = "dox"))]
    //fn get_state_reason(&self) -> /*Ignored*/ActiveConnectionStateReason;

    fn get_uuid(&self) -> Option<String>;

    fn get_vpn(&self) -> bool;

    fn get_property_type(&self) -> Option<String>;

    fn connect_state_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_connection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default6_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_devices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_dhcp4_config_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_dhcp6_config_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ip4_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ip6_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_master_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_specific_object_path_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_state_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_uuid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_vpn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ActiveConnection> + IsA<glib::object::Object>> ActiveConnectionExt for O {
    fn get_connection(&self) -> Option<RemoteConnection> {
        unsafe {
            from_glib_none(ffi::nm_active_connection_get_connection(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_connection_type(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_active_connection_get_connection_type(
                self.to_glib_none().0,
            ))
        }
    }

    fn get_default(&self) -> bool {
        unsafe { from_glib(ffi::nm_active_connection_get_default(self.to_glib_none().0)) }
    }

    fn get_default6(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_active_connection_get_default6(
                self.to_glib_none().0,
            ))
        }
    }

    //fn get_devices(&self) -> /*Unknown conversion*//*Unimplemented*/PtrArray TypeId { ns_id: 1, id: 11 } {
    //    unsafe { TODO: call ffi::nm_active_connection_get_devices() }
    //}

    //fn get_dhcp4_config(&self) -> /*Ignored*/Option<DhcpConfig> {
    //    unsafe { TODO: call ffi::nm_active_connection_get_dhcp4_config() }
    //}

    //fn get_dhcp6_config(&self) -> /*Ignored*/Option<DhcpConfig> {
    //    unsafe { TODO: call ffi::nm_active_connection_get_dhcp6_config() }
    //}

    fn get_id(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_active_connection_get_id(self.to_glib_none().0)) }
    }

    //fn get_ip4_config(&self) -> /*Ignored*/Option<IPConfig> {
    //    unsafe { TODO: call ffi::nm_active_connection_get_ip4_config() }
    //}

    //fn get_ip6_config(&self) -> /*Ignored*/Option<IPConfig> {
    //    unsafe { TODO: call ffi::nm_active_connection_get_ip6_config() }
    //}

    fn get_master(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::nm_active_connection_get_master(self.to_glib_none().0)) }
    }

    fn get_specific_object_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::nm_active_connection_get_specific_object_path(
                self.to_glib_none().0,
            ))
        }
    }

    //fn get_state(&self) -> /*Ignored*/ActiveConnectionState {
    //    unsafe { TODO: call ffi::nm_active_connection_get_state() }
    //}

    //#[cfg(any(feature = "v1_10", feature = "dox"))]
    //fn get_state_flags(&self) -> /*Ignored*/ActivationStateFlags {
    //    unsafe { TODO: call ffi::nm_active_connection_get_state_flags() }
    //}

    //#[cfg(any(feature = "v1_8", feature = "dox"))]
    //fn get_state_reason(&self) -> /*Ignored*/ActiveConnectionStateReason {
    //    unsafe { TODO: call ffi::nm_active_connection_get_state_reason() }
    //}

    fn get_uuid(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_active_connection_get_uuid(self.to_glib_none().0)) }
    }

    fn get_vpn(&self) -> bool {
        unsafe { from_glib(ffi::nm_active_connection_get_vpn(self.to_glib_none().0)) }
    }

    fn get_property_type(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(
                self.to_glib_none().0,
                "type".to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value.get()
        }
    }

    fn connect_state_changed<F: Fn(&Self, u32, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, u32, u32) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "state-changed",
                transmute(state_changed_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_connection_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::connection",
                transmute(notify_connection_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::default",
                transmute(notify_default_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_default6_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::default6",
                transmute(notify_default6_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_devices_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::devices",
                transmute(notify_devices_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_dhcp4_config_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::dhcp4-config",
                transmute(notify_dhcp4_config_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_dhcp6_config_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::dhcp6-config",
                transmute(notify_dhcp6_config_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::id",
                transmute(notify_id_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_ip4_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::ip4-config",
                transmute(notify_ip4_config_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_ip6_config_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::ip6-config",
                transmute(notify_ip6_config_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_master_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::master",
                transmute(notify_master_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_specific_object_path_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::specific-object-path",
                transmute(notify_specific_object_path_trampoline::<Self> as usize),
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

    #[cfg(any(feature = "v1_10", feature = "dox"))]
    fn connect_property_state_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::state-flags",
                transmute(notify_state_flags_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::type",
                transmute(notify_type_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_uuid_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::uuid",
                transmute(notify_uuid_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    fn connect_property_vpn_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::vpn",
                transmute(notify_vpn_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

unsafe extern "C" fn state_changed_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    object: libc::c_uint,
    p0: libc::c_uint,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P, u32, u32) + 'static) = transmute(f);
    f(
        &ActiveConnection::from_glib_borrow(this).downcast_unchecked(),
        object,
        p0,
    )
}

unsafe extern "C" fn notify_connection_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_default_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_default6_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_devices_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_dhcp4_config_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_dhcp6_config_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_id_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ip4_config_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_ip6_config_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_master_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_specific_object_path_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_state_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_10", feature = "dox"))]
unsafe extern "C" fn notify_state_flags_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_type_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_uuid_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_vpn_trampoline<P>(
    this: *mut ffi::NMActiveConnection,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<ActiveConnection>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ActiveConnection::from_glib_borrow(this).downcast_unchecked())
}
