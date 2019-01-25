use std::{io, ptr, ffi, slice};
use libc::{c_int, statfs};
use data::*;
use super::common::*;

pub struct PlatformImpl;

impl Platform for PlatformImpl {
    #[inline(always)]
    fn new() -> Self {
        PlatformImpl
    }

    fn drives(&self) -> io::Result<Vec<Drive>> {
        let mut mptr: *mut statfs = ptr::null_mut();
        let len = unsafe { getmntinfo(&mut mptr, 2 as i32) };
        if len < 1 {
            println!("getmntinfo() failed")
        }
        let _drives = unsafe { slice::from_raw_parts(mptr, len as usize) };
        Ok(_drives.iter().map(statfs_to_drive).collect::<Vec<_>>())
    }
}

#[link(name = "c")]
extern "C" {
    #[link_name = "getmntinfo$INODE64"]
    fn getmntinfo(mntbufp: *mut *mut statfs, flags: c_int) -> c_int;
}

fn statfs_to_drive(stats: &statfs) -> Drive {
    Drive {
        drive_type: unsafe { ffi::CStr::from_ptr(&stats.f_fstypename[0]).to_string_lossy().into_owned() },
        mounted_from: unsafe { ffi::CStr::from_ptr(&stats.f_mntfromname[0]).to_string_lossy().into_owned() },
        mounted_on: unsafe { ffi::CStr::from_ptr(&stats.f_mntonname[0]).to_string_lossy().into_owned() },
    }
}
