//! Public APIs and types for [ArceOS] modules
//!
//! [ArceOS]: https://github.com/rcore-os/arceos

#![no_std]
#![feature(ip_in_core)]
#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]
#![feature(strict_provenance)] // TODO: remove this feature
#![allow(unused_imports)]

#[cfg(any(
    feature = "alloc",
    feature = "fs",
    feature = "net",
    feature = "multitask",
    feature = "dummy-if-not-enabled"
))]
extern crate alloc;
extern crate axruntime;

#[macro_use]
mod macros;
mod imp;

pub mod legacy;

pub use axerrno::{AxError, AxResult};

/// Platform-specific constants and parameters.
pub mod config {
    pub use axconfig::*;
}

/// System operations.
pub mod sys {
    define_api! {
        pub fn ax_terminate() -> !;
    }
}

/// Time-related operations.
pub mod time {
    define_api_type! {
        pub type AxTimeValue;
    }

    define_api! {
        pub fn ax_current_time() -> AxTimeValue;
    }
}

/// Memory management.
pub mod mem {
    use core::{alloc::Layout, ptr::NonNull};

    define_api! {
        @cfg "alloc";
        pub fn ax_alloc(layout: Layout) -> Option<NonNull<u8>>;
        pub fn ax_dealloc(ptr: NonNull<u8>, layout: Layout);
    }
}

/// Standard input and output.
pub mod stdio {
    use core::fmt;
    define_api! {
        pub fn ax_console_read_byte() -> Option<u8>;
        pub fn ax_console_write_bytes(buf: &[u8]) -> crate::AxResult<usize>;
        pub fn ax_console_write_fmt(args: fmt::Arguments) -> fmt::Result;
    }
}

/// Multi-threading management.
pub mod task {
    define_api_type! {
        @cfg "multitask";
        pub type AxTaskHandle;
        pub type AxWaitQueueHandle;
    }

    define_api! {
        pub fn ax_sleep_until(deadline: crate::time::AxTimeValue);
        pub fn ax_yield_now();
        pub fn ax_exit(exit_code: i32) -> !;
    }

    define_api! {
        @cfg "multitask";
        pub fn ax_current_task_id() -> u64;
        pub fn ax_spawn(
            f: impl FnOnce() + Send + 'static,
            name: alloc::string::String,
            stack_size: usize
        ) -> AxTaskHandle;
        pub fn ax_wait_for_exit(task: AxTaskHandle) -> Option<i32>;
        pub fn ax_set_current_priority(prio: isize) -> crate::AxResult;

        pub fn ax_wait_queue_wait(
            wq: &AxWaitQueueHandle,
            until_condition: impl Fn() -> bool,
            timeout: Option<core::time::Duration>,
        ) -> bool;
        pub fn ax_wait_queue_wake(wq: &AxWaitQueueHandle, count: u32);
    }
}

/// Filesystem manipulation operations.
pub mod fs {
    use crate::AxResult;

    define_api_type! {
        @cfg "fs";
        pub type AxFileHandle;
        pub type AxDirHandle;
        pub type AxOpenOptions;
        pub type AxFileAttr;
        pub type AxFileType;
        pub type AxFilePerm;
        pub type AxDirEntry;
        pub type AxSeekFrom;
    }

    define_api! {
        @cfg "fs";
        pub fn ax_open_file(path: &str, opts: &AxOpenOptions) -> AxResult<AxFileHandle>;
        pub fn ax_open_dir(path: &str, opts: &AxOpenOptions) -> AxResult<AxDirHandle>;

        pub fn ax_read_file(file: &mut AxFileHandle, buf: &mut [u8]) -> AxResult<usize>;
        pub fn ax_read_file_at(file: &AxFileHandle, offset: u64, buf: &mut [u8]) -> AxResult<usize>;
        pub fn ax_write_file(file: &mut AxFileHandle, buf: &[u8]) -> AxResult<usize>;
        pub fn ax_write_file_at(file: &AxFileHandle, offset: u64, buf: &[u8]) -> AxResult<usize>;
        pub fn ax_truncate_file(file: &AxFileHandle, size: u64) -> AxResult;
        pub fn ax_flush_file(file: &AxFileHandle) -> AxResult;
        pub fn ax_seek_file(file: &mut AxFileHandle, pos: AxSeekFrom) -> AxResult<u64>;
        pub fn ax_file_attr(file: &AxFileHandle) -> AxResult<AxFileAttr>;

        pub fn ax_read_dir(dir: &mut AxDirHandle, dirents: &mut [AxDirEntry]) -> AxResult<usize>;
        pub fn ax_create_dir(path: &str) -> AxResult;
        pub fn ax_remove_dir(path: &str) -> AxResult;
        pub fn ax_remove_file(path: &str) -> AxResult;

        pub fn ax_current_dir() -> AxResult<alloc::string::String>;
        pub fn ax_set_current_dir(path: &str) -> AxResult;
    }
}

/// Networking primitives for TCP/UDP communication.
pub mod net {
    use crate::{io::AxPollState, AxResult};
    use core::net::SocketAddr;

    define_api_type! {
        @cfg "net";
        pub type AxTcpSocketHandle;
    }

    define_api! {
        @cfg "net";
        pub fn ax_tcp_socket() -> AxTcpSocketHandle;
        pub fn ax_tcp_socket_addr(socket: &AxTcpSocketHandle) -> AxResult<SocketAddr>;
        pub fn ax_tcp_peer_addr(socket: &AxTcpSocketHandle) -> AxResult<SocketAddr>;
        pub fn ax_tcp_set_nonblocking(socket: &AxTcpSocketHandle, nonblocking: bool) -> AxResult;

        pub fn ax_tcp_connect(handle: &AxTcpSocketHandle, addr: SocketAddr) -> AxResult;
        pub fn ax_tcp_bind(socket: &AxTcpSocketHandle, addr: SocketAddr) -> AxResult;
        pub fn ax_tcp_listen(socket: &AxTcpSocketHandle, _backlog: usize) -> AxResult;
        pub fn ax_tcp_accept(socket: &AxTcpSocketHandle) -> AxResult<(AxTcpSocketHandle, SocketAddr)>;

        pub fn ax_tcp_send(socket: &AxTcpSocketHandle, buf: &[u8]) -> AxResult<usize>;
        pub fn ax_tcp_recv(socket: &AxTcpSocketHandle, buf: &mut [u8]) -> AxResult<usize>;
        pub fn ax_tcp_poll(socket: &AxTcpSocketHandle) -> AxResult<AxPollState>;
        pub fn ax_tcp_shutdown(socket: &AxTcpSocketHandle) -> AxResult;

        pub fn ax_get_addr_info(domain_name: &str, port: Option<u16>) -> AxResult<alloc::vec::Vec<SocketAddr>>;
    }
}

pub mod io {
    define_api_type! {
        pub type AxPollState;
    }
}
