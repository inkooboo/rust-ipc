#[cfg(not(windows))]
extern crate libc;

#[cfg(windows)]
extern crate kernel32;

#[cfg(windows)]
extern crate winapi;


pub mod shared_memory;

