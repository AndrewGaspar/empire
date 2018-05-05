use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::slice;
use std::sync::{Arc, RwLock};

use super::status::*;
use super::handles::*;

use empire::{Comm, Info, Universe};

static mut UNIVERSE: Option<Arc<RwLock<Universe>>> = None;

#[no_mangle]
pub static mut MPI_COMM_SELF: MPI_Comm = MPI_Comm {
    handle: ptr::null(),
};

#[no_mangle]
pub static mut MPI_COMM_WORLD: MPI_Comm = MPI_Comm {
    handle: ptr::null(),
};

#[no_mangle]
pub static mut MPI_COMM_NULL: MPI_Comm = MPI_Comm {
    handle: ptr::null(),
};

#[no_mangle]
pub static mut MPI_INFO_NULL: MPI_Info = MPI_Info {
    handle: ptr::null(),
};

pub fn universe() -> &'static Arc<RwLock<Universe>> {
    unsafe {
        UNIVERSE
            .as_ref()
            .expect("MPI must be initialized prior to calling this MPI routine.")
    }
}

fn initialize_mpi() -> Error {
    let locked = universe().write().unwrap();

    unsafe {
        MPI_COMM_SELF = MPI_Comm {
            handle: locked.comm_self_opt(),
        }
    };
    unsafe {
        MPI_COMM_WORLD = MPI_Comm {
            handle: locked.comm_world_opt(),
        }
    };
    unsafe {
        MPI_COMM_NULL = MPI_Comm {
            handle: Box::into_raw(Box::new(None)),
        }
    };
    unsafe {
        MPI_INFO_NULL = MPI_Info {
            handle: Box::into_raw(Box::new(None)),
        }
    };

    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Init(argc: *mut c_int, argv: *mut *mut *mut c_char) -> Error {
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

    initialize_mpi()
}

#[cfg(windows)]
#[no_mangle]
pub extern "C" fn MPI_InitW(argc: *mut c_int, argv: *mut *mut *mut u16) -> Error {
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

    initialize_mpi()
}

#[no_mangle]
pub extern "C" fn MPI_Finalize() -> Error {
    unsafe {
        UNIVERSE = None;

        Box::from_raw(MPI_COMM_NULL.handle as *mut Option<Comm>);
        Box::from_raw(MPI_INFO_NULL.handle as *mut Option<Info>);

        MPI_COMM_SELF.handle = ptr::null();
        MPI_COMM_WORLD.handle = ptr::null();
        MPI_COMM_NULL.handle = ptr::null();
    }

    Error::MPI_SUCCESS
}
