//! The [`RawHandleOrSocket`] type and accompanying [`AsRawHandleOrSocket`],
//! [`IntoRawHandleOrSocket`], and [`FromRawHandleOrSocket`] traits. These
//! provide minimal Windows analogs for the Posix-ish `RawFd` type and
//! accompanying `AsRawFd`, `IntoRawFd`, and `FromRawFd` traits.
//!
//! These types are only defined on Windows and do not requre implementors to
//! assert that they own their resources. See [`UnsafeHandle`] and
//! accompanying [`AsUnsafeHandle`], [`IntoRawHandle`], and
//! [`FromUnsafeHandle`] traits for platform-independent interfaces which do
//! include ownership assertions.
//!
//! [`UnsafeHandle`]: crate::UnsafeHandle
//! [`AsUnsafeHandle`]: crate::AsUnsafeHandle
//! [`IntoUnsafeHandle`]: crate::IntoUnsafeHandle
//! [`FromUnsafeHandle`]: crate::FromUnsafeHandle

#[cfg(feature = "os_pipe")]
use os_pipe::{PipeReader, PipeWriter};
use std::{
    fs::File,
    io::{Stderr, StderrLock, Stdin, StdinLock, Stdout, StdoutLock},
    net::{TcpListener, TcpStream, UdpSocket},
    os::windows::io::{
        AsRawHandle, AsRawSocket, IntoRawHandle, IntoRawSocket, RawHandle, RawSocket,
    },
    process::{ChildStderr, ChildStdin, ChildStdout},
};

pub use crate::read_write::AsRawReadWriteHandleOrSocket;

/// A Windows analog for the Posix-ish `AsRawFd` type. Unlike Posix-ish
/// platforms which have a single type for files and sockets, Windows has
/// distinct types, `RawHandle` and `RawSocket`; this type behaves like an
/// enum which can hold either.
///
/// It's reasonable to worry that this might be trying too hard to make Windows
/// work like Posix-ish platforms, however in this case, the number of types is
/// small, so the enum is simple and the overhead is relatively low, and the
/// benefit is that we can abstract over major [`Read`] and [`Write`]
/// resources.
///
/// [`Read`]: std::io::Read
/// [`Write`]: std::io::Write
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
#[repr(transparent)]
pub struct RawHandleOrSocket(pub(crate) RawEnum);

/// The enum itself is a private type so that we have the flexibility to change
/// the representation in the future.
///
/// It's possible that Windows could add other handle-like types in the future.
/// And it's possible that we'll want to optimize the representation, possibly
/// by finding an unused bit in the `RawHandle` and `RawSocket` representations
/// which we can repurpose as a discriminant.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
pub(crate) enum RawEnum {
    /// A `RawHandle`.
    Handle(RawHandle),

    /// A `RawSocket`.
    Socket(RawSocket),
}

impl RawHandleOrSocket {
    /// Like [`FromRawHandle::from_raw_handle`], but isn't unsafe because it
    /// doesn't imply a dereference.
    ///
    /// [`FromRawHandle::from_raw_handle`]: std::os::windows::io::FromRawHandle::from_raw_handle
    #[inline]
    #[must_use]
    pub const fn from_raw_handle(raw_handle: RawHandle) -> Self {
        Self(RawEnum::Handle(raw_handle))
    }

    /// Like [`FromRawSocket::from_raw_socket`], but isn't unsafe because it
    /// doesn't imply a dereference.
    ///
    /// [`FromRawSocket::from_raw_socket`]: std::os::windows::io::FromRawSocket::from_raw_socket
    #[inline]
    #[must_use]
    pub const fn from_raw_socket(raw_socket: RawSocket) -> Self {
        Self(RawEnum::Socket(raw_socket))
    }

    /// Like [`AsRawHandle::as_raw_handle`], but returns an `Option` so that
    /// it can return `None` if `self` doesn't contain a `RawHandle`.
    #[inline]
    #[must_use]
    pub const fn as_raw_handle(&self) -> Option<RawHandle> {
        match self.0 {
            RawEnum::Handle(raw_handle) => Some(raw_handle),
            RawEnum::Socket(_) => None,
        }
    }

    /// Like [`AsRawSocket::as_raw_socket`], but returns an `Option` so that
    /// it can return `None` if `self` doesn't contain a `RawSocket`.
    #[inline]
    #[must_use]
    pub const fn as_raw_socket(&self) -> Option<RawSocket> {
        match self.0 {
            RawEnum::Handle(_) => None,
            RawEnum::Socket(raw_socket) => Some(raw_socket),
        }
    }
}

