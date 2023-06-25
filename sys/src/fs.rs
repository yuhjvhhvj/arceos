use axfs::fops::{FileAttr, OpenOptions};
use axfs::fops::File;
use axfs::api::ReadDir;
use libax::fs::FileType;
use alloc::string::String;
use axbase::AxError;
use alloc::boxed::Box;

#[allow(dead_code)]
pub struct StdDirEntry {
    path: String,
    fname: String,
    ftype: FileType,
}

impl StdDirEntry {
    fn new(path: String, fname: String, ftype: FileType) -> Self {
        Self { path, fname, ftype }
    }
}

#[no_mangle]
pub fn sys_read_dir(path: &str) -> usize {
    let rd = axfs::api::read_dir(path).unwrap();
    let ptr = Box::leak(Box::new(rd));
    ptr as *mut ReadDir as usize
}

#[no_mangle]
pub unsafe fn sys_read_dir_next(handle: usize)
    -> Option<Result<StdDirEntry, AxError>>
{
    let ptr = handle as *mut ReadDir;
    if let Some(Ok(ref de)) = ptr.as_mut().unwrap().next() {
        return Some(Ok(StdDirEntry::new(de.path(), de.file_name(), de.file_type())));
    }
    None
}

#[no_mangle]
pub fn sys_stat(path: &str) -> Result<FileAttr, AxError> {
    axfs::api::File::open(path)?.get_attr()
}

#[no_mangle]
pub fn sys_open(path: &str, opts: u32) -> Result<usize, AxError> {
    libax::info!("sys_open... {} {:X}", path, opts);
    let opts = OpenOptions::from_flags(opts);
    libax::info!("sys_open opts {:?}", opts);
    let f = File::open(path, &opts)?;
    let ptr = Box::leak(Box::new(f));
    Ok(ptr as *mut File as usize)
}

#[no_mangle]
pub fn sys_write(handle: usize, buf: &[u8]) -> usize {
    let f = handle as *mut File;
    unsafe {
        f.as_mut().unwrap().write(buf).unwrap()
    }
}

#[no_mangle]
pub fn sys_read(handle: usize, buf: &mut [u8]) -> usize {
    let f = handle as *mut File;
    unsafe {
        f.as_mut().unwrap().read(buf).unwrap()
    }
}

#[no_mangle]
pub fn sys_mkdir(path: &str) -> Result<(), AxError> {
    axfs::api::create_dir(path)
}

#[no_mangle]
pub fn sys_rmdir(path: &str) -> Result<(), AxError> {
    axfs::api::remove_dir(path)
}

#[no_mangle]
pub fn sys_unlink(path: &str) -> Result<(), AxError> {
    axfs::api::remove_file(path)
}

#[no_mangle]
pub fn sys_getcwd() -> Result<String, AxError> {
    libax::env::current_dir()
}

#[no_mangle]
pub fn sys_chdir(path: &str) -> Result<(), AxError> {
    libax::env::set_current_dir(path)
}

#[no_mangle]
pub fn sys_close_file(handle: usize) {
    unsafe { core::ptr::drop_in_place(handle as *mut File) }
}

#[no_mangle]
pub fn sys_close_dir(handle: usize) {
    unsafe { core::ptr::drop_in_place(handle as *mut ReadDir) }
}
