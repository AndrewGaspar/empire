use std::ffi::CStr;

use std::os::raw::{c_char, c_int};
use std::slice;

use super::status::*;

use empire::Universe;

static mut UNIVERSE: Option<Universe> = None;

#[no_mangle]
pub extern "C" fn MPI_Init(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int {
    let args = match unsafe { (argc.as_ref(), argv.as_ref()) } {
        (Some(argc), Some(argv)) => {
            let args = unsafe { slice::from_raw_parts(*argv, *argc as usize) };
            Some(
                args.iter()
                    .map(|arg| unsafe { CStr::from_ptr(*arg) }.to_str().unwrap().to_owned()),
            )
        }
        _ => None,
    };

    unsafe { UNIVERSE = Some(Universe::from_args(args)) };

    MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Finalize() -> c_int {
    unsafe { UNIVERSE = None };

    MPI_SUCCESS
}
