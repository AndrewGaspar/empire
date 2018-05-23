use super::{constants, Error};

use conv::*;
use std::{mem, ptr, slice, collections::HashMap, ffi::CStr, os::raw::{c_char, c_int}};

#[no_mangle]
pub static mut MPI_INFO_NULL: MPI_Info = MPI_Info {
    handle: ptr::null_mut(),
};

#[derive(Clone)]
pub struct Info {
    map: HashMap<String, String>,
    keys: Vec<String>,
}

impl Info {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            keys: Vec::new(),
        }
    }

    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) -> Error {
        let key = key.into();
        let value = value.into();

        if key.len() > constants::MPI_MAX_INFO_KEY {
            return Error::MPI_ERR_INFO_KEY;
        }

        if value.len() > constants::MPI_MAX_INFO_VAL {
            return Error::MPI_ERR_INFO_VALUE;
        }

        if self.map.insert(key.clone(), value).is_none() {
            self.keys.push(key);
        }

        Error::MPI_SUCCESS
    }

    pub fn delete(&mut self, key: &str) -> Error {
        if self.map.remove(key).is_some() {
            let index = self.keys
                .iter()
                .position(|existing_key| existing_key == key)
                .unwrap();
            self.keys.remove(index);
            Error::MPI_SUCCESS
        } else {
            Error::MPI_ERR_INFO_NOKEY
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        assert!(key.len() <= constants::MPI_MAX_INFO_KEY);

        self.map.get(key).map(|string| string.as_str())
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn get_nthkey(&self, index: usize) -> &str {
        &self.keys[index]
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MPI_Info {
    pub handle: *mut Option<Info>,
}

impl MPI_Info {
    pub fn null() -> Self {
        Self {
            handle: Box::into_raw(Box::new(None)),
        }
    }

    pub fn new() -> Self {
        Self {
            handle: Box::into_raw(Box::new(Some(Info::new()))),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            handle: Box::into_raw(Box::new(unsafe { &*self.handle }.clone())),
        }
    }

    fn assert_non_null(&self) {
        assert!(
            self.handle != ptr::null_mut(),
            "NULL is not an allowed value for MPI_Comm. You may not have initialized MPI yet. Use \
             MPI_INFO_NULL instead."
        );
    }

    pub unsafe fn get(&self) -> &Option<Info> {
        self.assert_non_null();
        &*self.handle
    }

    pub unsafe fn get_mut(&mut self) -> &mut Option<Info> {
        self.assert_non_null();
        &mut *self.handle
    }

    pub unsafe fn is_null(&self) -> bool {
        self.assert_non_null();
        self.get().is_none()
    }

    pub unsafe fn unwrap(&self) -> &Info {
        self.get()
            .as_ref()
            .expect("MPI_INFO_NULL is not allowed in this routine.")
    }

    pub unsafe fn unwrap_mut(&mut self) -> &mut Info {
        self.get_mut()
            .as_mut()
            .expect("MPI_INFO_NULL is not allowed in this routine.")
    }

    pub unsafe fn free(&mut self) {
        self.assert_non_null();
        Box::from_raw(self.handle);
        self.handle = ptr::null_mut();
    }
}

#[no_mangle]
pub extern "C" fn MPI_Info_create(info: *mut MPI_Info) -> Error {
    unsafe { *info = MPI_Info::new() };
    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Info_set(
    mut info: MPI_Info,
    key: *const c_char,
    value: *const c_char,
) -> Error {
    let key = unsafe { CStr::from_ptr(key) }.to_str().unwrap();
    let value = unsafe { CStr::from_ptr(value) }.to_str().unwrap();
    unsafe { info.unwrap_mut() }.set(key, value)
}

#[no_mangle]
pub extern "C" fn MPI_Info_delete(mut info: MPI_Info, key: *const c_char) -> Error {
    let key = unsafe { CStr::from_ptr(key) }.to_str().unwrap();
    unsafe { info.unwrap_mut() }.delete(key)
}

#[no_mangle]
pub extern "C" fn MPI_Info_get(
    info: MPI_Info,
    key: *const c_char,
    valuelen: c_int,
    value: *mut c_char,
    flag: *mut c_int,
) -> Error {
    let key = unsafe { CStr::from_ptr(key) }.to_str().unwrap();

    match unsafe { info.unwrap() }.get(key) {
        Some(value_str) => {
            let valuelen = valuelen
                .value_as::<usize>()
                .expect("A negative valuelen is malformed.");

            // The standard (Chapter 9) indicates that the actual buffer must be (valuelen + 1),
            // allowing for the null terminator to be written there.
            assert!(valuelen >= value_str.len());

            let str_len = value_str.len();

            let value_slice = unsafe { slice::from_raw_parts_mut(value as *mut u8, str_len + 1) };

            value_slice[..str_len].copy_from_slice(value_str.as_bytes());
            // null terminate the string
            value_slice[value_str.len()] = 0;

            unsafe {
                *flag = 1;
            }
        }
        None => unsafe {
            *flag = 0;
        },
    }

    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Info_get_valuelen(
    info: MPI_Info,
    key: *const c_char,
    valuelen: *mut c_int,
    flag: *mut c_int,
) -> Error {
    let key = unsafe { CStr::from_ptr(key) }.to_str().unwrap();

    match unsafe { info.unwrap() }.get(key) {
        Some(value_str) => {
            unsafe {
                *valuelen = value_str.len().value_as().unwrap();
            }

            unsafe {
                *flag = 1;
            }
        }
        None => unsafe {
            *flag = 0;
        },
    }

    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Info_get_nkeys(info: MPI_Info, nkeys: *mut c_int) -> Error {
    unsafe {
        *nkeys = info.unwrap().len().value_as().unwrap();
    }
    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Info_get_nthkey(info: MPI_Info, n: c_int, key: *mut c_char) -> Error {
    let key_str =
        unsafe { info.unwrap() }.get_nthkey(n.value_as().expect("A negative n is malformed."));

    let key_str_len = key_str.len();

    let key_slice = unsafe { slice::from_raw_parts_mut(key as *mut u8, key_str_len + 1) };

    key_slice[..key_str_len].copy_from_slice(key_str.as_bytes());
    // null terminate the string
    key_slice[key_str.len()] = 0;

    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Info_dup(info: MPI_Info, newinfo: *mut MPI_Info) -> Error {
    unsafe {
        *newinfo = info.clone();
    }
    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Info_free(info: Option<&mut MPI_Info>) -> Error {
    unsafe {
        mem::replace(
            info.expect("NULL is not a valid parameter to MPI_Comm_free."),
            MPI_INFO_NULL,
        )
    };

    Error::MPI_SUCCESS
}
