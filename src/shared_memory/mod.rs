//!

extern crate libc

use libc::{c_int, c_char, c_void, size_t, mode_t, off_t, O_RDONLY, O_RDWR, O_CREAT, O_EXCL, S_IRWXG};
use std::ffi::CString;
use std::result;

extern {
    fn shm_open(name: *const c_char, flag: c_int, mode: mode_t) -> c_int;
    fn ftruncate(fd: c_int, length: off_t) -> c_int;
    fn shm_unlink(name: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mmap(addr: *c_void, length: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *c_void;
    fn munmap(addr: *c_void, length: size_t) -> c_int;
}

enum CreateMode {
    CreateOnly,
    OpenOrCreate,
    OpenOnly,
}

enum AccessMode {
    ReadOnly,
    ReadWrite,
}

enum Permissions {
    Default,
    // TODO
}

pub struct Handle {
    shm_fd: c_int,
    name: CString,
    size: u32,
    create_mode: CreateMode,
    access_mode: AccessMode,
}

impl Handle {
    fn new(name: &str, size: u32, create_mode: CreateMode, access_mode: AccessMode, permissions: Premissions) -> Result<Object, &'static str> {
        unsafe {
            let cmode = match create_mode {
                CreateOnly => O_CREAT | O_EXCL,
                OpenOrCreate => O_CREAT,
                OpenOnly => 0,
            };
            let amode = match access_mode {
                ReadOnly => O_RDONLY,
                ReadWrite => O_RDWR,
            };
            let perm = match permissions {
                Default => S_IRWXG,
                // TODO
            }
            let cname = CString::new(name);
            let rc = shm_open(cname.as_ptr(), cmode | amode, perm);
            match rc {
                -1 => "unable to open/create shared memory object",
            };
        }
    }
    fn name(&self) -> &CString {
        self.name
    }
    fn size(&self) -> u32 {
        self.size
    }
    fn nativeHandle(&self) -> c_int {
        self.c_int
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn breathTest() {
    }
}
