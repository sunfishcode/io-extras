//! Define the [`OwnsRaw`] trait and implement it for well-known types.

#[cfg(feature = "os_pipe")]
use os_pipe::{PipeReader, PipeWriter};
#[cfg(unix)]
use std::os::unix::net::{UnixDatagram, UnixListener, UnixStream};
use std::{
    fs::File,
    io::{Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    net::{TcpListener, TcpStream, UdpSocket},
    process::{ChildStderr, ChildStdin, ChildStdout},
};

/// Assert that a type owns its raw file descriptor or handle.
///
/// The `AsRaw*` and `IntoRaw*` traits by themselves are not sufficient to
/// describe the ownership of the file descriptor or handle, as they aren't
/// unsafe to implement. See the discussion in [rust-lang/rust#76969] for
/// additional background. The [`UnsafeHandle`] type provided by this crate
/// serves as an implementation of [this suggestion].
///
/// `OwnsRaw` is a trait that types can implement to declare that they do
/// own their file descriptors or handles.
///
/// [rust-lang/rust#76969]: https://github.com/rust-lang/rust/issues/76969
/// [this suggestion]: https://github.com/rust-lang/rust/pull/76969#issuecomment-696275470
/// [`UnsafeHandle`]: crate::UnsafeHandle
///
/// # Safety
///
/// Types implementing `OwnsRaw` must own the file desctiptors or handles
/// they return in their `AsRaw*` and `IntoRaw*` implementations.
pub unsafe trait OwnsRaw {}

// Safety: The following types are all known to own their file descriptors or
// handles.
unsafe impl OwnsRaw for Stdin {}
unsafe impl OwnsRaw for StdinLock<'_> {}
unsafe impl OwnsRaw for Stdout {}
unsafe impl OwnsRaw for StdoutLock<'_> {}
unsafe impl OwnsRaw for Stderr {}
unsafe impl OwnsRaw for StderrLock<'_> {}
unsafe impl OwnsRaw for File {}
unsafe impl OwnsRaw for ChildStdin {}
unsafe impl OwnsRaw for ChildStdout {}
unsafe impl OwnsRaw for ChildStderr {}
unsafe impl OwnsRaw for TcpStream {}
unsafe impl OwnsRaw for TcpListener {}
unsafe impl OwnsRaw for UdpSocket {}
#[cfg(feature = "os_pipe")]
unsafe impl OwnsRaw for PipeReader {}
#[cfg(feature = "os_pipe")]
unsafe impl OwnsRaw for PipeWriter {}
#[cfg(unix)]
unsafe impl OwnsRaw for UnixStream {}
#[cfg(unix)]
unsafe impl OwnsRaw for UnixListener {}
#[cfg(unix)]
unsafe impl OwnsRaw for UnixDatagram {}
#[cfg(feature = "async-std")]
unsafe impl OwnsRaw for async_std::io::Stdin {}
#[cfg(feature = "async-std")]
unsafe impl OwnsRaw for async_std::io::Stdout {}
#[cfg(feature = "async-std")]
unsafe impl OwnsRaw for async_std::io::Stderr {}
#[cfg(feature = "async-std")]
unsafe impl OwnsRaw for async_std::fs::File {}
// async_std's `ChildStdin`, `ChildStdout`, and `ChildStderr` don't implement
// `AsRawFd` or `AsRawHandle`.
#[cfg(feature = "async-std")]
unsafe impl OwnsRaw for async_std::net::TcpStream {}
#[cfg(feature = "async-std")]
unsafe impl OwnsRaw for async_std::net::TcpListener {}
#[cfg(feature = "async-std")]
unsafe impl OwnsRaw for async_std::net::UdpSocket {}
#[cfg(all(feature = "async-std", unix))]
unsafe impl OwnsRaw for async_std::os::unix::net::UnixStream {}
#[cfg(all(feature = "async-std", unix))]
unsafe impl OwnsRaw for async_std::os::unix::net::UnixListener {}
#[cfg(all(feature = "async-std", unix))]
unsafe impl OwnsRaw for async_std::os::unix::net::UnixDatagram {}
