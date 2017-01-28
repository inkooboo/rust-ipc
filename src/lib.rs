pub mod types;
pub mod shared_memory;

#[cfg(unix)] #[path = "posix.rs"] mod detail;
#[cfg(windows)] #[path = "win32.rs"] mod detail;

