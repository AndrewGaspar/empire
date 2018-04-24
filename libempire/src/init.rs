use std::os::raw::{c_char, c_int};
use std::ffi::CStr;
use super::status::*;

#[no_mangle]
pub extern "C" fn MPI_Init(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int {
    println!("Hello from eMPIRe!");

    match unsafe { (argc.as_ref(), argv.as_ref()) } {
        (Some(argc), Some(argv)) => {
            println!("Args are:");
            for i in 0..*argc {
                let cstr = unsafe { CStr::from_ptr(*argv.offset(i as isize)) };
                println!("- {}", cstr.to_str().unwrap());
            }
        }
        _ => println!("Args were not specified"),
    }

    MPI_SUCCESS
}

#[no_mangle]
pub extern "C" fn MPI_Finalize() -> c_int {
    println!("Buh-bye from eMPIRe!");

    MPI_SUCCESS
}
