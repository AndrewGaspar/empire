extern crate conv;
extern crate empire;
#[macro_use]
extern crate itertools;

// Status contains the macro for mpitry, so list first.
#[macro_use]
mod status;

// Contain MPI function definitions
pub mod comm;
pub mod init;
pub mod port;
pub mod spawn;

// Supporting modules
mod constants;
mod handles;

// Commonly used supporting routines
pub use init::universe;
pub use status::Error;

#[cfg(windows)]
mod windows;
