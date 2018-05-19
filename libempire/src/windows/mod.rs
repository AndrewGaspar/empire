use std::{slice, ffi::OsString, os::windows::ffi::OsStringExt};

pub unsafe fn win_string_from_ptr(ptr: *const u16) -> OsString {
    let mut null_term_offset: usize = 0;
    while *ptr.offset(null_term_offset as isize) != 0 {
        null_term_offset += 1;
    }
    OsString::from_wide(slice::from_raw_parts(ptr, null_term_offset))
}
