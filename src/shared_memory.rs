//! TODO docs

use std::ffi::CString;
use std::slice;

use types;

pub struct Handle {
    shm_fd: ::detail::FileHandle,
    name: String,
    access_mode: types::AccessMode,
}

impl Handle {
    pub fn new(name: &str,
               create_mode: types::CreateMode,
               access_mode: types::AccessMode,
               permissions: types::Permissions) -> Result<Handle, types::Error> {
        let cstr = match CString::new(name) {
            Err(_) => return Err(types::Error::ConvertString),
            Ok(val) => val,
        };
        match ::detail::create_shm_handle(&cstr, &create_mode, &access_mode, &permissions) {
            None => Err(types::Error::CreateFile),
            Some(fd) => Ok(Handle {shm_fd: fd, name: String::from(name), access_mode: access_mode}),
        }
    }

    pub fn remove(name: &str) -> Result<(), types::Error> {
        let cstr = match CString::new(name) {
            Err(_) => return Err(types::Error::ConvertString),
            Ok(val) => val,
        };
        match ::detail::delete_file(&cstr) {
            false => Err(types::Error::DeleteFile),
            true => Ok(()),
        }
    }

    pub fn native_handle(&self) -> ::detail::FileHandle {
        self.shm_fd
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn access_mode(&self) -> types::AccessMode {
        self.access_mode.clone()
    }
}

impl Drop for Handle {
    fn drop(&mut self) {
        ::detail::close_handle(self.shm_fd)
    }
}


pub struct MappedRegion {
    _handle: Handle,
    ptr: *mut u8,
    size: usize,
}

impl MappedRegion {
    pub fn new(handle: Handle, size: usize) -> Result<MappedRegion, types::Error> {
        if !::detail::truncate_file(handle.native_handle(), size) {
            return Err(types::Error::ResizeFile);
        }
        match ::detail::map_memory(handle.native_handle(), size, handle.access_mode()) {
            None => Err(types::Error::MapRegion),
            Some(ptr) => Ok(MappedRegion {_handle: handle, ptr: ptr, size: size}),
        }
    }

    pub fn memory(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.ptr, self.size) }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn breath_test_handle() {
        use super::*;
        use types;
        let name = "handleBreathTest";
        {
            let handle = Handle::new(name, types::CreateMode::CreateOnly, types::AccessMode::ReadWrite, types::Permissions::User).unwrap();
            assert_eq!(handle.name(), name);
            match handle.access_mode() {
                types::AccessMode::ReadWrite => {},
                _ => assert!(false, "wrong access mode"),
            };
            let _ = handle.native_handle();
        }
        match Handle::remove(name) {
            Ok(_) => {},
            Err(err) => assert!(false, "Can't remove file: {}", err),
        }
    }

    #[test]
    fn breath_test_mapped_region() {
        use super::*;
        use types;
        let name = "mappedRegionBreathTest";
        const SIZE: usize = 4096;
        let data: &[u8] = b"Hello Memory Mapped Area!";
        {
            let _ = Handle::remove(name);
            let handle = Handle::new(name, types::CreateMode::CreateOnly, types::AccessMode::ReadWrite, types::Permissions::User).unwrap();
            let mut mapped_region = MappedRegion::new(handle, SIZE).unwrap();
            assert_eq!(mapped_region.size(), SIZE);
            assert_eq!(mapped_region.memory().len(), SIZE);
            mapped_region.memory()[..data.len()].copy_from_slice(&data);
        }
        {
            let handle = Handle::new(name, types::CreateMode::OpenOnly, types::AccessMode::ReadWrite, types::Permissions::User).unwrap();
            let mut mapped_region = MappedRegion::new(handle, SIZE).unwrap();
            let mut buffer = [0u8; SIZE];
            buffer[..data.len()].copy_from_slice(&mapped_region.memory()[..data.len()]);
            assert_eq!(data, &buffer[..data.len()]);
        }

        match Handle::remove(name) {
            Ok(_) => {},
            Err(err) => assert!(false, "Can't remove file: {}", err),
        }
    }
}
