//! TODO docs

#[cfg(not(windows))]
use libc::*;
#[cfg(not(windows))]
type FileHandle = c_int;
#[cfg(not(windows))]
const INVALID_HANDLE_VALUE: FileHandle = -1;

#[cfg(windows)]
use kernel32::*;
#[cfg(windows)]
use winapi::*;
#[cfg(windows)]
use std::ptr;
#[cfg(windows)]
type FileHandle = HANDLE;

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
    User,
    Group,
    Everybody
}

pub struct Handle {
    shm_fd: FileHandle,
    name: String,
    access_mode: AccessMode,
}

impl Handle {
    pub fn new(name: &str, create_mode: CreateMode, access_mode: AccessMode, permissions: Permissions) -> Result<Handle, String> {
        let cstr = match CString::new(name) {
            Err(_) => return Err(format!("Unable to convert to CString: {}", name)),
            Ok(val) => val,
        };
        let fd = create_shm_handle_impl(&cstr, &create_mode, &access_mode, &permissions);
        if fd == INVALID_HANDLE_VALUE {
            Err(format!("Unable to open/create shared memory object: {}", name))
        } else {
            Ok(Handle {shm_fd: fd, name: String::from(name), access_mode: access_mode})
        }
    }

    pub fn remove(name: &str) -> bool {
        let cstr = match CString::new(name) {
            Err(_) => return false,
            Ok(val) => val,
        };
        remove_shm_impl(&cstr)
    }

    pub fn native_handle(&self) -> FileHandle {
        self.shm_fd
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn access_mode(&self) -> &AccessMode {
        &self.access_mode
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        close_shm_impl(self.shm_fd)
    }
}


#[cfg(not(windows))]
fn create_shm_handle_impl(name: &CString, create_mode: &CreateMode, access_mode: &AccessMode, permissions: &Permissions) -> FileHandle {
    let cmode = match create_mode {
        &CreateMode::CreateOnly => O_CREAT | O_EXCL,
        &CreateMode::OpenOrCreate => O_CREAT,
        &CreateMode::OpenOnly => 0,
    };
    let amode = match access_mode {
        &AccessMode::ReadOnly => O_RDONLY,
        &AccessMode::ReadWrite => O_RDWR,
    };
    let perm = match permissions {
        &Permissions::User => S_IRWXU,
        &Permissions::Group => S_IRWXG,
        &Permissions::Everybody => S_IRWXO,
    };
    unsafe { shm_open(name.as_ptr(), cmode | amode as c_int, perm as c_uint) }
}

#[cfg(not(windows))]
fn remove_shm_impl(name: &CString) -> bool {
    match unsafe { shm_unlink(name.as_ptr()) } {
        -1 => false,
        _ => true,
    }
}

#[cfg(not(windows))]
fn close_shm_impl(fd: FileHandle) {
    unsafe { let _ = close(fd); }
}

#[cfg(windows)]
fn create_shm_handle_impl(name: &CString, create_mode: &CreateMode, access_mode: &AccessMode, permissions: &Permissions) -> FileHandle {
    let cmode = match create_mode {
        &CreateMode::CreateOnly => CREATE_NEW,
        &CreateMode::OpenOrCreate => OPEN_ALWAYS,
        &CreateMode::OpenOnly => OPEN_EXISTING,
    };
    let amode = match access_mode {
        &AccessMode::ReadOnly => GENERIC_READ,
        &AccessMode::ReadWrite => GENERIC_READ | GENERIC_WRITE,
    };
    let perm = match permissions {
    // TODO
        _ => ptr::null_mut(),
    };
    unsafe {
        CreateFileA(name.as_ptr(),
                    amode,
                    FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                    perm as *mut SECURITY_ATTRIBUTES,
                    cmode,
                    0,
                    ptr::null_mut() as *mut c_void)
    }
}

#[cfg(windows)]
fn remove_shm_impl(name: &CString) -> bool {
    match unsafe { DeleteFileA(name.as_ptr()) } {
        0 => false,
        _ => true,
    }
}

#[cfg(windows)]
fn close_shm_impl(fd: FileHandle) {
    unsafe { let _ = CloseHandle(fd); }
}



#[cfg(test)]
mod tests {
    #[test]
    fn breath_test_handle() {
        use super::*;
        let name = "handleBreathTest";
        {
            let handle = Handle::new(name, CreateMode::CreateOnly, AccessMode::ReadWrite, Permissions::User).unwrap();
            assert_eq!(handle.name(), name);
            match handle.access_mode() {
                &AccessMode::ReadWrite => {},
                _ => assert!(false, "wrong access mode"),
            };
            let _ = handle.native_handle();
        }
        let removed = Handle::remove(name);
        assert!(removed);
    }
}
