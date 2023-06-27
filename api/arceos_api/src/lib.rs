#![no_std]
#![feature(strict_provenance)]
#![feature(ip_in_core)]

extern crate alloc;

//pub use axruntime::lang_items::panic;

use axbase::timespec;
use axhal::time::current_time;
use core::alloc::Layout;

#[cfg(not(feature = "multitask"))]
use core::sync::atomic::AtomicU32;
#[cfg(not(feature = "multitask"))]
use core::time::Duration;

#[cfg(feature = "multitask")]
mod task;

#[cfg(feature = "fs")]
mod fs;

#[cfg(feature = "net")]
mod net;

#[cfg(feature = "use_ramfs")]
mod ramfs;

//
// These sys_* functions are used to support rust-std.
//

#[no_mangle]
pub fn sys_terminate() -> ! {
    axhal::misc::terminate()
}

#[no_mangle]
pub fn sys_alloc(layout: Layout) -> *mut u8 {
    if let Ok(ptr) = axalloc::global_allocator().alloc(layout.size(), layout.align()) {
        ptr as *mut u8
    } else {
        core::ptr::null::<*mut u8>() as *mut u8
    }
}

#[no_mangle]
pub unsafe fn sys_realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
    // SAFETY: the caller must ensure that the `new_size` does not overflow.
    // `layout.align()` comes from a `Layout` and is thus guaranteed to be valid.
    let new_layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };
    // SAFETY: the caller must ensure that `new_layout` is greater than zero.
    let new_ptr = sys_alloc(new_layout);
    if !new_ptr.is_null() {
        // SAFETY: the previously allocated block cannot overlap the newly allocated block.
        // The safety contract for `dealloc` must be upheld by the caller.
        unsafe {
            core::ptr::copy_nonoverlapping(ptr, new_ptr, core::cmp::min(layout.size(), new_size));
        }
        axalloc::global_allocator().dealloc(ptr as usize, layout.size(), layout.align())
    }
    new_ptr
}

#[no_mangle]
pub fn sys_dealloc(ptr: *mut u8, layout: Layout) {
    axalloc::global_allocator().dealloc(ptr as usize, layout.size(), layout.align())
}

#[no_mangle]
pub fn sys_console_write_bytes(bytes: &[u8]) {
    axhal::console::write_bytes(bytes);
}

#[no_mangle]
pub fn sys_console_read_bytes(bytes: &mut [u8]) -> usize {
    axhal::console::read_bytes(bytes)
}

#[no_mangle]
pub unsafe fn sys_clock_gettime(_clock_id: u64, tp: *mut timespec) -> i32 {
    let now = current_time();
    let ret = timespec {
        tv_sec: now.as_secs() as i64,
        tv_nsec: now.subsec_nanos() as i64,
    };
    unsafe {
        *tp = ret;
    }
    0
}

#[no_mangle]
pub fn sys_rand_u32() -> u32 {
    libax::rand::rand_u32()
}

//
// Just single task, i.e., NO 'multitask' feature
//
#[cfg(not(feature = "multitask"))]
#[no_mangle]
pub fn sys_futex_wait(_: &AtomicU32, _: u32, _: Option<Duration>) -> bool {
    true
}

#[cfg(not(feature = "multitask"))]
#[no_mangle]
pub fn sys_futex_wake(_: &AtomicU32, _: i32) {}

#[cfg(not(feature = "fs"))]
#[no_mangle]
pub fn sys_getcwd() -> Result<alloc::string::String, axbase::AxError> {
    Err(axbase::AxError::NotFound)
}
