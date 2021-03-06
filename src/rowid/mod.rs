// Copyright (c) 2017 mimir developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! This structure is used to represent the unique identifier of a row in the database and is
//! available by handle to a calling application or driver. Rowids cannot be created or set directly
//! but are created implicitly when a variable of type `DPI_ORACLE_TYPE_ROWID` is created. They are
//! destroyed when the last reference is released by a call to the function `Rowid::release()`. All
//! of the attributes of the structure `ODPIBaseType` are included in this structure in addition to
//! the ones specific to this structure described below.
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::opaque::ODPIRowid;
use std::ptr;
use util::ODPIStr;

/// This structure is used to represent the unique identifier of a row in the database and is
/// available by handle to a calling application or driver.
pub struct Rowid {
    /// The ODPI-C rowid
    inner: *mut ODPIRowid,
}

impl Rowid {
    /// Get the `inner` value.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIRowid {
        self.inner
    }

    /// Returns the string (base64) representation of the rowid.
    pub fn get_string_value(&self) -> Result<String> {
        let mut value = ptr::null();
        let mut value_length = 0;

        try_dpi!(
            externs::dpiRowid_getStringValue(self.inner, &mut value, &mut value_length),
            {
                if value.is_null() {
                    Ok("".to_string())
                } else {
                    let res = ODPIStr::new(value, value_length);
                    Ok(res.into())
                }
            },
            ErrorKind::Rowid("dpiRowid_getStringValue".to_string())
        )
    }
}

impl From<*mut ODPIRowid> for Rowid {
    fn from(inner: *mut ODPIRowid) -> Self {
        Self { inner }
    }
}

impl Drop for Rowid {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe {
                externs::dpiRowid_release(self.inner);
            }
        }
    }
}
