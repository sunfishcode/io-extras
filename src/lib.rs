//! Non-owning unsafe I/O
//!
//! This is a very low-level library that just serves to factor out
//! platform-specific code for working with platform-specific handle types such
//! as [`RawFd`], [`RawHandle`], and [`RawSocket`].
//!
//! Being non-owning, these handles operate much like raw pointers in Rust. They
//! are considered safe to construct, but unsafe to use in any way that depends on
//! the resource they point to.
//!
//! [`RawFd`]: https://doc.rust-lang.org/std/os/unix/io/type.RawFd.html
//! [`RawHandle`]: https://doc.rust-lang.org/std/os/windows/io/type.RawHandle.html
//! [`RawSocket`]: https://doc.rust-lang.org/std/os/windows/io/type.RawSocket.html

#![deny(missing_docs)]
#![cfg_attr(can_vector, feature(can_vector))]
#![cfg_attr(write_all_vectored, feature(write_all_vectored))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

#[cfg(windows)]
mod raw_handle_or_socket;
mod read_write;
mod unsafe_handle;

#[cfg(windows)]
pub use raw_handle_or_socket::{AsRawHandleOrSocket, RawHandleOrSocket};
#[cfg(not(windows))]
pub use read_write::AsRawReadWriteFd;
#[cfg(windows)]
pub use read_write::AsRawReadWriteHandleOrSocket;
pub use read_write::AsUnsafeReadWriteHandle;
pub use unsafe_handle::{
    AsUnsafeFile, AsUnsafeHandle, AsUnsafeSocket, FromUnsafeFile, FromUnsafeSocket, IntoUnsafeFile,
    IntoUnsafeHandle, IntoUnsafeSocket, UnsafeFile, UnsafeHandle, UnsafeReadable, UnsafeSocket,
    UnsafeWriteable,
};
