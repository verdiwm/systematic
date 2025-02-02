use std::{ffi::CStr, io};

use crate::sys;

pub fn session_is_active(session: &CStr) -> io::Result<bool> {
    let ret = unsafe { sys::sd_session_is_active(session.as_ptr()) };

    if ret < 0 {
        return Err(io::Error::from_raw_os_error(ret.wrapping_neg()));
    }

    Ok(ret != 0)
}
