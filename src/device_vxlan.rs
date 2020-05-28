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
    pub struct DeviceVxlan(Object<nm_sys::NMDeviceVxlan, nm_sys::NMDeviceVxlanClass, DeviceVxlanClass>) @extends Device, Object;

    match fn {
        get_type => || nm_sys::nm_device_vxlan_get_type(),
    }
}

impl DeviceVxlan {
    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_ageing(&self) -> u32 {
        unsafe { nm_sys::nm_device_vxlan_get_ageing(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_carrier(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_vxlan_get_carrier(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_dst_port(&self) -> u32 {
        unsafe { nm_sys::nm_device_vxlan_get_dst_port(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_group(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_device_vxlan_get_group(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_id(&self) -> u32 {
        unsafe { nm_sys::nm_device_vxlan_get_id(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_l2miss(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_vxlan_get_l2miss(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_l3miss(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_vxlan_get_l3miss(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_learning(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_vxlan_get_learning(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_limit(&self) -> u32 {
        unsafe { nm_sys::nm_device_vxlan_get_limit(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_local(&self) -> Option<GString> {
        unsafe { from_glib_none(nm_sys::nm_device_vxlan_get_local(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_parent(&self) -> Option<Device> {
        unsafe { from_glib_none(nm_sys::nm_device_vxlan_get_parent(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_proxy(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_vxlan_get_proxy(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_rsc(&self) -> bool {
        unsafe { from_glib(nm_sys::nm_device_vxlan_get_rsc(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_src_port_max(&self) -> u32 {
        unsafe { nm_sys::nm_device_vxlan_get_src_port_max(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_src_port_min(&self) -> u32 {
        unsafe { nm_sys::nm_device_vxlan_get_src_port_min(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_tos(&self) -> u32 {
        unsafe { nm_sys::nm_device_vxlan_get_tos(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn get_ttl(&self) -> u32 {
        unsafe { nm_sys::nm_device_vxlan_get_ttl(self.to_glib_none().0) }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_ageing_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ageing_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::ageing\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ageing_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_carrier_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_carrier_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_dst_port_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_dst_port_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::dst-port\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dst_port_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_group_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_group_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
    pub fn connect_property_id_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_id_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::id\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_id_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_l2miss_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_l2miss_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::l2miss\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_l2miss_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_l3miss_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_l3miss_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::l3miss\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_l3miss_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_learning_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_learning_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::learning\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_learning_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_limit_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_limit_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::limit\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_limit_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_local_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_local_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::local\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_local_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_parent_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_proxy_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_proxy_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::proxy\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_proxy_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_rsc_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_rsc_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::rsc\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_rsc_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_src_port_max_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_src_port_max_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::src-port-max\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_src_port_max_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_src_port_min_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_src_port_min_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::src-port-min\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_src_port_min_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_tos_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_tos_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::tos\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_tos_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v1_2", feature = "dox"))]
    pub fn connect_property_ttl_notify<F: Fn(&DeviceVxlan) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_ttl_trampoline<F: Fn(&DeviceVxlan) + 'static>(
            this: *mut nm_sys::NMDeviceVxlan,
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
                b"notify::ttl\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ttl_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for DeviceVxlan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceVxlan")
    }
}
