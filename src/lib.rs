//! Non-owning unsafe I/O
//!
//! In table form, the main types and traits provided by this crate are:
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
//! | [`RawHandleOrSocket`] | [`AsRawHandleOrSocket`] | [`IntoRawHandleOrSocket`] | [`FromRawHandleOrSocket`] |
//! |                  |                    |                      |                      |
//! | [`UnsafeFile`]   | [`AsUnsafeFile`]   | [`IntoUnsafeFile`]   | [`FromUnsafeFile`]   |
//! | [`UnsafeSocket`] | [`AsUnsafeSocket`] | [`IntoUnsafeSocket`] | [`FromUnsafeSocket`] |
//! | [`UnsafeHandle`] | [`AsUnsafeHandle`] | [`IntoUnsafeHandle`] | [`FromUnsafeHandle`] |
//!
//! A brief explanation of the naming convention:
//!
//! "Raw" is for platform-specific types and traits, such as `std`'s [`RawFd`],
//! [`AsRawFd`], [`RawHandle`], [`AsRawHandle`], and similar, as well as
//! `unsafe-io`'s [`RawHandleOrSocket`], [`AsRawHandleOrSocket`], and similar.
//! "Handle" in this context means a Windows [`HANDLE`].
//!
//! "Unsafe" is for minimal platform-independent abstractions on top of the
//! platform-specific types, such as [`UnsafeHandle`] (abstracts over `RawFd`
//! and `RawHandleOrSocket`), [`UnsafeFile`] (abstracts over `RawFd` and
//! `RawHandle`), and [`UnsafeSocket`] (abstracts over `RawFd` and
//! `RawSocket`). "Handle" in this context means any kind of I/O handle.
//!
//! # Ownership Guarantees
//!
//! The `AsUnsafe*` and `IntoUnsafe*` trait functions return non-owning handles,
//! however these traits do require types that implement them to guarantee that
//! they own the handles. This differs from their `AsRaw*` and `IntoRaw*`
//! counterparts, which do not require such a guarantee. This crate defines an
//! [`OwnsRaw`] trait which is unsafe to implement and which allows types to
//! declare that they own the handles they hold, allowing them to opt into the
//! blanket `AsUnsafe*` and `IntoUnsafe*` implementations. See
//! [rust-lang/rust#76969] for further background.
//!
//! # Additional Utilities
//!
//! This crates also defines several additional utilities:
//!
//!  - `UnsafeHandle` has functions [`as_readable`] and [`as_writeable`] which
//!    return similar non-owning types [`UnsafeReadable`] and
//!    [`UnsafeWriteable`], respectively, which implement [`Read`] and
//!    [`Write`].
//!
//!  - `AsUnsafeFile` has functions [`as_file_view`], `as_pipe_reader_view`,
//!    and `as_pipe_writer_view` (enable [the `os_pipe` feature]), and
//!    `AsUnsafeSocket` has functions [`as_tcp_stream_view`],
//!    [`as_tcp_listener_view`], [`as_udp_socket_view`], and
//!    [`as_unix_stream_view`] (on [`unix`] platforms), for obtaining a
//!    temporary [view] of a handle as various higher-level types.
//!
//!  - `AsUnsafeHandle` has an [`eq_handle`] function, `AsUnsafeFile` has an
//!    [`eq_file`] function, and `AsUnsafeSocket` has an [`eq_socket`]
//!    function, for comparing whether two types hold the same handle, and are
//!    safe to call.
//!
//!  - `FromUnsafeFile` has a [`from_filelike`] function, and
//!    `FromUnsafeSocket` has a [`from_socketlike`] function, for constructing
//!    a type from any type which implements [`IntoUnsafeFile`], or
//!    [`IntoUnsafeSocket`], respectively. Unlike in the corresponding "Raw"
//!    traits, these functions are safe to call.
//!
//!  - [`AsUnsafeReadWriteHandle`], [`AsRawReadWriteFd`], and
//!    [`AsRawReadWriteHandleOrSocket`] are traits for working with types that
//!    implement both [`Read`] and [`Write`] and may contain either one handle
//!    (such as a socket) or two (such as stdin and stdout, or a pair of
//!    pipes).
//!
//! # Examples
//!
//! To use the [`AsUnsafeHandle`] trait:
//!
//! ```rust
//! use unsafe_io::AsUnsafeHandle;
//!
//! // Open a normal file.
//! let file = std::fs::File::open("Cargo.toml").unwrap();
//!
//! // Extract the handle.
//! let unsafe_handle = file.as_unsafe_handle();
//! ```
//!
//! Many types implement `AsUnsafeHandle`, however very few types implement
//! `FromUnsafeHandle`. Most types implement only `FromUnsafeFile` or
//! `FromUnsafeSocket` instead. For an example that extracts a handle and
//! constructs a new file, we use the [`IntoUnsafeFile`] and [`AsUnsafeFile`]
//! traits:
//!
//! ```rust
//! use unsafe_io::{FromUnsafeFile, IntoUnsafeFile};
//!
//! // Open a normal file.
//! let file = std::fs::File::open("Cargo.toml").unwrap();
//!
//! // Consume the file, returning the handle.
//! let unsafe_file = file.into_unsafe_file();
//!
//! // Construct a new file with the handle.
//! let file = unsafe { std::fs::File::from_unsafe_file(unsafe_file) };
//! ```
//!
//! To implement the [`AsUnsafeHandle`] trait for a type, implement the
//! [`AsRawFd`] trait for Posix-ish platforms and the [`AsRawHandleOrSocket`]
//! trait for Windows platforms, and then implement [`OwnsRaw`]:
//!
//! ```rust
//! #[cfg(not(windows))]
//! use unsafe_io::os::posish::{AsRawFd, RawFd};
//! #[cfg(windows)]
//! use unsafe_io::os::windows::{AsRawHandleOrSocket, RawHandleOrSocket};
//! use unsafe_io::OwnsRaw;
//!
//! struct MyType(std::fs::File);
//!
//! #[cfg(not(windows))]
//! impl AsRawFd for MyType {
//!     fn as_raw_fd(&self) -> RawFd {
//!         self.0.as_raw_fd()
//!     }
//! }
//!
//! #[cfg(windows)]
//! impl AsRawHandleOrSocket for MyType {
//!     fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
//!         self.0.as_raw_handle_or_socket()
//!     }
//! }
//!
//! // Safety: `MyType` owns its handle.
//! unsafe impl OwnsRaw for MyType {}
//! ```
//!
//! This requires `unsafe` because implementing [`OwnsRaw`] asserts that the
//! type owns its raw handle. See the [char-device] crate for a complete
//! example.
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
//! [`FromRawHandleOrSocket`]: https://docs.rs/unsafe-io/latest/x86_64-pc-windows-msvc/unsafe_io/trait.FromRawHandleOrSocket.html
//! [`AsRawReadWriteHandleOrSocket`]: https://docs.rs/unsafe-io/latest/x86_64-pc-windows-msvc/unsafe_io/trait.AsRawReadWriteHandleOrSocket.html
//! [`AsRawReadWriteFd`]: https://docs.rs/unsafe-io/latest/unsafe_io/trait.AsRawReadWriteFd.html
//! [`HANDLE`]: https://doc.rust-lang.org/std/os/windows/raw/type.HANDLE.html
//! [`Read`]: std::io::Read
//! [`Write`]: std::io::Write
//! [`as_readable`]: UnsafeHandle::as_readable
//! [`as_writeable`]: UnsafeHandle::as_writeable
//! [`as_file_view`]: AsUnsafeFile::as_file_view
//! [`as_tcp_stream_view`]: AsUnsafeSocket::as_tcp_stream_view
//! [`as_tcp_listener_view`]: AsUnsafeSocket::as_tcp_listener_view
//! [`as_udp_socket_view`]: AsUnsafeSocket::as_udp_socket_view
//! [`as_unix_stream_view`]: https://docs.rs/unsafe-io/latest/unsafe_io/trait.AsUnsafeSocket.html#method.as_unix_stream_view
//! [`eq_handle`]: AsUnsafeHandle::eq_handle
//! [`eq_file`]: AsUnsafeFile::eq_file
//! [`eq_socket`]: AsUnsafeSocket::eq_socket
//! [`from_handlelike`]: FromUnsafeHandle::from_handlelike
//! [`from_filelike`]: FromUnsafeFile::from_filelike
//! [`from_socketlike`]: FromUnsafeSocket::from_socketlike
//! [rust-lang/rust#76969]: https://github.com/rust-lang/rust/issues/76969
//! [char-device]: https://crates.io/crates/char-device/
//! [the `os_pipe` feature]: https://docs.rs/crate/unsafe-io/latest/features#os_pipe
//! [`unix`]: https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html
//! [view]: View

