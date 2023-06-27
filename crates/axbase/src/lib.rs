//! Error code definition used by [ArceOS](https://github.com/rcore-os/arceos).
//!
//! It provides two error types and the corresponding result types:
//!
//! - [`AxError`] and [`AxResult`]: A generic error type similar to
//!   [`std::io::ErrorKind`].
//! - [`LinuxError`] and [`LinuxResult`]: Linux specific error codes defined in
//!   `errno.h`. It can be converted from [`AxError`].
//!
//! [`std::io::ErrorKind`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html

#![no_std]

mod linux_errno {
    include!(concat!(env!("OUT_DIR"), "/linux_errno.rs"));
}

pub use linux_errno::LinuxError;

/// The error type used by ArceOS.
///
/// Similar to [`std::io::ErrorKind`].
///
/// [`std::io::ErrorKind`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html
#[repr(i32)]
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AxError {
    /// A socket address could not be bound because the address is already in use elsewhere.
    AddrInUse,
    /// An entity already exists, often a file.
    AlreadyExists,
    /// Bad address.
    BadAddress,
    /// Bad internal state.
    BadState,
    /// The connection was refused by the remote server,
    ConnectionRefused,
    /// A non-empty directory was specified where an empty directory was expected.
    DirectoryNotEmpty,
    /// Data not valid for the operation were encountered.
    ///
    /// Unlike [`InvalidInput`], this typically means that the operation
    /// parameters were valid, however the error was caused by malformed
    /// input data.
    ///
    /// For example, a function that reads a file into a string will error with
    /// `InvalidData` if the file's contents are not valid UTF-8.
    ///
    /// [`InvalidInput`]: AxError::InvalidInput
    InvalidData,
    /// Invalid parameter/argument.
    InvalidInput,
    /// Input/output error.
    Io,
    /// The filesystem object is, unexpectedly, a directory.
    IsADirectory,
    /// Not enough space/cannot allocate memory.
    NoMemory,
    /// A filesystem object is, unexpectedly, not a directory.
    NotADirectory,
    /// The network operation failed because it was not connected yet.
    NotConnected,
    /// The requested entity is not found.
    NotFound,
    /// The operation lacked the necessary privileges to complete.
    PermissionDenied,
    /// Device or resource is busy.
    ResourceBusy,
    /// The underlying storage (typically, a filesystem) is full.
    StorageFull,
    /// An error returned when an operation could not be completed because an
    /// "end of file" was reached prematurely.
    UnexpectedEof,
    /// This operation is unsupported or unimplemented.
    Unsupported,
    /// The operation needs to block to complete, but the blocking operation was
    /// requested to not occur.
    WouldBlock,
    /// An error returned when an operation could not be completed because a
    /// call to `write()` returned [`Ok(0)`](Ok).
    WriteZero,
}

/// A specialized [`Result`] type with [`AxError`] as the error type.
pub type AxResult<T = ()> = Result<T, AxError>;

/// A specialized [`Result`] type with [`LinuxError`] as the error type.
pub type LinuxResult<T = ()> = Result<T, LinuxError>;

impl AxError {
    /// Returns the error description.
    pub fn as_str(&self) -> &'static str {
        use AxError::*;
        match *self {
            BadState => "Bad internal state",
            InvalidData => "Invalid data",
            Unsupported => "Operation not supported",
            UnexpectedEof => "Unexpected end of file",
            WriteZero => "Write zero",
            _ => LinuxError::from(*self).as_str(),
        }
    }
}

impl From<AxError> for LinuxError {
    fn from(e: AxError) -> Self {
        use AxError::*;
        match e {
            AddrInUse => LinuxError::EADDRINUSE,
            AlreadyExists => LinuxError::EEXIST,
            BadAddress | BadState => LinuxError::EFAULT,
            ConnectionRefused => LinuxError::ECONNREFUSED,
            DirectoryNotEmpty => LinuxError::ENOTEMPTY,
            InvalidInput | InvalidData => LinuxError::EINVAL,
            Io => LinuxError::EIO,
            IsADirectory => LinuxError::EISDIR,
            NoMemory => LinuxError::ENOMEM,
            NotADirectory => LinuxError::ENOTDIR,
            NotConnected => LinuxError::ENOTCONN,
            NotFound => LinuxError::ENOENT,
            PermissionDenied => LinuxError::EACCES,
            ResourceBusy => LinuxError::EBUSY,
            StorageFull => LinuxError::ENOSPC,
            Unsupported => LinuxError::ENOSYS,
            UnexpectedEof | WriteZero => LinuxError::EIO,
            WouldBlock => LinuxError::EAGAIN,
        }
    }
}

//
// Socket stuff
//
pub const AF_UNSPEC: i32 = 0;
pub const AF_INET: i32 = 2;

pub const SOCK_STREAM: i32 = 1;
pub const SOCK_DGRAM: i32 = 2;

//
// Time stuff
//
pub const NSEC_PER_SEC: u64 = 1_000_000_000;
pub const CLOCK_REALTIME: u64 = 1;
pub const CLOCK_MONOTONIC: u64 = 4;

/// `timespec` is used by `clock_gettime` to retrieve the
/// current time
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct timespec {
    /// seconds
    pub tv_sec: i64,
    /// nanoseconds
    pub tv_nsec: i64,
}

pub enum HandleType {
    File(usize),
    ReadDir(usize),
    Socket(usize),
    Thread(usize),
}
