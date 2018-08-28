// This file was generated by gir (https://github.com/gtk-rs/gir @ 464833e)
// from gir-files (https://github.com/gtk-rs/gir-files @ ???)
// DO NOT EDIT

use ffi;
use glib::object::Downcast;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;
use Setting;

glib_wrapper! {
    pub struct SettingDummy(Object<ffi::NMSettingDummy, ffi::NMSettingDummyClass>): Setting;

    match fn {
        get_type => || ffi::nm_setting_dummy_get_type(),
    }
}

impl SettingDummy {
    #[cfg(any(feature = "v1_8", feature = "dox"))]
    pub fn new() -> SettingDummy {
        unsafe { Setting::from_glib_full(ffi::nm_setting_dummy_new()).downcast_unchecked() }
    }
}

#[cfg(any(feature = "v1_8", feature = "dox"))]
impl Default for SettingDummy {
    fn default() -> Self {
        Self::new()
    }
}