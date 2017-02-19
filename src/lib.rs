pub mod types;
pub mod shared_memory;

#[cfg(unix)]
extern crate libc;
#[cfg(unix)] #[path = "posix.rs"] mod detail;

#[cfg(windows)]
extern crate kernel32;
#[cfg(windows)]
extern crate winapi;

#[cfg(windows)] #[path = "win32.rs"] mod detail;

