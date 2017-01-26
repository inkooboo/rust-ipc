//!

use libc::*;
use std::ffi::CString;

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
    pub fn new(name: &str, create_mode: CreateMode, access_mode: AccessMode, permissions: Permissions) -> Result<Handle, String> {
        let cmode = match create_mode {
            CreateMode::CreateOnly => O_CREAT | O_EXCL,
            CreateMode::OpenOrCreate => O_CREAT,
            CreateMode::OpenOnly => 0,
        };
        let amode = match access_mode {
            AccessMode::ReadOnly => O_RDONLY,
            AccessMode::ReadWrite => O_RDWR,
        };
        let perm = match permissions {
            Permissions::Default => S_IRWXG,
            // TODO
        };
        let cstr = match CString::new(name) {
            Err(_) => return Err(format!("Unable to convert to CString: {}", name)),
            Ok(val) => val,
        };
        let fd = unsafe { shm_open(cstr.as_ptr(), cmode | amode, perm) };
        match fd {
            -1 => Err(format!("Unable to open/create shared memory object: {}", name)),
            _ => Ok(Handle {shm_fd: fd, name: cstr, access_mode: access_mode}),
        }
    }
    pub fn remove(name: &str) -> bool {
        let cstr = match CString::new(name) {
            Err(_) => return false,
            Ok(val) => val,
        };
        match unsafe { shm_unlink(cstr.as_ptr()) } {
            -1 => false,
            _ => true,
        }
    }
    pub fn name(&self) -> String {
        self.name.to_string_lossy().into_owned()
    }
    pub fn native_handle(&self) -> c_int {
        self.shm_fd
    }
    pub fn access_mode(&self) -> &AccessMode {
        &self.access_mode
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { close(self.shm_fd) };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn breath_test_handle() {
        use super::*;
        let name = "handleBreathTest";
        {
            let handle = Handle::new(name, CreateMode::CreateOnly, AccessMode::ReadWrite, Permissions::Default).unwrap();
            assert_eq!(handle.name(), name);
            match handle.access_mode() {
                &AccessMode::ReadWrite => {},
                _ => assert!(false, "wrong access mode"),
            };
            assert!(handle.native_handle() >= 0);
        }
        let removed = Handle::remove(name);
        assert!(removed);
    }
}
