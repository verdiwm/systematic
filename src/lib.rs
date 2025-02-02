#[allow(missing_docs)]
#[allow(nonstandard_style)]
#[allow(clippy::all)]
pub mod sys;

pub mod login;

use std::{
    borrow::Cow,
    ffi::{CStr, CString},
    io,
};

pub trait CStrArg {
    fn into_c_str(&self) -> io::Result<Cow<'_, CStr>>;
}

impl CStrArg for &str {
    fn into_c_str(&self) -> io::Result<Cow<'_, CStr>> {
        Ok(Cow::Owned(
            CString::new(*self).map_err(|_| io::Error::from_raw_os_error(22))?,
        ))
    }
}

impl CStrArg for &CStr {
    fn into_c_str(&self) -> io::Result<Cow<'_, CStr>> {
        Ok(Cow::Borrowed(&self))
    }
}
