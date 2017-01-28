//!

#[cfg(not(windows))]
use libc::*;

#[cfg(windows)]
use kernel32::*;

#[cfg(windows)]
use winapi::*;

use std::ffi::CString;
use std::ptr;

#[cfg(not(windows))]
type FileHandle = c_int;


#[cfg(windows)]
type FileHandle = HANDLE;

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
    Others
}

pub struct Handle {
    shm_fd: FileHandle,
    name: CString,
    access_mode: AccessMode,
}

impl Handle {
    #[cfg(not(windows))]
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
            Permissions::User => S_IRWXU,
            Permissions::Group => S_IRWXG,
            Permissions::Others => S_IRWXO,
        };
        let cstr = match CString::new(name) {
            Err(_) => return Err(format!("Unable to convert to CString: {}", name)),
            Ok(val) => val,
        };
        let fd = unsafe { shm_open(cstr.as_ptr(), cmode | amode as c_int, perm as c_uint) };
        match fd {
            -1 => Err(format!("Unable to open/create shared memory object: {}", name)),
            _ => Ok(Handle {shm_fd: FileHandle::Posix(fd), name: cstr, access_mode: access_mode}),
        }
    }

    #[cfg(not(windows))]
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

    #[cfg(windows)]
    pub fn new(name: &str, create_mode: CreateMode, access_mode: AccessMode, permissions: Permissions) -> Result<Handle, String> {
        let cmode = match create_mode {
            CreateMode::CreateOnly => CREATE_NEW,
            CreateMode::OpenOrCreate => OPEN_ALWAYS,
            CreateMode::OpenOnly => OPEN_EXISTING,
        };
        let amode = match access_mode {
            AccessMode::ReadOnly => GENERIC_READ,
            AccessMode::ReadWrite => GENERIC_READ | GENERIC_WRITE,
        };
        let perm = match permissions {
	    // TODO
            _ => ptr::null_mut(),
        };
        let cstr = match CString::new(name) {
            Err(_) => return Err(format!("Unable to convert to CString: {}", name)),
            Ok(val) => val,
        };
        let fd = unsafe {
	    CreateFileA(cstr.as_ptr(),
	                amode,
	                FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
	                perm as *mut SECURITY_ATTRIBUTES,
	                cmode,
	                0,
	                ptr::null_mut() as *mut c_void)
	};
        if fd == INVALID_HANDLE_VALUE {
	    Err(format!("Unable to open/create shared memory object: {}", name))
	} else {
            Ok(Handle {shm_fd: fd, name: cstr, access_mode: access_mode})
	}
    }

    #[cfg(windows)]
    pub fn remove(name: &str) -> bool {
        let cstr = match CString::new(name) {
            Err(_) => return false,
            Ok(val) => val,
        };
        match unsafe { DeleteFileA(cstr.as_ptr()) } {
            0 => false,
            _ => true,
        }
    }

    pub fn native_handle(&self) -> FileHandle {
        self.shm_fd
    }

    pub fn name(&self) -> String {
        self.name.to_string_lossy().into_owned()
    }

    pub fn access_mode(&self) -> &AccessMode {
        &self.access_mode
    }
}

#[cfg(not(windows))]
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { let _ = close(self.shm_fd); }
    }
}

#[cfg(windows)]
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe { let _ = CloseHandle(self.shm_fd); }
    }
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
