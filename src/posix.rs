extern crate libc;
use self::libc::*;

use std::ptr;
use std::ffi::CString;

use types;

pub type FileHandle = c_int;

pub fn create_shm_handle(name: &CString,
                         create_mode: &types::CreateMode,
                         access_mode: &types::AccessMode,
                         permissions: &types::Permissions) -> Option<FileHandle> {
    let cmode = match create_mode {
        &types::CreateMode::CreateOnly => O_CREAT | O_EXCL,
        &types::CreateMode::OpenOrCreate => O_CREAT,
        &types::CreateMode::OpenOnly => 0,
    };
    let amode = match access_mode {
        &types::AccessMode::ReadOnly => O_RDONLY,
        &types::AccessMode::ReadWrite => O_RDWR,
    };
    let perm = match permissions {
        &types::Permissions::User => S_IRWXU,
        &types::Permissions::Group => S_IRWXG,
        &types::Permissions::Everybody => S_IRWXO,
    };
    unsafe {
        let fd = shm_open(name.as_ptr(), cmode | amode as c_int, perm as c_uint);
        if fd == -1 {
            None
        } else {
            Some(fd)
        }
    }
}

pub fn delete_file(name: &CString) -> bool {
    match unsafe { shm_unlink(name.as_ptr()) } {
        -1 => false,
        _ => true,
    }
}

pub fn close_handle(fd: FileHandle) {
    unsafe { let _ = close(fd); }
}

pub fn truncate_file(fd: FileHandle, size: usize) -> bool {
    match unsafe { ftruncate(fd, size as i64) } {
        -1 => false,
        _ => true,
    }
}

pub fn map_memory(fd: FileHandle, size: usize, access_mode: types::AccessMode) -> Option<*mut u8> {
    let amode = match access_mode {
        types::AccessMode::ReadOnly => PROT_READ,
        types::AccessMode::ReadWrite => PROT_READ | PROT_WRITE,
    };
    let ptr = unsafe { mmap(ptr::null_mut() as *mut c_void, size, amode, MAP_SHARED, fd, 0) };
    if MAP_FAILED == ptr {
        None
    } else {
        Some(ptr as *mut u8)
    }
}


