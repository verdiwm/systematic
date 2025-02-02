use std::{
    ffi::{CStr, CString},
    io,
    mem::MaybeUninit,
};

use crate::{sys, CStrArg};

macro_rules! try_err {
    ($ret:expr) => {
        if $ret < 0 {
            return Err(io::Error::from_raw_os_error($ret.wrapping_neg()));
        }
    };
}

pub fn session_is_active<C: CStrArg>(session: C) -> io::Result<bool> {
    let ret = unsafe { sys::sd_session_is_active(session.into_c_str()?.as_ptr()) };

    try_err!(ret);

    Ok(ret != 0)
}

pub fn session_get_seat<C: CStrArg>(session: C) -> io::Result<CString> {
    let mut seat = MaybeUninit::uninit();

    try_err!(unsafe {
        sys::sd_session_get_seat(session.into_c_str()?.as_ptr(), seat.as_mut_ptr())
    });

    let seat_ptr = unsafe { seat.assume_init() };

    let seat = unsafe { CStr::from_ptr(seat_ptr).to_owned() };

    unsafe { libc::free(seat_ptr.cast()) }

    Ok(seat)
}
