use std::io;

use crate::{sys, CStrArg};

pub fn session_is_active<C: CStrArg>(session: C) -> io::Result<bool> {
    let ret = unsafe { sys::sd_session_is_active(session.into_c_str()?.as_ptr()) };

    if ret < 0 {
        return Err(io::Error::from_raw_os_error(ret.wrapping_neg()));
    }

    Ok(ret != 0)
}
