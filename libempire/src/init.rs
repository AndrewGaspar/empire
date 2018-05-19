use std::{ptr, boxed::Box, os::raw::{c_char, c_int}, sync::{Arc, RwLock}};

use super::{handles::*, status::*};

use empire::{Info, Universe};

static mut UNIVERSE: Option<Arc<RwLock<Universe>>> = None;

#[no_mangle]
pub static mut MPI_COMM_SELF: MPI_Comm = MPI_Comm {
    handle: ptr::null_mut(),
};

#[no_mangle]
pub static mut MPI_COMM_WORLD: MPI_Comm = MPI_Comm {
    handle: ptr::null_mut(),
};

#[no_mangle]
pub static mut MPI_COMM_NULL: MPI_Comm = MPI_Comm {
    handle: ptr::null_mut(),
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
        MPI_COMM_SELF = MPI_Comm::new(CommHandle::SystemComm(Arc::downgrade(&locked.comm_self())))
    };
    unsafe {
        MPI_COMM_WORLD = MPI_Comm::new(CommHandle::SystemComm(Arc::downgrade(&locked.comm_world())))
    };
    unsafe { MPI_COMM_NULL = MPI_Comm::new(CommHandle::NullComm) };
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
        MPI_COMM_SELF.free();
        MPI_COMM_WORLD.free();
        MPI_COMM_NULL.free();

        Box::from_raw(MPI_INFO_NULL.handle as *mut Option<Info>);
        MPI_INFO_NULL.handle = ptr::null();

        UNIVERSE = None;
    }

    Error::MPI_SUCCESS
}
