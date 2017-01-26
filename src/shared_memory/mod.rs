//!

extern crate libc

use libc;
use std::ffi::CString;

extern {
    fn shm_open(name: *const c_char, flag: c_int, mode: mode_t) -> c_int;
    fn ftruncate(fd: c_int, length: off_t) -> c_int;
    fn shm_unlink(name: *const c_char) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mmap(addr: *c_void, length: size_t, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *c_void;
    fn munmap(addr: *c_void, length: size_t) -> c_int;
}

pub enum CreateMode {
    CreateOnly,
    OpenOrCreate,
    OpenOnly,
}

pub enum AccessMode {
    ReadOnly,
    ReadWrite,
}

pub enum Permissions {
    Default,
    // TODO
}

pub struct Handle {
    shm_fd: c_int,
    name: CString,
    access_mode: AccessMode,
}

impl Handle {
    fn new(name: &CString, create_mode: CreateMode, access_mode: AccessMode, permissions: Premissions) -> Result<Handle, String> {
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
            let fd = shm_open(name.as_ptr(), cmode | amode, perm);
            match fd {
                -1 => Err("Unable to open/create shared memory object: ".name),
                _ => Ok(Handle {shm_fd: fd, name: name, access_mode: access_mode}),
            }
        }
    }
    fn remove(name: &CString) -> bool {
        unsafe {
            match shm_unlink(name.as_ptr()) {
                -1 => false,
                _ => true,
            }
        }
    }
    fn name(&self) -> &CString {
        self.name
    }
    fn native_handle(&self) -> c_int {
        self.shm_fd
    }
    fn access_mode(&self) -> AccessMode {
        self.access_mode
    }
}

impl Drop for Handle {
    fn drop(&self) {
        unsafe {
            close(self.shm_fd);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn handleBreathTest() {
        let name = "handleBreathTest";
        {
            let handle = Handle::new(name, CreateMode::CreateOnly, AccessMode::ReadWrite, Permissions::Default);
            assert_eq!(handle.name(), name);
            assert_eq!(handle.access_mode(), AccessMode::ReadWrite);
        }
        let removed = Handle::remove(name);
        assert!(removed);
    }
}
