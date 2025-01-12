//! Internal representations to communicate with giac

use std::{
    ffi::{c_char, CStr},
    fmt::{self, Debug, Display, Formatter},
    ops::Deref,
};

use crate::ffi;

/// Represents errors returned by any call to a giac function
#[derive(Debug, PartialEq, Eq)]
pub enum GiacError {
    /// Represents an error which occured inside the giac library
    InternalError(GiacString),
    /// Equation don't have any solutions
    NoSolution(&'static str),
}

/// Represents all string created from giac. You should use this type if possible to avoid a copy, as a conversion to a rust string needs to copy the entire string.
#[derive(Eq)]
pub struct GiacString {
    pub(crate) ptr: *const c_char,
}

impl GiacString {
    pub(crate) unsafe fn new(ptr: *const c_char) -> Self {
        GiacString { ptr }
    }
}

impl Deref for GiacString {
    type Target = CStr;

    fn deref(&self) -> &Self::Target {
        unsafe { CStr::from_ptr(self.ptr) }
    }
}

impl Debug for GiacString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.deref())
    }
}

impl Display for GiacString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.deref().to_string_lossy())
    }
}

impl PartialEq for GiacString {
    fn eq(&self, other: &GiacString) -> bool {
        **self == **other
    }
}

impl Drop for GiacString {
    fn drop(&mut self) {
        unsafe {
            ffi::giacrs_free_str(self.ptr);
        }
    }
}

impl From<*const c_char> for GiacString {
    fn from(value: *const c_char) -> Self {
        unsafe { Self::new(value) }
    }
}
