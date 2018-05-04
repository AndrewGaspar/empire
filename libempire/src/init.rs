use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::slice;
use std::sync::{Arc, RwLock};

use super::status::*;

use empire::{Comm, Universe};

static mut UNIVERSE: Option<Arc<RwLock<Universe>>> = None;

#[no_mangle]
pub static mut MPI_COMM_SELF: *const Comm = ptr::null();

#[no_mangle]
pub static mut MPI_COMM_WORLD: *const Comm = ptr::null();

pub fn universe() -> &'static Arc<RwLock<Universe>> {
    unsafe {
        UNIVERSE
            .as_ref()
            .expect("MPI must be initialized prior to calling this MPI routine.")
    }
}

#[no_mangle]
pub extern "C" fn MPI_Init(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int {
    {
        let universe = match unsafe { (argc.as_ref(), argv.as_ref()) } {
            (Some(argc), Some(argv)) => {
                let args = unsafe { slice::from_raw_parts(*argv, *argc as usize) };
                Universe::from_args(
                    args.iter()
                        .map(|arg| unsafe { CStr::from_ptr(*arg) }.to_str().unwrap().to_owned()),
                )
            }
            _ => Universe::new(),
        };

        unsafe { UNIVERSE = Some(universe) };
    }

    let locked = universe().write().unwrap();

    unsafe { MPI_COMM_SELF = locked.comm_self() };
    unsafe { MPI_COMM_WORLD = locked.comm_world() };

    MPI_SUCCESS
}

#[cfg(windows)]
#[no_mangle]
pub extern "C" fn MPI_InitW(argc: *mut c_int, argv: *mut *mut *mut u16) -> c_int {
    {
        let universe = match unsafe { (argc.as_ref(), argv.as_ref()) } {
            (Some(argc), Some(argv)) => {
                let args = unsafe { slice::from_raw_parts(*argv, *argc as usize) };
                Universe::from_args_os(
                    args.iter()
                        .map(|arg| unsafe { super::windows::win_string_from_ptr(*arg) }),
                )
            }
            _ => Universe::new(),
        };

        unsafe { UNIVERSE = Some(universe) };
    }

    let locked = universe().write().unwrap();

    unsafe {
        MPI_COMM_SELF = locked.comm_self();
        MPI_COMM_WORLD = locked.comm_world();
    }

    MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Finalize() -> c_int {
    unsafe {
        UNIVERSE = None;
        MPI_COMM_SELF = ptr::null();
        MPI_COMM_WORLD = ptr::null();
    }

    MPI_SUCCESS
}
