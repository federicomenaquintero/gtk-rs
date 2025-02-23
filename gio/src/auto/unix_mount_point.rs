// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_54", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
use crate::Icon;
#[cfg(any(feature = "v2_54", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
use glib::translate::*;
#[cfg(any(feature = "v2_54", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
use std::cmp;
#[cfg(any(feature = "v2_66", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
use std::mem;

glib::wrapper! {
    #[derive(Debug)]
    pub struct UnixMountPoint(Boxed<ffi::GUnixMountPoint>);

    match fn {
        copy => |ptr| ffi::g_unix_mount_point_copy(mut_override(ptr)),
        free => |ptr| ffi::g_unix_mount_point_free(ptr),
        get_type => || ffi::g_unix_mount_point_get_type(),
    }
}

impl UnixMountPoint {
    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_compare")]
    fn compare(&self, mount2: &UnixMountPoint) -> i32 {
        unsafe {
            ffi::g_unix_mount_point_compare(
                mut_override(self.to_glib_none().0),
                mut_override(mount2.to_glib_none().0),
            )
        }
    }

    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_get_device_path")]
    pub fn get_device_path(&self) -> std::path::PathBuf {
        unsafe {
            from_glib_none(ffi::g_unix_mount_point_get_device_path(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_get_fs_type")]
    pub fn get_fs_type(&self) -> glib::GString {
        unsafe {
            from_glib_none(ffi::g_unix_mount_point_get_fs_type(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_get_mount_path")]
    pub fn get_mount_path(&self) -> std::path::PathBuf {
        unsafe {
            from_glib_none(ffi::g_unix_mount_point_get_mount_path(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_get_options")]
    pub fn get_options(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_unix_mount_point_get_options(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_guess_can_eject")]
    pub fn guess_can_eject(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_mount_point_guess_can_eject(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_guess_icon")]
    pub fn guess_icon(&self) -> Icon {
        unsafe {
            from_glib_full(ffi::g_unix_mount_point_guess_icon(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_guess_name")]
    pub fn guess_name(&self) -> glib::GString {
        unsafe {
            from_glib_full(ffi::g_unix_mount_point_guess_name(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_guess_symbolic_icon")]
    pub fn guess_symbolic_icon(&self) -> Icon {
        unsafe {
            from_glib_full(ffi::g_unix_mount_point_guess_symbolic_icon(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_is_loopback")]
    pub fn is_loopback(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_mount_point_is_loopback(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_is_readonly")]
    pub fn is_readonly(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_mount_point_is_readonly(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_54", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
    #[doc(alias = "g_unix_mount_point_is_user_mountable")]
    pub fn is_user_mountable(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_mount_point_is_user_mountable(mut_override(
                self.to_glib_none().0,
            )))
        }
    }

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_unix_mount_point_at")]
    pub fn at<P: AsRef<std::path::Path>>(mount_path: P) -> (Option<UnixMountPoint>, u64) {
        unsafe {
            let mut time_read = mem::MaybeUninit::uninit();
            let ret = from_glib_full(ffi::g_unix_mount_point_at(
                mount_path.as_ref().to_glib_none().0,
                time_read.as_mut_ptr(),
            ));
            let time_read = time_read.assume_init();
            (ret, time_read)
        }
    }
}

#[cfg(any(feature = "v2_54", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
impl PartialEq for UnixMountPoint {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.compare(other) == 0
    }
}

impl Eq for UnixMountPoint {}

#[cfg(any(feature = "v2_54", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_54")))]
impl PartialOrd for UnixMountPoint {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.compare(other).partial_cmp(&0)
    }
}

impl Ord for UnixMountPoint {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.compare(other).cmp(&0)
    }
}
