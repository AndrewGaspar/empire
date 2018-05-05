use conv::*;

use std::os::raw::c_int;

use super::status::*;
use super::handles::MPI_Comm;

#[no_mangle]
pub extern "C" fn MPI_Comm_rank(comm: MPI_Comm, rank: *mut c_int) -> Error {
    unsafe { *rank = comm.unwrap().rank().value_as().unwrap() };

    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Comm_size(comm: MPI_Comm, rank: *mut c_int) -> Error {
    unsafe { *rank = comm.unwrap().size().value_as().unwrap() };

    Error::MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Comm_test_inter(comm: MPI_Comm, flag: *mut c_int) -> Error {
    unsafe { *flag = comm.unwrap().is_intercomm() as c_int };

    Error::MPI_SUCCESS
}