/// Like [`AsRawHandle`] and [`AsRawSocket`], but implementable by types which
/// can implement either one.
pub trait AsRawHandleOrSocket {
    /// Like [`AsRawHandle::as_raw_handle`] and [`AsRawSocket::as_raw_socket`]
    /// but can return either type.
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket;
}

/// Like [`IntoRawHandle`] and [`IntoRawSocket`], but implementable by types
/// which can implement either one.
pub trait IntoRawHandleOrSocket {
    /// Like [`IntoRawHandle::into_raw_handle`] and
    /// [`IntoRawSocket::into_raw_socket`] but can return either type.
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket;
}

/// Like [`FromRawHandle`] and [`FromRawSocket`], but implementable by types
/// which can implement both.
///
/// Note: Don't implement this trait for types which can only implement one
/// or the other, such that it would need to panic if passed the wrong form.
///
/// [`FromRawHandle`]: std::os::windows::io::FromRawHandle
/// [`FromRawSocket`]: std::os::windows::io::FromRawSocket
pub trait FromRawHandleOrSocket {
    /// Like [`FromRawHandle::from_raw_handle`] and
    /// [`FromRawSocket::from_raw_socket`] but can be passed either type.
    ///
    /// # Safety
    ///
    /// `raw_handle_or_socket` must be valid and otherwise unowned.
    ///
    /// [`FromRawHandle::from_raw_handle`]: std::os::windows::io::FromRawHandle::from_raw_handle
    /// [`FromRawSocket::from_raw_socket`]: std::os::windows::io::FromRawSocket::from_raw_socket
    unsafe fn from_raw_handle_or_socket(raw_handle_or_socket: RawHandleOrSocket) -> Self;
}

/// The Windows [`HANDLE`] and [`SOCKET`] types may be sent between threads.
///
/// [`HANDLE`]: std::os::windows::raw::HANDLE
/// [`SOCKET`]: std::os::windows::raw::SOCKET
unsafe impl Send for RawHandleOrSocket {}

/// The Windows [`HANDLE`] and [`SOCKET`] types may be shared between threads.
///
/// [`HANDLE`]: std::os::windows::raw::HANDLE
/// [`SOCKET`]: std::os::windows::raw::SOCKET
unsafe impl Sync for RawHandleOrSocket {}

impl AsRawHandleOrSocket for RawHandleOrSocket {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> Self {
        *self
    }
}

impl AsRawHandleOrSocket for Stdin {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for StdinLock<'_> {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for Stdout {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for StdoutLock<'_> {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for Stderr {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for StderrLock<'_> {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for File {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for ChildStdin {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for ChildStdout {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for ChildStderr {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

impl AsRawHandleOrSocket for TcpStream {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_socket(Self::as_raw_socket(self))
    }
}

impl AsRawHandleOrSocket for TcpListener {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_socket(Self::as_raw_socket(self))
    }
}

impl AsRawHandleOrSocket for UdpSocket {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_socket(Self::as_raw_socket(self))
    }
}

#[cfg(feature = "os_pipe")]
impl AsRawHandleOrSocket for PipeReader {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

#[cfg(feature = "os_pipe")]
impl AsRawHandleOrSocket for PipeWriter {
    #[inline]
    fn as_raw_handle_or_socket(&self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::as_raw_handle(self))
    }
}

impl IntoRawHandleOrSocket for File {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::into_raw_handle(self))
    }
}

impl IntoRawHandleOrSocket for ChildStdin {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::into_raw_handle(self))
    }
}

impl IntoRawHandleOrSocket for ChildStdout {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::into_raw_handle(self))
    }
}

impl IntoRawHandleOrSocket for ChildStderr {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::into_raw_handle(self))
    }
}

impl IntoRawHandleOrSocket for TcpStream {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_socket(Self::into_raw_socket(self))
    }
}

impl IntoRawHandleOrSocket for TcpListener {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_socket(Self::into_raw_socket(self))
    }
}

impl IntoRawHandleOrSocket for UdpSocket {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_socket(Self::into_raw_socket(self))
    }
}

#[cfg(feature = "os_pipe")]
impl IntoRawHandleOrSocket for PipeReader {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::into_raw_handle(self))
    }
}

#[cfg(feature = "os_pipe")]
impl IntoRawHandleOrSocket for PipeWriter {
    #[inline]
    fn into_raw_handle_or_socket(self) -> RawHandleOrSocket {
        RawHandleOrSocket::from_raw_handle(Self::into_raw_handle(self))
    }
}
