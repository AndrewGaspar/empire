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
    unsafe { UNIVERSE = Some(mpitry!(Universe::from_env())) };

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
pub extern "C" fn MPI_Init(_: *mut c_int, _: *mut *mut *mut c_char) -> Error {
    initialize_mpi()
}

#[cfg(windows)]
#[no_mangle]
pub extern "C" fn MPI_InitW(_: *mut c_int, _: *mut *mut *mut u16) -> Error {
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