// This crate is very minimal and doesn't do anything except define types and
// traits and implement them with trivial inline implementations. As an
// experiment, let's throw ludicrous amounts of checks at it and see how it
// goes. As always, exercise discretion: if fixing a warning would make the
// code worse, disable the warning instead.
#![deny(
    future_incompatible,
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused,
    warnings,
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    invalid_html_tags,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_copy_implementations,
    missing_crate_level_docs,
    non_ascii_idents,
    pointer_structural_match,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unaligned_references,
    unreachable_pub,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]
#![allow(missing_doc_code_examples)]
#![deny(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction
)]
#![allow(clippy::blanket_clippy_restriction_lints, clippy::implicit_return)]
#![cfg_attr(can_vector, feature(can_vector))]
#![cfg_attr(write_all_vectored, feature(write_all_vectored))]
#![cfg_attr(target_os = "wasi", feature(wasi_ext))]

mod owns_raw;
mod read_write;
mod unsafe_handle;

pub use owns_raw::OwnsRaw;
pub use read_write::{AsUnsafeReadWriteHandle, ReadHalf, WriteHalf};
pub use unsafe_handle::{
    AsUnsafeFile, AsUnsafeHandle, AsUnsafeSocket, FromUnsafeFile, FromUnsafeHandle,
    FromUnsafeSocket, IntoUnsafeFile, IntoUnsafeHandle, IntoUnsafeSocket, UnsafeFile, UnsafeHandle,
    UnsafeReadable, UnsafeSocket, UnsafeWriteable, View,
};

pub mod os;
