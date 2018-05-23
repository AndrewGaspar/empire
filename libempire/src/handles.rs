use empire::{Comm, universe::CommRegistration};
use std::{mem, ptr, sync::{Arc, Weak}};

pub enum CommHandle {
    NullComm,
    SystemComm(Weak<Comm>),
    UserComm(CommRegistration),
}

impl CommHandle {
    pub fn get(&self) -> Arc<Comm> {
        match self {
            &CommHandle::SystemComm(ref comm) => comm.upgrade()
                .expect("MPI is not currently initialized")
                .clone(),
            &CommHandle::UserComm(ref registration) => registration.unwrap(),
            &CommHandle::NullComm => panic!("MPI_COMM_NULL is not allowed in this routine."),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MPI_Comm {
    pub handle: *mut CommHandle,
}

impl MPI_Comm {
    pub fn new(handle: CommHandle) -> Self {
        Self {
            handle: Box::into_raw(Box::new(handle)),
        }
    }

    fn assert_non_null(&self) {
        assert!(
            self.handle != ptr::null_mut(),
            "NULL is not an allowed value for MPI_Comm. You may not have initialized MPI yet. Use \
             MPI_COMM_NULL instead."
        );
    }

    pub unsafe fn get(&self) -> Arc<Comm> {
        self.assert_non_null();
        (*self.handle).get()
    }

    pub unsafe fn is_null(&self) -> bool {
        self.assert_non_null();
        match &*self.handle {
            &CommHandle::NullComm => true,
            _ => false,
        }
    }

    pub unsafe fn free(&mut self) {
        Box::from_raw(self.handle as *mut CommHandle);
        self.handle = ptr::null_mut();
    }

    pub unsafe fn expect_user_comm(self) -> CommRegistration {
        let handle = mem::replace(&mut *self.handle, CommHandle::NullComm);

        match handle {
            CommHandle::UserComm(registration) => registration,
            CommHandle::SystemComm(_) => {
                panic!("System communicators are not compatible with this routine.")
            }
            CommHandle::NullComm => {
                panic!("The null communicator is not compatible with this routine.")
            }
        }
    }
}
