//! Non-owning unsafe I/O
//!
//! A brief explanation of the naming convention:
//!
//! `Raw` is for platform-specific types and traits, such as `std`'s [`RawFd`],
//! [`AsRawFd`], [`RawHandle`], [`AsRawHandle`], and similar, as well as
//! `unsafe-io`'s [`RawHandleOrSocket`], [`AsRawHandleOrSocket`], and similar.
//! "Handle" in this context means a Windows [`HANDLE`].
//!
//! `Unsafe` is for minimal platform-independent abstractions on top of the
//! platform-specific types, such as [`UnsafeHandle`] (abstracts over `RawFd`
//! and `RawHandleOrSocket`), [`UnsafeFile`] (abstracts over `RawFd` and
//! `RawHandle`), and [`UnsafeSocket`] (abstracts over `RawFd` and `RawSocket`).
//! "Handle" in this context means any kind of I/O handle.
//!
//! In table form, the main types are:
//!
//! | Resource | Posix-ish type | Windows type          || Platform-independent types           |
//! | -------- | -------------- | --------------------- || ------------------------------------ |
//! | File     | [`RawFd`]      | [`RawHandle`]         || [`UnsafeHandle`] or [`UnsafeFile`]   |
//! | Pipe     | [`RawFd`]      | [`RawHandle`]         || [`UnsafeHandle`] or [`UnsafeFile`]   |
//! | Socket   | [`RawFd`]      | [`RawSocket`]         || [`UnsafeHandle`] or [`UnsafeSocket`] |
//! | Any      | [`RawFd`]      | [`RawHandleOrSocket`] || [`UnsafeHandle`]                     |
//!
//! and the main traits are:
//!
//! | Type             | `As` trait         | `Into` trait         | `From` trait         |
//! | ---------------- | ------------------ | -------------------- | -------------------- |
//! | [`RawFd`]        | [`AsRawFd`]        | [`IntoRawFd`]        | [`FromRawFd`]        |
//! | [`RawHandle`]    | [`AsRawHandle`]    | [`IntoRawHandle`]    | [`FromRawHandle`]    |
//! | [`RawSocket`]    | [`AsRawSocket`]    | [`IntoRawSocket`]    | [`FromRawSocket`]    |
//! | [`RawHandleOrSocket`] | [`AsRawHandleOrSocket`] | [`IntoRawHandleOrSocket`] | *     |
//! |                  |                    |                      |                      |
//! | [`UnsafeFile`]   | [`AsUnsafeFile`]   | [`IntoUnsafeFile`]   | [`FromUnsafeFile`]   |
//! | [`UnsafeSocket`] | [`AsUnsafeSocket`] | [`IntoUnsafeSocket`] | [`FromUnsafeSocket`] |
//! | [`UnsafeHandle`] | [`AsUnsafeHandle`] | [`IntoUnsafeHandle`] | *                    |
//!
//! \* These types do not have `From` traits.
//!
//! This crates also defines several additional utilities:
//!
//! [`UnsafeHandle`] has methods [`as_readable`] and [`as_writeable`] which
//! return similar non-owning types [`UnsafeReadable`] and [`UnsafeWriteable`],
//! respectively, which implement [`Read`] and [`Write`].
//!
//! [`AsUnsafeReadWriteHandle`], [`AsRawReadWriteFd`], and
//! [`AsRawReadWriteHandleOrSocket`], are traits for working with types that
//! implement both [`Read`] and [`Write`] and may contain either one handle
//! (such as a socket) or two (such as stdin and stdout, or a pair of pipes).
//!
//! [`RawFd`]: https://doc.rust-lang.org/std/os/unix/io/type.RawFd.html
//! [`AsRawFd`]: https://doc.rust-lang.org/std/os/unix/io/trait.AsRawFd.html
//! [`IntoRawFd`]: https://doc.rust-lang.org/std/os/unix/io/trait.IntoRawFd.html
//! [`FromRawFd`]: https://doc.rust-lang.org/std/os/unix/io/trait.FromRawFd.html
//! [`RawHandle`]: https://doc.rust-lang.org/std/os/windows/io/type.RawHandle.html
//! [`AsRawHandle`]: https://doc.rust-lang.org/std/os/windows/io/trait.AsRawHandle.html
//! [`IntoRawHandle`]: https://doc.rust-lang.org/std/os/windows/io/trait.IntoRawHandle.html
//! [`FromRawHandle`]: https://doc.rust-lang.org/std/os/windows/io/trait.FromRawHandle.html
//! [`RawSocket`]: https://doc.rust-lang.org/std/os/windows/io/type.RawSocket.html
//! [`AsRawSocket`]: https://doc.rust-lang.org/std/os/windows/io/trait.AsRawHandle.html
//! [`IntoRawSocket`]: https://doc.rust-lang.org/std/os/windows/io/trait.IntoRawHandle.html
//! [`FromRawSocket`]: https://doc.rust-lang.org/std/os/windows/io/trait.FromRawHandle.html
//! [`RawHandleOrSocket`]: https://docs.rs/unsafe-io/latest/x86_64-pc-windows-msvc/unsafe_io/struct.RawHandleOrSocket.html
//! [`AsRawHandleOrSocket`]: https://docs.rs/unsafe-io/latest/x86_64-pc-windows-msvc/unsafe_io/trait.AsRawHandleOrSocket.html
//! [`IntoRawHandleOrSocket`]: https://docs.rs/unsafe-io/latest/x86_64-pc-windows-msvc/unsafe_io/trait.IntoRawHandleOrSocket.html
//! [`AsRawReadWriteHandleOrSocket`]: https://docs.rs/unsafe-io/latest/x86_64-pc-windows-msvc/unsafe_io/trait.AsRawReadWriteHandleOrSocket.html
//! [`AsRawReadWriteFd`]: https://docs.rs/unsafe-io/latest/unsafe_io/trait.AsRawReadWriteFd.html
//! [`HANDLE`]: https://doc.rust-lang.org/std/os/windows/raw/type.HANDLE.html
//! [`Read`]: std::io::Read
//! [`Write`]: std::io::Write
//! [`as_readable`]: UnsafeHandle::as_readable
//! [`as_writeable`]: UnsafeHandle::as_writeable

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
