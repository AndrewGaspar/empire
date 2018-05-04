use conv::*;

use std::os::raw::c_int;

use empire::Comm;
use super::status::*;

#[no_mangle]
pub extern "C" fn MPI_Comm_rank(comm: *const Comm, rank: *mut c_int) -> c_int {
    unsafe { *rank = (*comm).rank().value_as().unwrap() };

    MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Comm_size(comm: *const Comm, rank: *mut c_int) -> c_int {
    unsafe { *rank = (*comm).size().value_as().unwrap() };

    MPI_SUCCESS
}
