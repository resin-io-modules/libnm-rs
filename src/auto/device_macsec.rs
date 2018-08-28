// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use glib::signal::connect;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v1_6", feature = "dox"))]
use std::mem::transmute;
use std::ptr;
use Device;

glib_wrapper! {
    pub struct DeviceMacsec(Object<ffi::NMDeviceMacsec, ffi::NMDeviceMacsecClass>): Device;

    match fn {
        get_type => || ffi::nm_device_macsec_get_type(),
    }
}

pub trait DeviceMacsecExt {
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_cipher_suite(&self) -> u64;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_encoding_sa(&self) -> u8;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_encrypt(&self) -> bool;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_es(&self) -> bool;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_icv_length(&self) -> u8;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_include_sci(&self) -> bool;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_parent(&self) -> Option<Device>;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_protect(&self) -> bool;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_replay_protect(&self) -> bool;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_scb(&self) -> bool;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_sci(&self) -> u64;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_validation(&self) -> Option<String>;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_window(&self) -> u32;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_cipher_suite_notify<F: Fn(&Self) + 'static>(&self, f: F)
        -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_encoding_sa_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_encrypt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_es_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_icv_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_include_sci_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_protect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_replay_protect_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_scb_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_sci_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_validation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DeviceMacsec> + IsA<glib::object::Object>> DeviceMacsecExt for O {
    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_cipher_suite(&self) -> u64 {
        unsafe { ffi::nm_device_macsec_get_cipher_suite(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_encoding_sa(&self) -> u8 {
        unsafe { ffi::nm_device_macsec_get_encoding_sa(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_encrypt(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_macsec_get_encrypt(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_es(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_macsec_get_es(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_icv_length(&self) -> u8 {
        unsafe { ffi::nm_device_macsec_get_icv_length(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_include_sci(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_macsec_get_include_sci(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_parent(&self) -> Option<Device> {
        unsafe { from_glib_none(ffi::nm_device_macsec_get_parent(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_protect(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_macsec_get_protect(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_replay_protect(&self) -> bool {
        unsafe {
            from_glib(ffi::nm_device_macsec_get_replay_protect(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_scb(&self) -> bool {
        unsafe { from_glib(ffi::nm_device_macsec_get_scb(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_sci(&self) -> u64 {
        unsafe { ffi::nm_device_macsec_get_sci(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_validation(&self) -> Option<String> {
        unsafe { from_glib_none(ffi::nm_device_macsec_get_validation(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn get_window(&self) -> u32 {
        unsafe { ffi::nm_device_macsec_get_window(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_cipher_suite_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::cipher-suite",
                transmute(notify_cipher_suite_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_encoding_sa_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::encoding-sa",
                transmute(notify_encoding_sa_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_encrypt_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::encrypt",
                transmute(notify_encrypt_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_es_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::es",
                transmute(notify_es_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_hw_address_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::hw-address",
                transmute(notify_hw_address_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_icv_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::icv-length",
                transmute(notify_icv_length_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_include_sci_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::include-sci",
                transmute(notify_include_sci_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
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

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_protect_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::protect",
                transmute(notify_protect_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_replay_protect_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::replay-protect",
                transmute(notify_replay_protect_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_scb_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::scb",
                transmute(notify_scb_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_sci_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::sci",
                transmute(notify_sci_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_validation_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::validation",
                transmute(notify_validation_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }

    #[cfg(any(feature = "v1_6", feature = "dox"))]
    fn connect_property_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(
                self.to_glib_none().0,
                "notify::window",
                transmute(notify_window_trampoline::<Self> as usize),
                Box_::into_raw(f) as *mut _,
            )
        }
    }
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_cipher_suite_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_encoding_sa_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_encrypt_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_es_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_hw_address_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_icv_length_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_include_sci_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_parent_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_protect_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_replay_protect_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_scb_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_sci_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_validation_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v1_6", feature = "dox"))]
unsafe extern "C" fn notify_window_trampoline<P>(
    this: *mut ffi::NMDeviceMacsec,
    _param_spec: glib_ffi::gpointer,
    f: glib_ffi::gpointer,
) where
    P: IsA<DeviceMacsec>,
{
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DeviceMacsec::from_glib_borrow(this).downcast_unchecked())
}