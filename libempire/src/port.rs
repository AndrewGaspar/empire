use super::Error;

use super::constants::*;
use super::handles::MPI_Info;
use super::universe;

use std::cmp::min;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::slice;

#[no_mangle]
pub extern "C" fn MPI_Open_port(_: MPI_Info, port_name: *mut c_char) -> Error {
    let mut locked = universe().write().unwrap();

    let port = mpitry!(locked.open_port());

    let out_port_name = unsafe { slice::from_raw_parts_mut(port_name, MAX_PORT_NAME) };
    let port_name_bytes = port.name().as_bytes();

    let name_length = min(out_port_name.len() - 1, port_name_bytes.len());

    for i in 0..name_length {
        out_port_name[i] = port_name_bytes[i] as c_char;
    }

    out_port_name[name_length] = 0;

    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Close_port(port_name: *mut c_char) -> Error {
    let mut locked = universe().write().unwrap();

    let port_name = unsafe { CStr::from_ptr(port_name) };

    mpitry!(locked.close_port(port_name.to_str().unwrap()));

    Error::MPI_SUCCESS
}
