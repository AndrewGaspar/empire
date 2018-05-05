use std::ptr;

use empire::{Comm, Info};

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct MPI_Comm {
    pub handle: *const Option<Comm>,
}

impl MPI_Comm {
    pub unsafe fn get(&self) -> &Option<Comm> {
        assert!(
            self.handle != ptr::null(),
            "NULL is not an allowed value for MPI_Comm. You may not have initialized MPI yet. Use \
             MPI_COMM_NULL instead."
        );

        &*self.handle
    }

    pub unsafe fn is_null(&self) -> bool {
        self.get().is_none()
    }

    pub unsafe fn unwrap(&self) -> &Comm {
        (*self.handle)
            .as_ref()
            .expect("MPI_COMM_NULL is not allowed in this routine.")
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct MPI_Info {
    pub handle: *const Option<Info>,
}

impl MPI_Info {
    pub unsafe fn get(&self) -> &Option<Info> {
        assert!(
            self.handle != ptr::null(),
            "NULL is not an allowed value for MPI_Info. You may not have initialized MPI yet. Use \"
            MPI_INFO_NULL instead."
        );

        &*self.handle
    }

    pub unsafe fn is_null(&self) -> bool {
        self.get().is_none()
    }

    pub unsafe fn unwrap(&self) -> &Info {
        (*self.handle)
            .as_ref()
            .expect("MPI_INFO_NULL is not allowed in this routine.")
    }
}
