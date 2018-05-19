use conv::*;

use std::{mem, os::raw::c_int};

use super::{universe, handles::MPI_Comm, status::*};

#[no_mangle]
pub extern "C" fn MPI_Comm_rank(comm: MPI_Comm, rank: *mut c_int) -> Error {
    unsafe { *rank = comm.get().rank().value_as().unwrap() };

    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Comm_size(comm: MPI_Comm, rank: *mut c_int) -> Error {
    unsafe { *rank = comm.get().size().value_as().unwrap() };

    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Comm_test_inter(comm: MPI_Comm, flag: *mut c_int) -> Error {
    unsafe { *flag = comm.get().is_intercomm() as c_int };

    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Comm_get_parent(parent: *mut MPI_Comm) -> Error {
    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Comm_free(comm: Option<&mut MPI_Comm>) -> Error {
    let comm = unsafe {
        mem::replace(
            comm.expect("NULL is not a valid parameter to MPI_Comm_free."),
            super::init::MPI_COMM_NULL,
        )
    };

    let registration = unsafe { comm.expect_user_comm() };

    {
        let mut locked = universe().write().unwrap();
        locked.free_comm(registration);
    }

    Error::MPI_SUCCESS
}
