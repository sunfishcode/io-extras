//! Non-owning unsafe I/O
//!
//! A brief explanation of the naming convention:
//!
//! `Raw` is for platform-specific types, such as `std`'s [`RawFd`],
//! [`AsRawFd`], [`RawHandle`], [`AsRawHandle`], and similar, as well as
//! `unsafe-io`'s [`RawHandleOrSocket`], [`AsRawHandleOrSocket`], and similar.
//! `Handle` in this context means a Windows `HANDLE`.
//!
//! `Unsafe` is for (mostly) platform-independent abstractions on top of the
//! platform-specific types, such as [`UnsafeHandle`] (abstracts over `RawFd`
//! and `RawHandleOrSocket`), [`UnsafeFile`] (abstracts over `RawFd` and
//! `RawHandle`), and `[UnsafeSocket`] (abstracts over `RawFd` and `RawSocket`).
//! [`UnsafeReadable`] and [`UnsafeWriteable`] are similar to `UnsafeHandle`,
//! but require `unsafe` to construct, so that they can safely implement `Read`
//! and `Write`. `Handle` in this context means a value representing an open
//! I/O resource.
//!
//! `ReadWrite` describes types which support both `Read` and `Write` and may
//! contain either one or two handles.
//!
//! [`RawFd`]: https://doc.rust-lang.org/std/os/unix/io/type.RawFd.html
//! [`AsRawFd`]: https://doc.rust-lang.org/std/os/unix/io/trait.AsRawFd.html
//! [`RawHandle`]: https://doc.rust-lang.org/std/os/windows/io/type.RawHandle.html
//! [`AsRawHandle`]: https://doc.rust-lang.org/std/os/windows/io/trait.AsRawHandle.html
//! [`RawHandleOrSocket`]: https://docs.rs/unsafe-io/latest/unsafe_io/struct.RawHandleOrSocket.html
//! [`AsRawHandleOrSocket`]: https://docs.rs/unsafe-io/latest/unsafe_io/trait.AsRawHandleOrSocket.html

#![deny(missing_docs)]
#![cfg_attr(can_vector, feature(can_vector))]
#![cfg_attr(write_all_vectored, feature(write_all_vectored))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

#[cfg(windows)]
mod raw_handle_or_socket;
mod read_write;
mod unsafe_handle;

#[cfg(windows)]
pub use raw_handle_or_socket::{AsRawHandleOrSocket, IntoRawHandleOrSocket, RawHandleOrSocket};
#[cfg(not(windows))]
pub use read_write::AsRawReadWriteFd;
#[cfg(windows)]
pub use read_write::AsRawReadWriteHandleOrSocket;
pub use read_write::AsUnsafeReadWriteHandle;
pub use unsafe_handle::{
    AsUnsafeFile, AsUnsafeHandle, AsUnsafeSocket, FromUnsafeFile, FromUnsafeSocket, IntoUnsafeFile,
    IntoUnsafeHandle, IntoUnsafeSocket, UnsafeFile, UnsafeHandle, UnsafeReadable, UnsafeSocket,
    UnsafeWriteable, View,
};
