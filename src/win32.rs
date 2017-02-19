use kernel32::*;
use winapi::*;

use std::ptr;
use std::ffi::CString;

use types;

pub type FileHandle = HANDLE;

pub fn create_shm_handle(name: &CString,
                         create_mode: &types::CreateMode,
                         access_mode: &types::AccessMode,
                         permissions: &types::Permissions) -> Option<FileHandle> {
    let cmode = match create_mode {
        &types::CreateMode::CreateOnly => CREATE_NEW,
        &types::CreateMode::OpenOrCreate => OPEN_ALWAYS,
        &types::CreateMode::OpenOnly => OPEN_EXISTING,
    };
    let amode = match access_mode {
        &types::AccessMode::ReadOnly => GENERIC_READ,
        &types::AccessMode::ReadWrite => GENERIC_READ | GENERIC_WRITE,
    };
    let perm = match permissions {
    // TODO
        _ => ptr::null_mut(),
    };
    unsafe {
        let fd = CreateFileA(name.as_ptr(),
                             amode,
                             FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE,
                             perm as *mut SECURITY_ATTRIBUTES,
                             cmode,
                             0,
                             ptr::null_mut() as *mut c_void);
        if fd == INVALID_HANDLE_VALUE {
            None
        } else {
            Some(fd)
        }
    }
}

pub fn delete_file(name: &CString) -> bool {
    match unsafe { DeleteFileA(name.as_ptr()) } {
        0 => false,
        _ => true,
    }
}

pub fn close_handle(fd: FileHandle) {
    unsafe { let _ = CloseHandle(fd); }
}


pub fn truncate_file(fd: FileHandle, size: usize) -> bool {
    // TODO
    true
}

pub fn map_memory(fd: FileHandle, size: usize, access_mode: &types::AccessMode) -> Option<*mut u8> {
    // TODO
    None
}
